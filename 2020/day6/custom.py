#!/bin/python
import sys

#
# Counts number of questions answered with yes.
#
# @param grp The groups answers
# @return Count of commons in group.
def count_any_yes(grp):
    return len(set(grp.replace("\n", "")))

#
# Counts number of questions everyone answered with yes.
#
# @param grp The groups answers.
# @return Count of commons.
def count_every_yes(grp):
    count = 0
    nparts = len(grp.splitlines())
    answers = grp.replace("\n", "")
    charset = set(answers)

    for char in charset:
        if answers.count(char) == nparts:
            count += 1

    return count


def main():

    if len(sys.argv) < 2:
        return

    fin = open(sys.argv[1], "r")
    data = fin.read().split("\n\n")
    fin.close()

    count1 = 0
    count2 = 0
    for grp in data:
        count1 += count_any_yes(grp)
        count2 += count_every_yes(grp)

    print(f"Yes count any: {count1}")
    print(f"Yes count every: {count2}")

main()
