import networkx as nx

# Create a graph of programs
graph = nx.Graph()

LINES = open("puzzleinput").read().splitlines()

for line in LINES:
    # Parse the line
    node, neighbors = line.split(' <-> ')

    # Add edges defined by this line
    graph.add_edges_from((node, neighbor) for neighbor in neighbors.split(', '))

print('Part 1:', len(nx.node_connected_component(graph, '0')))
