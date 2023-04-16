# Recommended API Design to Avoid Having the Full DAG in Memory

This file was written as a pitch on a way to solve the problem of not having
all nodes in memory. The general design is to store a highly compressable
DAG representation which is just a mapping of node_ids to a list of children
node IDs.

This API was also designed to enable parallel work, and is solvable in a sequence of 4 batches
of promises:

1. Get SelectionSpecs from input string (WASM) while fetching compressed graph (database/S3?)
2. The second set of batches is comprised of 2 phases for each Selection Spec:

   a. Use the SelectionSpecs to query for all target nodes (database) while instantiating a NodeSelector with compressed graph (WASM)
   
   b. Get selection/exclusion children set for each selection spec by traversing the graph (WASM)
3. Invert the graph representation to traverse parents efficiently (WASM)
4. Get selection/exclusion parents (or parents of children if the spec asks for it with "@") set for each selection spec by traversing the graph (WASM)

We wait for the fourth batch of requests to resolve, and then we can just do unions and differences on the outputs and be done!

The compressed graph representation could be something like the following:

```json
{
    "ids": ["model_a", "model_b", "model_c"],
    "children": {
        "model_a": [1, 2],
        "model_b": [2],
        "model_c": []
    }
}
```

This would allow for a relative optimal compression, O(1) lookup of children, and a small
memory footprint when uncompressed. This option would be used if we need to take the full compressed graph as one input string and manage it all in memory.

This would be in comparison to:

```json
{
    "model_a": ["model_b", "model_c"],
    "model_b": ["model_c"],
    "model_c": []
}
```

which would like compress better, and still support O(1) lookup of children, but take up a much larger memory footprint if we loaded the full thing into memory at once, uncompressed. This solution would work better if the WASM logic was fed in complete lines of the JSON file batch-by-batch, as we wouldn't have to worry about managing the full uncompressed version, and would have less state to worry about (mapping ID's to indices) while parsing the data.

Any compressed graph will just be read in and then converted into a `HashMap<UniqueId, Vec<UniqueId>`.

A problem with just passing in the compressed raw string.

```Typescript
// We do INTERSECT up front, and do UNION and DIFFERENCE at the end

// Fake Code to demonstrate an API that may work

import { NodeSelector, SelectorSpec } from '@dbt-labs/node-selector';

type UniqueId = string;

/** Given a list of sets, get the union of all sets */
function unionSet(sets: Set<UniqueId>[]) : Set<UniqueId> { return new Set(); }

/** Given a list of sets, take the first set and subtract elements from all other sets */ 
function differenceSet(sets: Set<UniqueId>[]) : Set<UniqueId> { return new Set(); }

// Intersection is just done as a SQL query

/**
 * https://www.cs.cmu.edu/afs/cs/project/pscico-guyb/realworld/www/slidesS18/compression6.pdf
 * 
 * The only information we need is a unique ID, and a list of children
 * for each node (to represent the DAG).
 * 
 * In the library, we will select children first, then invert this graph,
 * and then select children. This is due to the "@" operator being 
 * "Select all children and then all parents", and there being no other
 * reason to select parents before children
 * 
 * By only storing the children, and converting, we also save on half the
 * memory usage without much effort!
 * 
 * An example compression can just be a map of { uniqueId: [uniqueId] },
 * or some variation where we store all the unique IDs in a list, and then
 * just store a matrix of booleans or something.
 * 
 * The only operation we need from this representation is "getChildren(uniqueId)".
 * We then invert the graph, and run "getParents(uniqueId)", which saves on space
 * and shouldn't be that expensive of an operation
 */
async function retrieveCompressedGraph(): Promise<string> { return '' }

async function queryForSpec(spec: SelectorSpec){}

const selector = "2+my_model+3 tags:nightly,config.materialized:table";
const exclude = "path:marts/finance"

async function getSubgraph(selector, exclude) {
    // Start Batch 1
    const compressedGraphPromise = retrieveCompressedGraph();

    // Converted into instructions on what base nodes to retrieve, in this case
    // 1. my_model
    // 2. all nodes tagged as nightly INTERSECT all all configs materialized as table
    // 3. all nodes in marts/finance directory
    
    // Difference(Union(), AllItemsInMarts/Finance)
    const [selectSpecs, excludeSpecs] = await Promise.all([
        SelectorSpec(selector),
        SelectorSpec(exclude)
    ]);

    // Resolve Batch 1
    // Start Batch 2
    const nodeSelectorPromise = NodeSelector(await compressedGraphPromise);
    
    const selectedNodesChildrenPromises = Promise.all(selectSpecs.map(async (selectSpec) => {
        return queryForSpec(selectSpec).then(async (selectedNodeIds) => {
            const nodeSelector = await nodeSelectorPromise;
            return nodeSelector.selectChildren(selectSpec, selectedNodeIds);
        });
    })).then((nodeSelectorSets) => {
        return unionSet(nodeSelectorSets);
    });
    const excludedNodesChildrenPromises = Promise.all(selectSpecs.map(async (excludeSpec) => {
        return queryForSpec(excludeSpec).then(async (excludedNodeIds) => {
            const nodeSelector = await nodeSelectorPromise;
            nodeSelector.selectChildren(excludeSpec, excludedNodeIds);
        });
    })).then((nodeSelectorSets) => {
        return unionSet(nodeSelectorSets);
    });

    // Resolve Batch 2
    const [selectedSet, excludedSet] = await Promise.all([
        selectedNodesChildrenPromises,
        excludedNodesChildrenPromises
    ]);
    
    const selectedNodesChildrenPromises = Promise.all(selectSpecs.map(async (selectSpec) => {
        return queryForSpec(selectSpec).then(async (selectedNodeIds) => {
            const nodeSelector = await nodeSelectorPromise;
            return nodeSelector.selectChildren(selectSpec, selectedNodeIds);
        });
    })).then((nodeSelectorSets) => {
        return unionSet(nodeSelectorSets);
    });
    const excludedNodesChildrenPromises = Promise.all(selectSpecs.map(async (excludeSpec) => {
        return queryForSpec(excludeSpec).then(async (excludedNodeIds) => {
            const nodeSelector = await nodeSelectorPromise;
            nodeSelector.selectChildren(excludeSpec, excludedNodeIds);
        });
    })).then((nodeSelectorSets) => {
        return unionSet(nodeSelectorSets);
    });
    
    return differenceSet([selectedSet, excludedSet]);
}
```

# Notes

If instead we were planning on doing graph traversal on the frontend, we would want to embed the job ID in the binary so that we can discover a mismatch and re-request the WASM binary/compressed DAG.