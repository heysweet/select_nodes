# Recommended architecture

After [exploring compression](graph_compression_testing/CompressionTesting.md), and prodding what a [fully-backend implementation API could look like](ExampleAPIUsage.md), I think I've come across the path I want to advocate for:

## GraphQL Endpoints

Where `selector` is a user inputted string following the [selector syntax](https://docs.getdbt.com/reference/node-selection/syntax)

`UniqueId` is just a string

and `SelectorSpec` is an object that looks like:

```Typescript
{
    target_ids: UniqueId[],
    childrens_parents: bool,
    parents: bool,
    parents_depth?: int,
    children: bool,
    children_depth?: int,
    indirect_selection: 'Eager' | 'Cautious' | 'Buildable' | 'Empty',
}
```

```Typescript
function getDagTraversalSpec(environmentId: int, selector: string): TraversalSpec[] {}
```

To achieve the above, the Discovery API will only need a small amount of wasm that takes in a raw string selector, and returns a list of `SelectionCriteria`:

```Rust
pub enum MethodName {
    /// Yield all nodes in the graph that match the selector.
    FQN,
    /// Yields nodes from included that have the specified tag.
    Tag,
    /// Yields nodes from included in the specified group.
    Group,
    /// Yields nodes from included are the specified source.
    Source,
    /// Yields nodes from included that match the given path.
    Path,
    /// Yields nodes from included that match the given file name.
    File,
    /// Yields nodes from included that have the specified package.
    Package,
    Config,
    TestName,
    TestType,
    ResourceType,
    State,
    Exposure,
    Metric,
    RunResult,
    SourceStatus,
    Wildcard,
}

pub struct SelectionCriteria {
    pub raw: String,
    pub method: MethodName,
    pub method_arguments: Vec<String>,
    pub value: String,
    pub childrens_parents: bool,
    pub parents: bool,
    pub parents_depth: Option<usize>,
    pub children: bool,
    pub children_depth: Option<usize>,
    // TODO: Default to Eager
    pub indirect_selection: IndirectSelection,
}
```

for each SelectionCriteria, you can use the `method`, `method_arguments`, and `value` to implement a query for some data. For instance, if the method was `'Tag'`, then you could write a simple query to go find all elements that match the tag with the value `'nightly'`.

Once you've retrieved all of the matching nodes for each selection criteria (or we could paginate this endpoint by making each selection criteria its own page), then this endpoint generates a `TraversalSpec` for each `SelectionCriteria`. This would simply copy over most of the data from the `SelectionCriteria`, and then append a list of `UniqueId`s from the matching result queries.


```Typescript
type TraversalSpec = {
    target_ids: UniqueId[],
    childrens_parents: bool,
    parents: bool,
    parents_depth?: int,
    children: bool,
    children_depth?: int,
    indirect_selection: 'Eager' | 'Cautious' | 'Buildable' | 'Empty',
}
```

## Frontend Logic

When we load up the dbt Explorer, we load the most recent [`CompressedDAG.json`](graph_compression_testing/CompressionTesting.md#id%20map), and load the WASM library for Graph Traversal. Even for very large projects, an uncompressed version of the DAG will easily fit in less than 4GB of memory ([the limit for WASM in the browser, today](graph_compression_testing/CompressionTesting.md#Frontend%20WASM)).

A cool benefit of having the full DAG in the frontend is we should be able to render the dag or a small subset of the DAG without any network requests, which should make navigating between pages and displaying default-y "2+my_node+2" or even "+my_node+" DAGs a trivial exercise.

If we want to allow custom selectors, a user will type in an arbitrary selector and send it over the wire to the Discovery API:

```Typescript
getDagTraversalSpec(environmentId: 1, selector: "2+my_model+3 tags:nightly,config.materialized:table"): TraversalSpec[] {}
```

The above would match to `my_model` in one traversal spec, and then match all nodes tagged `nightly` intersected with all materialized tables. We'd receive these two traversal specs:

```Typescript
[
    {
        target_ids: ["my_model"],
        childrens_parents: false,
        parents: true,
        parents_depth: 2,
        children: true,
        children_depth: 3,
        indirect_selection: 'Eager',
    },
    {
        target_ids: ["my_nightly_model", "my_other_model", "my_model"],
        childrens_parents: false,
        parents: false,
        children: false,
        indirect_selection: 'Eager',
    }
]
```

We can then pass this data into the Traversal WASM library, which will:
* get 3 tiers of children and 2 tiers of parents away from `my_model` (including `my_model`)
* return the `target_id`s for the other traversal spec since there are no traversal instructions
* union the two sets of results

And now we have a list of unique IDs of all the nodes the user cares about.

A really fortunate fact for us is the unique ID's come in the following form:

`<resource_type>.<project_name>.<name>.v<v>`

This means, without any data other than the Unique ID, we should have enough data to:
* Determine the resource type, change color, icons
* Filter by `project_name` if need be (only showing the current project vs dependencies)
* Have some display name (not the `display_name` so this may not be 100% accurate)
* Develop a UI for showing multiple versions of nodes

### Future edgecases to explore

I wanted to note that while there are edgecases we have not discussed in this doc, we can see a fairly robust end-to-end implementation with 2 WASM binaries and no state needed in the backend beyond the databases!

The main edgecases to think about are any additional data we will need in the DAG that is not embedded in the `unique_id` itself. This would include things like:
* Private v Public (I think this should be handled at Graph Compression time)
* docs configuration (custom node colors, show/hide nodes)
* `display_name`s which vary from file names

Two options for this data are either:
* Expand the size of the compressed DAG to make this hyper localized solution work
* Add another new endpoint for getting the data we want in a DAG from a list of `unique_id`s

### Supporting Exclusion

As a tailend exercise, if we want to support both `select` and `exclude`, you just run the same logic twice, and then perform a Set Difference on the selected and excluded set. This is not something we support in today's docs, but is a trivial lift once the rest of this is implemented.