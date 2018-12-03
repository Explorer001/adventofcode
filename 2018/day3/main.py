FIELDSIZE = 1000

def format_input():
    pinput = []
    fd = open("puzzleinput", "r")
    for line in fd.readlines():
        element = []
        line = line.strip("\n")
        line = line.replace(" ", "")
        #format left side of @
        ID, pos = line.split("@")
        element.append(ID.strip("#"))

        #format right side of @
        dist, size = pos.split(":")
        ldist, rdist = dist.split(",")
        length, width = size.split("x")
        element.append((int(ldist), int(rdist)))
        element.append((int(length) + int(ldist), int(width) + int(rdist)))
        
        #add element to input
        pinput.append(element[:])
    fd.close()
    return pinput

def print_field(field):
    for row in field:
        print(''.join(row) + "\n")

def draw_rectangle(topleft, botright, field):
    for i in range(topleft[0], botright[0]):
        for j in range(topleft[1], botright[1]):
            if field[j][i] == ".":
                field[j][i] = "#"
            else:
                field[j][i] = "x"

def get_overlapping_count(field):
    count = 0
    for row in field:
        line = ''.join(row)
        count += line.count("x")
    return count

def get_overlapping(pinput):
    field = [["."] * FIELDSIZE for _ in range(FIELDSIZE)]
    for ID, tl, br in pinput:
        draw_rectangle(tl, br, field)
    return (get_overlapping_count(field), field)
    
def is_overlapping(field, tl, br):
    for i in range(tl[0], br[0]):
        for j in range(tl[1], br[1]):
            if field[j][i] == "x":
                return True
    return False

def get_intact(field, pinput):
    for ID, tl, br in pinput:
        overlaps = is_overlapping(field, tl, br)
        if not overlaps:
            return ID
    return -1

pinput = format_input()
count, field = get_overlapping(pinput)
print(f"Overlapping: {count}")
ID = get_intact(field, pinput)
print(f"Not overlapping: {ID}")
