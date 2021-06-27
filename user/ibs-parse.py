#!/usr/bin/python3

from sys import argv
from struct import unpack

if len(argv) < 2:
    print("usage: {} <file>".format(argv[0]))
    exit()

with open(argv[1], "rb") as f:
    data = f.read()

cur = 0
entry_len = (8 * 8) + (4 * 4)
while cur < len(data):
    entry = unpack("<8Q4L", data[cur:cur+entry_len])

    ctl = entry[0]
    rip = entry[1]
    d = entry[2]
    d2 = entry[3]
    d3 = entry[4]
    d4 = entry[5]
    linad = entry[6]
    phyad = entry[7]

    print("{:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x}".format(
        rip, d, d2, d3, d4, linad, phyad
    ))

    cur += entry_len


