#!/bin/python
import sys

#
# Retrieve seat id from encoding.
#
# @param src The encoded seat id.
# @param nrows The number of rows.
# @param ncols The number of cols.
# @return The seat id.
def get_seat_id(src, nrows, ncols):
    
    lrow = 0
    urow = nrows - 1
    lcol = 0
    ucol = ncols - 1
    
    for c in src:
        if c == 'F':
            urow = int((urow + lrow)/2)
        elif c == 'B':
            lrow = int((urow + lrow)/2) + 1
        elif c == 'L':
            ucol = int((ucol + lcol)/2)
        elif c == 'R':
            lcol = int((ucol + lcol)/2) + 1
    
    return lrow * 8 + lcol

def main():
    
    if len(sys.argv) < 2:
        return

    fin = open(sys.argv[1], "r")
    data = fin.read().splitlines()
    fin.close()

    mx = 0
    ids = []
    for seat in data:
        _id = get_seat_id(seat, 128, 8)
        ids.append(_id)
        if _id > mx:
            mx = _id
   
    ids = sorted(ids)

    for i in range(len(ids) - 1):
        if ids[i] != ids[i+1] - 1:
            print(ids[i] + 1)

    print(f"Highest seat ID: {mx}")
            

main()
