from operator import itemgetter

def format_input():
    pinput = []
    fd = open("puzzleinput", "r")
    for line in fd.readlines():
        element = []

        line = line.strip("\n")
        timestamp, log = line.split("]")

        #format time
        date, time = timestamp.split(" ")
        year, month, day = date[1:].split("-")
        hour, minute = time.split(":")
        element.append(([int(year), int(month), int(day)], [int(hour), int(minute)]))
        
        #format log
        if "begins" in log:
            parts = log[1:].split(" ")
            for part in parts:
                if part[0] == "#":
                    element.append(int(part[1:]))
                    break
        elif "wakes" in log:
            element.append("w")
        elif "falls" in log:
            element.append("f")
        pinput.append(element[:])
    fd.close()
    
    #sort input
    pinput = sorted(pinput, key=itemgetter(0))

    return pinput

pinput = format_input()
for line in pinput:
    print(line)
