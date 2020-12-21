#!/bin/python
import sys

#
# Checks wether a password violates the given policy.
#
# @param lower The lower inclusive bound of char occurances in password.
# @param upper The upper inclusive bound of char occurances in password.
# @param char The specified character of the policy.
# @param passwd The given password.
# @return True if password is valid, False otherwise.
def is_valid_pw1(lower, upper, char, passwd):
    count = passwd.count(char)
    if count < lower or count > upper:
        return False
    return True

#
# Checks wether a password violates the given policy.
#
# @param i1 Lower index.
# @param i2 Upper index.
# @param char The specified character of the policy.
# @param passwd The given password.
# @return True if password is valid, False otherwise.
def is_valid_pw2(i1, i2, char, passwd):
    c1 = passwd[i1-1]
    c2 = passwd[i2-1]

    if c1 == c2:
        return False
    elif c1 == char or c2 == char:
        return True
    return False

#
# Parses password an policy and returns is_valid.
# 
# @param _passwd The password an policy string
# @param _policy Number of the policy.
# @return True if password is valid, False otherwise.
def parse_is_valid(_passwd, _policy):
    policy, passwd = _passwd.split(": ")
    interval, char = policy.split()
    lower, upper = interval.split("-")
    if _policy == 1:
        return is_valid_pw1(int(lower), int(upper), char, passwd)
    elif _policy == 2:
        return is_valid_pw2(int(lower), int(upper), char, passwd)
    else:
        return False

#
# Returns count of valid passwords in data.
#
# @param data The input data (list of strings).
# @param policy Number of the policy.
# @return The number of valid passwords in data.
def count_valids(data, policy):
    valids = 0

    for line in data:
        if parse_is_valid(line, policy):
            valids += 1

    return valids

def main():
    if len(sys.argv) < 2:
        return

    fin = open(sys.argv[1], "r")
    data = fin.read()
    fin.close()

    print("Valid passwords: " + str(count_valids(data.splitlines(), 1)))
    print("Valid passwords: " + str(count_valids(data.splitlines(), 2)))

main()
