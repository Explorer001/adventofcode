

def main():
    file = open("puzzleinput", "r")
    inp = file.read().strip().split(' ')
    file.close()
    ls = []
    print(inp)
    for i in range(len(inp)):
        if inp[i] != '':
            ls.append(int(inp[i]))
    print(solve(ls))

def solve(inp):
    states = []
    cycles = 0
    while inp not in states:
        states.append(inp[:])
        redist = get_first_max_index(inp)
        temp = inp[redist]
        inp[redist] = 0
        while temp > 0:
            redist = (redist+1)%len(inp)
            inp[redist] += 1
            temp -= 1
        cycles += 1
    states.append(inp[:])
    loopsize = 0
    for i in range(len(states)):
        if states[i] == inp:
            loopsize = len(states) - 1 - i
            break
    return (cycles, loopsize)
        
        

def get_first_max_index(ls):
    index = 0
    mx = 0
    for i in range(len(ls)):
        if ls[i] > mx:
            mx = ls[i]
            index = i
    return index

main()
