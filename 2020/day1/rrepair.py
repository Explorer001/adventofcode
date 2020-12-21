#!/bin/python
import sys

#
# Finds pair in data that sums to _sum.
#
# @param data The input data (int array).
# @param _sum The required sum.
# @return Product of the summands or -1.
def sum_pair(data, _sum):
    for x in data:
        for y in data:
            if x + y == _sum:
                return x * y
    return -1

#
# Finds triple of data that sums to _sum.
#
# @param data The input data (int array).
# @param _sum The required sum.
# @param Product of the summands or -1.
def sum_trips(data, _sum):
    for x in data:
        for y in data:
            for z in data:
                if x + y + z == _sum:
                    return x * y * z
    return -1

def main():

    if len(sys.argv) < 2:
        return

    fin = open(sys.argv[1], "r")
    data = fin.read()
    fin.close()

    ints = [int(i) for i in data.split()]
    print("Multiply pair: " + str(sum_pair(ints, 2020)))
    print("Multiply trip: " + str(sum_trips(ints, 2020)))

main()
