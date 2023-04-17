# Graph Compression Exploration

I explored two alternatives for a simple graph compression: 

1. String Map: Just map the ID's to a list of the children.
2. ID Map: Store an order list of IDs, `ids`. Store a list of lists `children` where each index in the outer list represents the node we are describing, and each number in the inner list maps to a child id, which you could find in the `ids` list.

Below are visual outputs of `python3 generate_json.py 5 3`, where I'm requesting 5 nodes with a max of 3 edges per node:

## String map

_458 bytes_

```JSON
{
    "model.imported_package.0_my_finance.v0": [],
    "macro.our_package_name.1_project_idea.v1": [
        "source.our_package_name.2_my_ingested.v2",
        "group.our_package_name.4_my_revenue.v4"
    ],
    "source.our_package_name.2_my_ingested.v2": [
        "group.our_package_name.4_my_revenue.v4",
        "documentation.our_package_name.3_dbt_project.v3"
    ],
    "documentation.our_package_name.3_dbt_project.v3": [
        "group.our_package_name.4_my_revenue.v4"
    ],
    "group.our_package_name.4_my_revenue.v4": []
}
```

## ID map

_(275 bytes)_

```JSON
{
    "ids": [
        "model.imported_package.0_my_finance.v0",
        "macro.our_package_name.1_project_idea.v1",
        "source.our_package_name.2_my_ingested.v2",
        "documentation.our_package_name.3_dbt_project.v3",
        "group.our_package_name.4_my_revenue.v4"
    ], "children": [
        [],
        [2, 4],
        [4, 3],
        [4],
        []
    ]}
```

# Raw and compressed output sizes

The script I create creates a legal DAG, where each node has a random number of edges between [0, MAX_NUM_EDGES].

I then ran examples like `python3 generate_json.py 50000 150` (50,000 nodes, max 150 edges per node) to generate out of 50,000 nodes with 3,756,593 edges (~75 edges per node avg). I believe this number of edges skews much higher than we'd expect, though 50,000 nodes seems like a reasonably large project by today's standards. I then just converted this files into a `.zip` to get a sense of how well this
data compresses.

Here are some outputs:

| Number of Nodes | Average Num Edges | String Map (uncompressed) | String Map (compressed) | ID Map (uncompressed) | ID Map (compressed) |
| --------------: | ----------------: |-------------------------: | ----------------------: | --------------------: | ------------------: |
| 100             | 10                |   49 KB                   |     4 KB                |     9 KB              |     3 KB            |
| 2,000           | 1000              | 62.3 MB                   |   6.0 MB                |   7.7 MB              |   2.5 MB            |
| 10,000          | 100               |  47.2 MB                  |   4.9 MB                |   6.4 MB              |   2.2 MB            |
| 50,000          | 75                | 183.4 MB                  |  21.2 MB                |  28.7 MB              |   9.8 MB            |
| 100,000         | 15                |  77.4 MB                  |   9.4 MB                |  15.5 MB              |   4.6 MB            |
| 1,000,000       | 5                 | 297.0 MB                  |  39.1 MB                |  91.2 MB              |  21.5 MB            |
| 1,000,000       | 25                |  1.28 GB                  | 169.5 MB                | 250.9 MB              |  88.0 MB            |
| 10,000,000      | 5                 |  3.03 GB                  | 425.3 MB                | 972.3 MB              | 242.1 MB            |
| 10,000,000      | 15                |  8.05 GB                  |  1.16 GB                |  1.87 GB              | 640.8 MB            |

The last entry above represents 10,000,000 nodes and 149,986,094 edges, for a total of 159,986,094 entries. Even though ID's are 28-65 bytes (chars) long in my example, we're only using 11.86 bytes per entry in the uncompressed ID Map, and 4.000 bytes per entry in the compressed ID Map. In particular, each new node means a new Unique we have to store which takes up ~40 chars per ID, while each new edge takes up to log_10(num_nodes), which in the 10,000,000 case is a max of 8 chars per edge (plus some additional chars for for ", ").

