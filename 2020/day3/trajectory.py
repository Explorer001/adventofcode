#!/bin/python
import sys

#
# Counts number of trees in path.
#
# @param treemap The biome map.
# @param dr Path increment to the right.
# @param dd Path increment down.
# @return Number of collisions.
def count_collisions(treemap, dr, dd):
    collisions = 0
    px = 0
    mx = len(treemap[0])

    for py in range(0, len(treemap), dd):
        line = treemap[py]

        if line[px] == "#":
            collisions += 1

        px = (px + dr) % mx

    return collisions

def main():
    if len(sys.argv) < 2:
        return

    fin = open(sys.argv[1], "r")
    data = fin.read()
    fin.close()

    lines = data.splitlines()
    colls = count_collisions(lines, 3, 1)
    print("Collisions: " + str(colls))

    deltas = [(1,1), (3,1), (5,1), (7,1), (1,2)]
    collisions = 1

    for dr,dd in deltas:
        collisions *= count_collisions(lines, dr, dd)

    print("Collisions: " + str(collisions))

main()
