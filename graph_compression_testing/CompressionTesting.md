# Graph Compression Exploration

I explored two alternatives for a simple graph compression: 

1. String Map: Just map the ID's to a list of the children.
2. ID Map: Store an order list of IDs, `ids`. Store a list of lists `children` where each index in the outer list represents the node we are describing, and each number in the inner list maps to a child id, which you could find in the `ids` list.

Below are visual outputs of `python3 generate_json.py 5 3`, where I'm requesting 5 nodes with a max of 3 edges per node:

## String Map (338 bytes)

```JSON
{
    "imported_package::my_finance_model_0.sql": [
        "our_package_name::dbt_project_documentation_3.sql"
    ],
    "our_package_name::test_macro_1.sql": [], "our_package_name::my_ingested_source_2.sql": [
        "our_package_name::my_revenue_group_4.sql"
    ],
    "our_package_name::dbt_project_documentation_3.sql": [], "our_package_name::my_revenue_group_4.sql": []
}
```

## ID Map (270 bytes)

```JSON
{
    "ids": [
        "imported_package::my_finance_model_0.sql", "our_package_name::test_macro_1.sql", "our_package_name::my_ingested_source_2.sql", "our_package_name::dbt_project_documentation_3.sql", "our_package_name::my_revenue_group_4.sql"
        ],
    "children": [[3], [], [4], [], []]
}
```

# Raw and Compressed Output Sizes

The script I create creates a legal DAG, where each node has a random number of edges between [0, MAX_NUM_EDGES].

I then ran examples like `python3 generate_json.py 50000 150` (50,000 nodes, max 150 edges per node) to generate out of 50,000 nodes with 3,756,593 edges (~75 edges per node avg). I believe this number of edges skews much higher than we'd expect, though 50,000 nodes seems like a reasonably large project by today's standards. I then just converted this files into a `.zip` to get a sense of how well this
data compresses.

Here are some outputs:

| Number of Nodes | Average Num Edges | String Map (uncompressed) | String Map (compressed) | ID Map (uncompressed) | ID Map (compressed) |
| --------------: | ----------------: |------------------------: | ----------------------: | --------------------: | ------------------: |
| 100             | 10                |   49 KB                  |     4 KB                |     9 KB              |     3 KB            |
| 2,000           | 1000              | 62.3 MB                  |   6.0 MB                |   7.7 MB              |   2.5 MB            |
| 10,000          | 100               |  47.2 MB                 |   4.9 MB                |   6.4 MB              |   2.2 MB            |
| 50,000          | 75                | 183.4 MB                 |  21.2 MB                |  28.7 MB              |   9.8 MB            |
| 100,000         | 15                |  77.4 MB                 |   9.4 MB                |  15.5 MB              |   4.6 MB            |
| 1,000,000       | 5                 | 297.0 MB                 |  39.1 MB                |  91.2 MB              |  21.5 MB            |
| 1,000,000       | 25                |  1.28 GB                 | 169.5 MB                | 250.9 MB              |  88.0 MB            |
| 10,000,000      | 5                 |  3.03 GB                 | 425.3 MB                | 972.3 MB              | 242.1 MB            |

As we can see above, avoiding the repetition of IDs throughout the document is very helpful for the uncompressed version, and even though the String Map versions actually _compress better_ due to all the repeated substrings.

I'll need to look at real-world data to compare, but anecdotally, nodes having an _average_ number of nodes of 150, 200, 2000 seem absurd to me, and the most likely way we're talking about handling 10,000,000 nodes is if we're allowing multi-project graph traversal. Note that as long as we approach the problem from a solution similar to the ID Map compression, we only need to store MBs of compressed data for massie projects (we don't need historical versions of the DAG unless we're going to allow historical traversal), and as we're managing the whole DAG in memory in WASM, we just need to ensure the WASM runtime has ~1GB of memory available to handle these MASSIVE DAGs, or at launch we can provide as little as 250MB and still get a ton of mileage.

A fun note from above, note that the String Map actually _does_ compress better. For (2000,000), 7.7MB -> 2.5MB (32.4% of the original) vs 62.3MB -> 6.0MB (9.6% of the original). But even though the duplicated strings throughout are highly compressible, in both the compressed and uncompressed for, the ID Map approach outperforms in terms of raw byte count.

Even though the ID map is a much smaller representation, we will still get a O(1) retrieval time for children (after we do a `O(n + m)` parse time into a `HashMap<UniqueId, Vec<UniqueId>>`), a parse we would have to do for either format.