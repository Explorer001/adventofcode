progs = list("abcdefghijklmnop")
f = open('puzzleinput').read().strip().split(',')

def dance(reps, progs):
    seen = []
    for i in range(reps):
        s = ''.join(progs)
        if s in seen:  # cycles are short; no runtime lost for comparing full list instead of s == seen[0]
            print(seen[reps % i])
            return
        seen.append(s)

        for move in f:
            if move[0] == 's':
                i = int(move[1:])
                progs = progs[-i:] + progs[:-i]
            else:
                if move[0] == 'x':
                    a,b = map(int, move[1:].split('/'))
                    progs[a], progs[b] = progs[b], progs[a]
                if move[0] == 'p':
                    a,b = move[1:].split('/')
                    A = progs.index(a)
                    B = progs.index(b)
                    progs[A], progs[B] = progs[B], progs[A]

    print(''.join(progs))  # if no cycle - part 1

dance(1, progs[:])
dance(1000000000, progs[:])

