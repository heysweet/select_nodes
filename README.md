# dbt Node Selector

This library aims to solve the Node Selection problem currently solved in dbt-core in Python, written in Rust, compiled down to WASM. Many files have code links to the original implementation this was based on. The Rust implementation aims to stay fairly close to the original Python logic, in order to best position this library to be used in a production setting, maintained by core developers.

The reason this library was written was to solve a problem: the upcoming dbt Explorer (nee Docs) project will have no ability to interact with dbt at launch. In order to achieve parity with the original generated docs, we would need to support the dbt Selector logic, meaning the dbt Explorer launch required, at a minimum, a TypeScript port of today's selector logic and the longterm expectation to keep it in sync with core's definition.

There are 3 launch targets for this library: Either in the dbt Explorer frontend logic or in the Cloud Artifacts API endpoint for retrieving the DAG, and aspirationally this logic could one day replace the logic living in dbt Core for selecting nodes.

## Library Structure

This library builds to WASM and uses [Wasmer](https://wasmerio.github.io/wasmer-pack/user-docs/index.html) to generate a python and JS wrapper libraries. The [dbt-node-selector.wai](src/dbt-node-selector.wai) file specifies an interface to be used between rust and the target languages.

* [The *.wai format](https://wasmerio.github.io/wasmer-pack/user-docs/concepts/wai/index.html)
* [The Wasmer Guide](https://wasmerio.github.io/wasmer-pack/user-docs/)
* [WebAssembly Package Manager (wapm)](https://wapm.io/)

The `*.wai` file generates the WAI Bindgen code, which is then implemented in this rust code as the Guest WASM library which will be used in a Python or JS Host library.

The core logic is broken down into Graph logic and Selector logic, where Graph logic encapsulates nodes, edges, and graph traversal, while the Selector logic encapsulates parsing a dbt selector string ([Node Selector Syntax](https://docs.getdbt.com/reference/node-selection/syntax)), and determining how to traverse a graph using the criteria determined from the node selector.

## Further Exploration

* [Example API Usage](ExampleAPIUsage.md)
* [Graph Compression Testing](graph_compression_testing/CompressionTesting.md)
* [Recommended Architecture](RecommendedArchitecture.md)
