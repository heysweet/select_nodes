import json
import random
import math
import sys

def id(index) -> str:
    unique_id = ""

    if index % 13 == 0:
        unique_id += "imported_package::"
    else:
        unique_id += "our_package_name::"

    if index % 2 == 0:
        unique_id += "my_"

    if index % 5 == 0:
        unique_id += "finance_"
    elif index % 5 == 1:
        unique_id += "test_"
    elif index % 5 == 2:
        unique_id += "ingested_"
    elif index % 5 == 3:
        unique_id += "dbt_project_"
    else:
        unique_id += "revenue_"

    if index % 7 == 0:
        unique_id += "model"
    elif index % 7 == 1:
        unique_id += "macro"
    elif index % 7 == 2:
        unique_id += "source"
    elif index % 7 == 3:
        unique_id += "documentation"
    elif index % 7 == 4:
        unique_id += "group"
    elif index % 7 == 5:
        unique_id += "node"
    else:
        unique_id += "metric"

    return unique_id + "_" + str(index) + ".sql"

def make_json_files(num_nodes, max_num_edges):
    max_num_edges += 1
    num_edges = 0
    dict_with_strings = {}
    dict_with_indices = {
        "ids": [],
        "children": []
    }

    for node_index in range(num_nodes):
        node_id = id(node_index)
        min_child_id = node_index + 1
        target_child_count = math.floor(random.random() * max_num_edges)

        num_new_edges = min(target_child_count, num_nodes - min_child_id)
        num_edges += num_new_edges

        new_edges = random.sample(range(node_index+1, num_nodes), num_new_edges)

        dict_with_indices["ids"].append(node_id)
        dict_with_indices["children"].append(new_edges)

        dict_with_strings[node_id] = [id(edge_index) for edge_index in new_edges]

    int_id_json = json.dumps(dict_with_indices)
    string_id_json = json.dumps(dict_with_strings)

    with open("{0}_nodes_{1}_edges_string.json".format(num_nodes, num_edges), "w") as outfile:
        outfile.write(string_id_json)

    with open("{0}_nodes_{1}_edges_int.json".format(num_nodes, num_edges), "w") as outfile:
        outfile.write(int_id_json)


def main():
    assert "hello" == 0
    args = sys.argv[1:]
    num_nodes = int(args[0])
    max_num_edges = int(args[1])

    make_json_files(num_nodes, max_num_edges)

if __name__ == "__main__":
    main()