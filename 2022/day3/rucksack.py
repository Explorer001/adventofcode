#!/usr/bin/env python3

import sys
import getopt

def get_priority(c):
    if c.isupper():
        return ord(c) - 38
    else:
        return ord(c) - 96

def get_commons(s):
    strlen = len(s)
    commons = ""

    if strlen % 2 != 0:
        print(f"Malformed input: {s}")
        return ""

    # Unify the lists
    s1 = sorted(set(s[:strlen//2]))
    s2 = sorted(set(s[strlen//2:]))

    for c1 in s1:
        for c2 in s2:
            if c1 == c2:
                commons += c1

    return commons

def calc_rucksack_sum(rucksacks):
    total = 0
    for rucksack in rucksacks:
        commons = get_commons(rucksack)
        debug_print(f"commons=\"{commons}\"")
        for common in commons:
            prio = get_priority(common)
            debug_print(f"item={common},score={prio}")
            total += prio
    print(f"prio_sum={total}")

def get_group_commons(rucksacks, n):
    lowers = [0] * 26
    uppers = [0] * 26

    for rucksack in rucksacks:
        for item in sorted(set(rucksack)):
            if item.isupper():
                uppers[ord(item) - 65] += 1
            else:
                lowers[ord(item) - 97] += 1

    debug_print(f"lowers={lowers}")
    debug_print(f"uppers={uppers}")
    
    for i in range(len(lowers)):
        if lowers[i] == n:
            return chr(i + 97)
        elif uppers[i] == n:
            return chr(i + 65)
        

def calc_group_sum(rucksacks, groupsize):
    total = 0
    for i in range(0, len(rucksacks), groupsize):
        common = get_group_commons(rucksacks[i:i+groupsize], groupsize)
        debug_print(f"common={common}")
        total += get_priority(common)
    print(f"group_sum={total}")

verbose = False
def debug_print(s):
    if verbose:
        print(s)

def print_usage(name):
    print(f"usage: {name} -f <file> [-v][-h]")
    print("\t-f,--file\tPuzzle input textfile")
    print("\t-v,--v\tEnable verbose prints")
    print("\t-h,--help\tPrint this help")

def main():
    global verbose
    puzzle_file = None
    try:
        opts, _ = getopt.getopt(sys.argv[1:], "f:vh", ["file=", "verbose", "help"])
    except getopt.GetoptError as err:
        print(err)
        print_usage(sys.argv[0])
        sys.exit(2)

    for opt, arg in opts:
        if opt in ("-h", "--help"):
            print_usage(sys.argv[0])
        elif opt in ("-f", "--file"):
            puzzle_file = arg
        elif opt in ("-v", "--verbose"):
            verbose = True

    if puzzle_file == None:
        print("No input file given!")
        sys.exit(2)
    
    try:
        fin = open(puzzle_file, "r")
        data = fin.read()
        fin.close()
    except OSError:
        print(f"Could not open file: {puzzle_file}")
        sys.exit(2)

    lines = data.split()
    calc_rucksack_sum(lines)
    calc_group_sum(lines, 3)

main()