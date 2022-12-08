#!/usr/bin/env python3

import sys
import getopt

class Treetop:
    OKBLUE = '\033[94m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'

    def __init__(self, data):
        self.trees = [list(map(int, [*e])) for e in data.split("\n")]
        self.x_lim = len(self.trees[0])
        self.y_lim = len(self.trees)

    def is_visible(self, x, y):
        if x <= 0 or x >= self.x_lim - 1 or y <= 0 or y >= self.y_lim - 1:
            return True

        val = self.trees[y][x]
        x_array = self.trees[y]
        y_array = [e[x] for e in self.trees]

        x_vis = val > max(x_array[:x]) or val > max(x_array[x+1:])
        y_vis = val > max(y_array[:y]) or val > max(y_array[y+1:])
        return x_vis or y_vis

    def count_visible(self):
        count = 0
        for i in range(self.x_lim):
            for j in range(self.y_lim):
                if self.is_visible(i, j):
                    count += 1
        return count

    def scenic_score(self, x, y):
        val = self.trees[y][x]
        score_xp = 0
        for i in range(x+1, self.x_lim):
            score_xp += 1
            if val <= self.trees[y][i]:
                break

        score_xn = 0
        for i in range(x-1, -1, -1):
            score_xn += 1
            if val <= self.trees[y][i]:
                break

        score_yp = 0
        for i in range(y+1, self.y_lim):
            score_yp += 1
            if val <= self.trees[i][x]:
                break

        score_yn = 0
        for i in range(y-1, -1, -1):
            score_yn += 1
            if val <= self.trees[i][x]:
                break

        return score_xp * score_xn * score_yp * score_yn

    def get_best_score(self):
        mx = 0
        for i in range(self.x_lim):
            for j in range(self.y_lim):
                sc = self.scenic_score(i, j)
                if  sc > mx:
                    mx = sc
        return mx

    def __repr__(self):
        s = ""
        for i in range(self.y_lim):
            l = ""
            for j in range(self.x_lim):
                if self.is_visible(j, i):
                    l += self.FAIL + str(self.trees[i][j])
                else:
                    l += self.OKBLUE + str(self.trees[i][j])
            l += self.ENDC + "\n"
            s += l
        return s
            

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
    treetop = Treetop(data)
    print(treetop)
    print(f"visible={treetop.count_visible()}")
    print(f"best_score={treetop.get_best_score()}")

main()