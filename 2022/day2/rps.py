#!/usr/bin/env python3

import sys

def calc_score1(rounds):
    gesture_lookup = {"X": 1, "Y": 2, "Z": 3}
    rule_lookup = {
        "A": {"X": 3, "Y": 6, "Z": 0},
        "B": {"X": 0, "Y": 3, "Z": 6},
        "C": {"X": 6, "Y": 0, "Z": 3},
    }
    
    total = 0

    for round in rounds:
        opponent, own = round.split()
        match_val = rule_lookup[opponent][own]
        gesture_val = gesture_lookup[own]
        score = match_val + gesture_val
        total = total + score

    return total

def calc_score2(rounds):
    gesture_lookup = {"A": 1, "B": 2, "C": 3}
    point_lookup = {"X": 0, "Y": 3, "Z": 6}
    move_lookup = {
        "A": {"X": "C", "Y": "A", "Z": "B"},
        "B": {"X": "A", "Y": "B", "Z": "C"},
        "C": {"X": "B", "Y": "C", "Z": "A"},
    }
    
    total = 0

    for round in rounds:
        opponent, res = round.split()
        move = move_lookup[opponent][res]
        score = point_lookup[res] + gesture_lookup[move]
        total = total + score

    return total

def main():

    if len(sys.argv) < 2:
        return

    fin = open(sys.argv[1], "r")
    data = fin.read()
    fin.close()

    rounds = data.split('\n')
    print(f"score1={calc_score1(rounds)}")
    print(f"score2={calc_score2(rounds)}")

main()