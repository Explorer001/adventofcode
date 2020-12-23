#!/bin/python
import sys
import re

#
# Processes passport string.
#
# @param pstring String containing passport data.
# @return List of parsed passport dicts.
def parse_passports(pstring):
    plist = []
    pports = pstring.strip().split("\n\n")
    
    for pport in pports:
        props = pport.split()
        dct = {}

        for prop in props:
            k,v = prop.split(":")
            dct[k] = v

        plist.append(dct)
    
    return plist

#
# Checks if a passport is valid.
#
# @param pport The passport dict.
# @param Simple or advanced check.
# @return True if valid, False if not.
def passport_valid(pport, simple):
    fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "ecl", "pid"]
    eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]

    for field in fields:
        if field not in pport:
            return False

    if simple:
        return True

    fourp = re.compile(r'\d{4}\Z')

    if not fourp.match(pport["byr"]):
        return False
    if not 1920 <= int(pport["byr"]) <= 2002:
        return False

    if not fourp.match(pport["iyr"]):
        return False
    if not 2010 <= int(pport["iyr"]) <= 2020:
        return False

    if not fourp.match(pport["eyr"]):
        return False
    if not 2020 <= int(pport["eyr"]) <= 2030:
        return False

    metricp = re.compile(r'\d*cm\Z')
    retardp = re.compile(r'\d*in\Z')

    if metricp.match(pport["hgt"]):
        height = int(re.findall(r'\b\d*\B', pport["hgt"])[0])
        if not 150 <= height <= 193:
            return False
    elif retardp.match(pport["hgt"]):
        height = int(re.findall(r'\b\d*\B', pport["hgt"])[0])
        if not 59 <= height <= 76:
            return False
    else:
        return False

    hexp = re.compile(r'#[0-9a-f]{6}\Z')
    if not hexp.match(pport["hcl"]):
        return False

    if pport["ecl"] not in eye_colors:
        return False

    pidp = re.compile(r'[0-9]{9}\Z')
    if not pidp.match(pport["pid"]):
        return False

    return True

#
# Count valid passports.
#
# @param pports List of passports.
# @param simple Simple or advanced check.
# @return number of valid passports.
def count_valids(pports, simple):
    valid = 0

    for pport in pports:
        if passport_valid(pport, simple):
            valid += 1

    return valid

def main():

    if len(sys.argv) < 2:
        return

    fin = open(sys.argv[1], "r")
    data = fin.read()
    fin.close()

    pports = parse_passports(data)

    print("Valid passports: " + str(count_valids(pports, True)))
    print("Valid passports: " + str(count_valids(pports, False)))

main()
