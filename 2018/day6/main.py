class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.cluster = []

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def __lt__(self, other):
        if self.x < other.x:
            return True
        if self.x == other.x:
            if self.y < other.y:
                return True
        return False

    def dist(self, other):
        return abs((self.x - other.x)) +abs( (self.y - other.y))

    def __print__(self):
        l = len(self.cluster)
        print(f"[{self.x}, {self.y}] | {l}")

    def __str__(self):
        l = len(self.cluster)
        return f"[{self.x}, {self.y}] | {l}"

def format_input():
    pinput = []
    fd = open("puzzleinput", "r")
    for line in fd.readlines():
        line = line.strip("\n")
        x, y = line.split(", ")
        pinput.append(Point(int(x), int(y)))
    fd.close()
    pinput.sort()
    return pinput

def find_min_index(l):
    min_ = max(l)
    min_i = -1
    for i in range(len(l)):
        if l[i] < min_:
            min_ = l[i]
            min_i = i
    #check if more points are min dist
    nl = sorted(l)
    if nl[0] == nl[1]:
        return -1
    return min_i

def min_dist_index(plist, point):
    dists = []
    #calc dists to all points
    for i in range(len(plist)):
        dists.append(point.dist(plist[i]))
    return find_min_index(dists)

def filter_infinite(ls, maxx, maxy):
    new_list = []
    for p  in ls:
        cont = True
        #if one point in cluster is field border, the cluster is infinite
        for pn in p.cluster:
            if pn.x == maxx or pn.x == 0 or pn.y == maxy or pn.y == 0:
                cont = False
                break

        if not cont:
            continue
        new_list.append(p)
    return new_list

def cluster(plist):
    for j in range(plist[-1].y + 1):
        for i in range(plist[-1].x + 1):
            p = Point(i,j)
            #calc dist from point to any cluster point
            index = min_dist_index(plist, p)
            if index >= 0:
                #update cluster points if point is near
                plist[index].cluster.append(p)
    finite_list = filter_infinite(plist, plist[-1].x, plist[-1].y)
    max_ = 0
    for p in finite_list:
        if len(p.cluster) > max_:
            max_ = len(p.cluster)
    return max_

def get_safe(plist):
    safe_count = 0
    for j in range(plist[-1].y + 1):
        for i in range(plist[-1].x + 1):
            p = Point(i, j)
            dist = 0
            #dist to all points
            for pn in plist:
                dist += p.dist(pn)
            if dist < 10000:
                safe_count += 1
    return safe_count

pinput = format_input()
mx = cluster(pinput)
print(f"Max Cluster: {mx}")
pinput = format_input()
safe = get_safe(pinput)
print(f"Safe site: {safe}")
