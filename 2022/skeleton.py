#!/usr/bin/env python3

import sys
import getopt

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

main()