#!/usr/bin/env python3

import sys

def get_top_n_calories(blocks, n):
    cal = sorted([sum(b) for b in blocks], reverse=True)
    return sum(cal[:n])


def main():

    if len(sys.argv) < 2:
        return

    fin = open(sys.argv[1], "r")
    data = fin.read()
    fin.close()

    blocks = [[int(j) for j in i.split()] for i in data.split('\n\n')]

    print(f"cal_max={get_top_n_calories(blocks, 1)}")
    print(f"cal3_max={get_top_n_calories(blocks, 3)}")

main()