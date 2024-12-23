from operator import itemgetter

def format_input():
    pinput = []
    num_guards = 0
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
                    guard_num = int(part[1:])
                    if guard_num > num_guards:
                        num_guards = guard_num
                    element.append(guard_num)
                    break
        elif "wakes" in log:
            element.append("w")
        elif "falls" in log:
            element.append("f")
        pinput.append(element[:])
    fd.close()
    
    #sort input
    pinput = sorted(pinput, key=itemgetter(0))
    return (num_guards + 1, pinput)

def get_max_index(l):
    maximum = 0
    index = 0
    for i in range(len(l)):
        if l[i] > maximum:
            maximum = l[i]
            index = i
    return index

def get_sleep(pinput, num_guards):
    sleep_time = [[0]*60 for _ in range(num_guards)]
    stime = 0
    wtime = 0
    active_guard = 0
    for i in range(len(pinput)):
        ts, log = pinput[i]
        if type(log) is int:
            active_guard = log
        else:
            _, time = ts
            if log == "f":
                stime = time[1]
            else:
                wtime = time[1]
                sleep_list = sleep_time[active_guard]
                for i in range(stime, wtime):
                    sleep_list[i] += 1
    return sleep_time

def part1(sleep_time):
    max_min_sleep = 0
    max_index = -1
    for i in range(len(sleep_time)):
        l = sleep_time[i]
        mx = sum(l)
        if mx > max_min_sleep:
            max_min_sleep = mx
            max_index = i
    return max_index * get_max_index(sleep_time[max_index])

def part2(sleep_time):
    max_min_index = 0
    max_min_val = 0
    max_guard = 0
    for i in range(60):
        for j in range(len(sleep_time)):
            l = sleep_time[j]
            if l[i] > max_min_val:
                max_min_val = l[i]
                max_min_index = i
                max_guard = j
    return max_min_index * max_guard

num_days, pinput = format_input()
sleep_time = get_sleep(pinput, num_days)
sol1 = part1(sleep_time)
print(f"Solution1: {sol1}")
sol2 = part2(sleep_time)
print(f"Solution2: {sol2}")
