def format_input(name):
    fd = open(name, "r")
    pinput = fd.read().strip()
    tokens = pinput.split()
    tokens = list(map(int, tokens))
    return tokens

pinput = []

def main():
    global pinput
    p = format_input("puzzleinput")
    pinput = p
    print(calc_sum(0))
    

def calc_sum(pos):
    local_pos = pos
    ival = 0
    ccount = pinput[local_pos]
    mcount = pinput[local_pos + 1]

    local_pos += 2

    for i in range(ccount):
        val, npos = calc_sum(local_pos)
        ival += val
        local_pos = npos


    for i in range(mcount):
        ival += pinput[local_pos]
        local_pos += 1
    return (ival, local_pos)

main()
