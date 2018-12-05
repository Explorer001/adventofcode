def format_input():
    fd = open("puzzleinput", "r")
    pinput = fd.read()
    pinput = pinput.strip("\n")
    fd.close()
    return pinput

def react(pinput):
    changed = True
    strlen = len(pinput)
    while changed:
        changed = False
        i = 0
        while i < strlen - 1:
            c1 = pinput[i]
            c2 = pinput[i+1]
            if c1.lower() == c2.lower():
                if (c1.islower() and c2.isupper()) or (c1.isupper() and c2.islower()):
                    changed = True
                    pinput = pinput[:i] + pinput[i+2:]
                    strlen = len(pinput)
            i += 1
    return len(pinput)

def get_shortest(pinput, n):
    ascii_A = 65
    ascii_a = 97
    min_ = n
    for i in range(26):
        c1 = chr(i+ascii_A)
        c2 = chr(i+ascii_a)
        new_poly = pinput.replace(c1, '')
        new_poly = new_poly.replace(c2, '')
        rv = react(new_poly)
        if rv < min_:
            min_ = rv
    return min_


pinput = format_input()
size = react(pinput)
print(f"Polymer size: {size}")
shortest = get_shortest(pinput, size)
print(f"Shortest Polymer: {shortest}")
