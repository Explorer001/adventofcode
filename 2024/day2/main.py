#!/usr/bin/env python

import argparse

def is_safe(report, min, max):
    diff = []
    for i in range(len(report) - 1):
        diff.append(report[i+1] - report[i])
        
    dist = [x for x in diff if abs(x) < min or abs(x) > max]
    pos = [x for x in diff if x > 0]
    neg = [x for x in diff if x < 0]

    return not dist and (pos == dist or neg == dist)

def is_safe_dampened(report, min, max):
    if is_safe(report, min, max):
        return True

    for i in range(len(report)):
        if is_safe(report[:i] + report[i+1:], min, max):
            return True

    return False

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('input', type=str, help="Puzzle input")
    args = parser.parse_args()

    with open(args.input) as f:
        lines = [list(map(int ,d.strip().split())) for d in f.readlines()]
        safe = [is_safe(l, 1, 3) for l in lines].count(True)
        safe_dampened = [is_safe_dampened(l, 1, 3) for l in lines].count(True)
        print(f"safe    ={safe}")
        print(f"dampened={safe_dampened}")

main()