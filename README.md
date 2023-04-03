# dbt Node Selector

This library aims to solve the Node Selection problem currently solved in dbt-core in Python, written in Rust, compiled down to WASM. Many files have code links to the original implementation this was based on. The Rust implementation aims to stay fairly close to the original Python logic, in order to best position this library to be used in a production setting, maintained by core developers.

The reason this library was written was to solve a problem: the upcoming dbt Explorer (nee Docs) project will have no ability to interact with dbt at launch. In order to achieve parity with the original generated docs, we would need to support the dbt Selector logic, meaning the dbt Explorer launch required, at a minimum, a TypeScript port of today's selector logic and the longterm expectation to keep it in sync with core's definition.

There are 3 launch targets for this library: Either in the dbt Explorer frontend logic or in the Cloud Artifacts API endpoint for retrieving the DAG, and aspirationally this logic could one day replace the logic living in dbt Core for selecting nodes.