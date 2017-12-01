def solve(inp, stepsize):
    re = 0
    for i in range(len(inp)):
        suc = (i+stepsize)%len(inp)
        digit = int(inp[i])
        digitp1 = int(inp[suc])
        if digit == digitp1:
            re += digit
    return re

def main():
    infile = open("puzzleinput", "r")
    inp = infile.read().strip("\n")
    print(solve(inp,1))
    ss = int(len(inp)/2)
    print(solve(inp,ss))
    infile.close()

main()
