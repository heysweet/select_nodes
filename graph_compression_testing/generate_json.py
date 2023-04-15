import json
import random
import math
import sys

def unique_id(resource_type: str, project_name: str, name: str, version: str):
    return "{resource_type}.{project_name}.{name}.v{version}".format(
        resource_type=resource_type,
        project_name=project_name,
        name=name,
        version=version
    )

other_packages = [
    "imported_package",
    "dbt_utils",
    "longer_project_name",
    "FOREIGN_IMPORTED_NAME",
    "their_package_name"
]

def id(index) -> str:
    resource_type = ""
    project_name = ""
    name = str(index)
    version = ""

    if index % 13 == 0:
        project_name = other_packages[index % len(other_packages)] ["imported_package"]
    else:
        project_name = "our_package_name"

    if index % 2 == 0:
        name += "_my"

    if index % 5 == 0:
        name += "_finance"
    elif index % 5 == 1:
        name += "_project_idea"
    elif index % 5 == 2:
        name += "_ingested"
    elif index % 5 == 3:
        name += "_dbt_project"
    else:
        name += "_revenue"

    if index % 7 == 0:
        resource_type = "model"
    elif index % 7 == 1:
        resource_type = "macro"
    elif index % 7 == 2:
        resource_type = "source"
    elif index % 7 == 3:
        resource_type = "documentation"
    elif index % 7 == 4:
        resource_type = "group"
    elif index % 7 == 5:
        resource_type = "node"
    else:
        resource_type = "metric"

    version = str(index % 17)

    return unique_id(resource_type, project_name, name, version)

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

    with open("{0}_nodes_{1}_max_edges_{2}_total_edges_string.json".format(num_nodes, max_num_edges, num_edges), "w") as outfile:
        outfile.write(string_id_json)

    with open("{0}_nodes_{1}_max_edges_{2}_total_edges_int.json".format(num_nodes, max_num_edges, num_edges), "w") as outfile:
        outfile.write(int_id_json)


def main():
    assert "hello" == 0
    args = sys.argv[1:]
    num_nodes = int(args[0])
    max_num_edges = int(args[1])

    make_json_files(num_nodes, max_num_edges)

if __name__ == "__main__":
    main()