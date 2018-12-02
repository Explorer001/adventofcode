def get_input():
    pinput = []
    fd = open("puzzleinput", "r")
    for line in fd.readlines():
        line = line.strip("\n")
        pinput.append(line)
    fd.close()
    return pinput

def diff(word1, word2):
    diff = 0
    for i in range(len(word1)):
        if word1[i] != word2[i]:
            diff += 1
    return diff

def char_count(word):
    chars = []
    count = []
    for char in word:
        if char not in chars:
            chars.append(char)
            count.append(word.count(char))
    return list(zip(chars, count))

def calc_checksum(pinput):
    doubles = 0
    triples = 0
    for ID in pinput:
        charcount = char_count(ID)
        got_double = False
        got_triple = False
        for char, count in charcount:
            if count == 2 and not got_double:
                got_double = True
                doubles += 1
            elif count == 3 and not got_triple:
                got_triple = True
                triples += 1
    return doubles * triples

def common(word1, word2):
    new_word = ""
    for i in range(len(word1)):
        if word1[i] == word2[i]:
            new_word += word1[i]
    return new_word

def get_common_letters(pinput):
    for i in range(len(pinput)):
        for j in range(i+1, len(pinput)):
            differ = diff(pinput[i], pinput[j])
            if differ == 1:
                return (common(pinput[i], pinput[j]))


pinput = get_input()
checksum = calc_checksum(pinput)
print(f"Checksum: {checksum}")
common = get_common_letters(pinput)
print(f"Common chars: {common}")
