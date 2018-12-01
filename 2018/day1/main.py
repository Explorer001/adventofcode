def format_input():
    pinput = []
    fd = open("puzzleinput", "r")
    for line in fd.readlines():
        line = line.strip("\n")
        sign = line[0]
        num = int(line[1:])
        pinput.append((sign, num))
    return pinput

def get_freq(pinput):
    freq = 0
    for sign, num in pinput:
        if sign == "+":
            freq += num
        elif sign == "-":
            freq -= num
        
    return freq

def get_first_rep(pinput):
    freq = 0
    seen_freqs = [freq]
    while True:
        for sign, num in pinput:
            if sign == "+":
                freq += num
            elif sign == "-":
                freq -= num
            if freq in seen_freqs:
                return freq
            seen_freqs.append(freq)
    
pinput = format_input()
freq = get_freq(pinput)
rep = get_first_rep(pinput)
print(f"Frequency is: {freq}")
print(f"First repeatet frequency: {rep}")
