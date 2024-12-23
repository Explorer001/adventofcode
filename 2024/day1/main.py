#!/usr/bin/env python

import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('input', type=str, help="Puzzle input")
    args = parser.parse_args()

    with open(args.input) as f:
        lines = [d.strip() for d in f.readlines()]
        l1 = sorted([int(x.split()[0]) for x in lines])
        l2 = sorted([int(x.split()[1]) for x in lines])

        d = sum([abs(a - b) for a,b in list(zip(l1, l2))])
        s = sum([a * l2.count(a) for a in l1])
        print(f"dist={d}")
        print(f"sim ={s}")

main()
