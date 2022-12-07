#!/usr/bin/env python3

import sys
import getopt

class FsNode:
    def __init__(self, name, size, type):
        self.name = name
        self.size = size
        self.type = type
        self.children = []
        self.parent = None

    def add_child(self, child):
        self.children.append(child)

    def set_parent(self, parent):
        self.parent = parent

    def __str__(self):
        return f"{self.name} ({self.type}, size={self.size})"

    def __repr__(self):
        return f"{self.name} ({self.type}, size={self.size})"

class Fs:
    def __init__(self, cmds):
        self.root = FsNode("/", 0, "dir")

        current_node = None
        for cmd in cmds:
            tokens = cmd.split()
            if tokens[0] == "$":
                if tokens[1] == "cd":
                    if tokens[2] == "/":
                        current_node = self.root
                    elif tokens[2] == "..":
                        current_node = current_node.parent
                    else:
                        node = FsNode(tokens[2], 0, "dir")
                        node.set_parent(current_node)
                        current_node.add_child(node)
                        current_node = node
                elif tokens[1] == "ls":
                    continue
            else:
                if tokens[0] == "dir":
                    continue
                else:
                    node = FsNode(tokens[1], int(tokens[0]), "file")
                    node.set_parent(current_node)
                    current_node.add_child(node)

        self.update_dir_size(self.root)

    def update_dir_size(self, node):
        if node.type == "dir":
            node.size = 0

        size = node.size

        for n in node.children:
            size += self.update_dir_size(n)

        if node.type == "dir":
            node.size = size

        return size

    def get_dirs_recursive(self, node):
        dirs = []
        if node.type == "dir":
            dirs.append(node)
        
        for n in node.children:
            d = self.get_dirs_recursive(n)
            for element in d:
                dirs.append(element)

        return dirs

    def get_dirs(self):
        return self.get_dirs_recursive(self.root)

    def get_string(self, node, depth):
        s = "\t" * depth + "- " + str(node) + "\n"

        for n in node.children:
            s += self.get_string(n, depth + 1)
        return s


    def __str__(self):
        return self.get_string(self.root, 0)
            

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
    cmds = data.split("\n")
    file_system = Fs(cmds)

    sum = 0
    dirs = file_system.get_dirs()
    for dir in dirs:
        if dir.size <= 100000:
            sum += dir.size
    print(f"d_sum={sum}")
    
    needed_size = 30000000
    disk_space = 70000000
    free_space = disk_space - dirs[0].size
    deletes = []
    for dir in dirs:
        if free_space + dir.size >= needed_size:
            deletes.append(dir.size)
    print(f"del_dir={min(deletes)}")

main()