As we can see above, avoiding the repetition of IDs throughout the document is very helpful for the uncompressed version, and even though the String Map versions actually _compress better_ due to all the repeated substrings.

I'll need to look at real-world data to compare, but anecdotally, nodes having an _average_ number of nodes of 150, 200, 2000 seem absurd to me, and the most likely way we're talking about handling 10,000,000 nodes is if we're allowing multi-project graph traversal. Note that as long as we approach the problem from a solution similar to the ID Map compression, we only need to store MBs of compressed data for massie projects (we don't need historical versions of the DAG unless we're going to allow historical traversal), and as we're managing the whole DAG in memory in WASM, we just need to ensure the WASM runtime has ~1GB of memory available to handle these MASSIVE DAGs, or at launch we can provide as little as 250MB and still get a ton of mileage.

A fun note from above, note that the String Map actually _does_ compress better. For (2000,000), 7.7MB -> 2.5MB (32.4% of the original) vs 62.3MB -> 6.0MB (9.6% of the original). But even though the duplicated strings throughout are highly compressible, in both the compressed and uncompressed for, the ID Map approach outperforms in terms of raw byte count.

Even though the ID map is a much smaller representation, we will still get a O(1) retrieval time for children (after we do a `O(n + m)` parse time into a `HashMap<UniqueId, Vec<UniqueId>>`), a parse we would have to do for either format.

# Frontend WASM

Mozilla claims [Note: A WebAssembly page has a constant size of 65,536 bytes, i.e., 64KiB and a maximum size of 100 pages (6.4MiB)](https://developer.mozilla.org/en-US/docs/WebAssembly/JavaScript_interface/Memory/Memory). This would mean we can't support graph traversal in the frontend (beyond small example DAGs). This didn't line up with my other findings.

A v8 blog claims you can have [up to 4GB of memory in WASM](https://v8.dev/blog/4gb-wasm-memory). V8 is used in all Chromium browsers (which is well over 60% of browser traffic, but does not include Safari or Firefox most notably). If we're able to use 4GB of memory in the majority of browsers consistently, we could offload the work of large graph traversal to the frontend, reducing the load on our endpoints, removing the expectation to load in a full graph/wasm binaries, and avoiding state management in the Discovery API.

You can use [this link](http://clb.confined.space/dump/mem_growth.html) to manually test the memory limits of the WASM runtime in different browsers. I tested this in Firefox, Safari, Chrome, and Arc and managed to allocation 4GB of heap space in each of the above browsers.

# Node WASM

Node also uses V8 which leaves us with 4GB of memory available.

## Ship a compressed JSON

The simplest path forward from here is to add a new ingestion step to codex which generates a JSON file like the above, and then compressed and publishes it somewhere. We likely only need the N most recent JSON blobs for a user (1 for discovery API, 2 if we want to allow diffing, or some small number like 5 if we want to allow users to have some recent historical comparisons)

A search would involve a step to pull down the JSON blob, process it in real time, adding to the request time. The best two optiosn for ptimization in this case would just be pre-downloading/caching the JSON, which would be hard to do for all possible users of the CA APIs, or we could bre-process the JSON blob

## Ship a compressed WASM binary that's already processed the JSON

If the ingestion step involved generating a JSON file like the above, and then building a WASM wrapper which has already processed the JSON file, then we would be adding extra work to every ingestion step, without every binary being used. This would also increase the size of the compressed file we would have to store, but I can't imagine substantially since the logic is mostly "Read in a JSON file, insert it into a HashMap, and then functionality to switch between children and parents modes.

## Store a JSON blob for everybody, expose an endpoint that allows users to generate a Parents and Children map binary/binaries

Always building compressed WASM libraries may be expensive to do on all runs, if we're expecting some small percentage of accounts to be using the Explorer compared to the number of runs. What we can do is on explorer startup call an endpoint which requests codex to pull down the most recently stored JSON file, and generate the parents and children selector(s) as one large or two smaller binaries.

These binaries are considered a pre-requisite of custom graph traversal selector logic, and if you request the DAG via a selector
