NAME_INDEX = 1
AFTER_INDEX = 7

def format_input():
    pinput = []
    fd = open("puzzleinput", "r")
    for line in fd.readlines():
        line = line.strip("\n")
        tokens = line.split()
        pinput.append((tokens[NAME_INDEX], tokens[AFTER_INDEX]))
    fd.close()
    pinput.sort()
    return pinput

def get_index(char):
    #ascii value - ascii A
    return ord(char) - 65

def get_ascii(val):
    return chr(val+65)

def build_graph(nlist):
    num_nodes = get_index(nlist[-1][0]) + 1
    adj_mat = [([0]*num_nodes) for _ in range(num_nodes)]
    for name, after in nlist:
        nindex = get_index(name)
        aindex = get_index(after)
        adj_mat[nindex][aindex] = 1
    return adj_mat

def get_sink_index(adj_mat):
    for i in range(len(adj_mat)):
        if sum(adj_mat[i]) == 0:
            return i
    return -1

def get_src_index(adj_mat):
    l = len(adj_mat)
    src = True
    for i in range(l):
        src = True
        for j in range(l):
            if adj_mat[j][i] != 0:
                src= False
                break
        if src:
            return i
    return -1

def get_reachable(node):
    reachable = []
    for i in range(len(node)-1, -1, -1):
        if node[i] != 0:
            reachable.append(i)
    return reachable

def get_order(adj_mat):
    src_index = get_src_index(adj_mat)
    sink_index = get_sink_index(adj_mat)
    ord_str = get_ascii(src_index)
    
    done = [src_index]
    stack = []

    stack.append(src_index)

    while sink_index not in done:
        for i in range(len(adj_mat)):
            if i in done:
                continue
            is_done = True
            for j in range(len(adj_mat[i])):
                if adj_mat[j][i] != 0:
                    if j not in done:
                        is_done = False
                        break
            if is_done:
                done.append(i)
                ord_str += get_ascii(i)
                break

    return ord_str

pinput = format_input()
graph = build_graph(pinput)
order = get_order(graph)
print(f"Order: {order}")
