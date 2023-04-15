# Recommended API Design to Avoid Having the Full DAG in Memory

This file was written as a pitch on a way to solve the problem of not having
all nodes in memory. The general design is to store a highly compressable
DAG representation which is just a mapping of node_ids to a list of children
node IDs.

This API was also designed to enable parallel work, and is solvable in 4 batches
of promises

1. Get SelectionSpecs from input string (WASM) while fetching compressed graph (database/S3?)
2. Use the SelectionSpecs to query for all target nodes (database) while instantiating instantiate NodeSelector with compressed graph (WASM)
3. Get selection/exclusion set for each selection spec by traversing the graph (WASM)

After all batches are complete, we can just do unions and differences on the outputs
and be done!

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
    const compressedGraphPromise = retrieveCompressedGraph();

    // Converted into instructions on what base nodes to retrieve, in this case
    // 1. my_model
    // 2. all nodes tagged as nightly INTERSECT all all configs materialized as table
    // 3. all nodes in marts/finance directory
    
    // Difference(Union(), AllItemsInMarts/Finance)
    // Resolve Batch 1
    const [selectSpecs, excludeSpecs] = await Promise.all([
        SelectorSpec(selector),
        SelectorSpec(exclude)
    ]);

    // Start Batch 2
    const nodeSelectorPromise = NodeSelector(await compressedGraphPromise);
    
    const selectedNodesPromises = Promise.all(selectSpecs.map(async (selectSpec) => {
        return queryForSpec(selectSpec).then(async (selectedNodeIds) => {
            const nodeSelector = await nodeSelectorPromise;
            nodeSelector.expandSet(selectSpec, selectedNodeIds);
        });
    })).then((nodeSelectorSets) => {
        return unionSet(nodeSelectorSets);
    });
    const excludedNodesPromises = Promise.all(selectSpecs.map(async (excludeSpec) => {
        return queryForSpec(excludeSpec).then(async (excludedNodeIds) => {
            const nodeSelector = await nodeSelectorPromise;
            nodeSelector.expandSet(excludeSpec, excludedNodeIds);
        });
    })).then((nodeSelectorSets) => {
        return unionSet(nodeSelectorSets);
    });

    // Resolve Batch 3
    const [selectedSet, excludedSet] = await Promise.all([
        selectedNodesPromises,
        excludedNodesPromises
    ]);
    
    return differenceSet([selectedSet, excludedSet]);
}
```