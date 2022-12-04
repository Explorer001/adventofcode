#!/usr/bin/env python3

import sys
import getopt

class SectionIds:
    def __init__(self, section_string):
        self.ranges = []
        
        ranges = section_string.split(",")
        for range in ranges:
            start, end = range.split("-")
            self.ranges.append((int(start), int(end)))

    def is_self_contained(self):
        for i in range(len(self.ranges)):
            for j in range(i + 1, len(self.ranges)):
                r0 = self.ranges[i]
                r1 = self.ranges[j]
                if r0[0] <= r1[0] and r0[1] >= r1[1]:
                    return True
                elif r1[0] <= r0[0] and r1[1] >= r0[1]:
                    return True
        return False

    def overlapping(self):
        for i in range(len(self.ranges)):
            for j in range(i + 1, len(self.ranges)):
                r0 = self.ranges[i]
                r1 = self.ranges[j]
                if r0[1] >= r1[0] and r0[0] <= r1[0]:
                    return True
                elif r1[1] >= r0[0] and r1[0] <= r0[0]:
                    return True
        return False


def count_containments(section_ids):
    containments = 0
    overlaps = 0
    for id in section_ids:
        if id.is_self_contained():
            containments += 1
        if id.overlapping():
            overlaps += 1
    print(f"Containments={containments}")
    print(f"Overlaps={overlaps}")

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

    # Do something with your input
    ids = [SectionIds(i) for i in data.splitlines()]
    count_containments(ids)

main()