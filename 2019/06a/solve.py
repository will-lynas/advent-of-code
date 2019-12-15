#!/usr/bin/env python2

with open("input.txt") as file:
	lines = [l.strip().split(")") for l in file.readlines()]

def getOrbits(lines,i):
	if lines[i][0] == "COM":
		return 1
	else:
		target = lines[i][0]
		for j in range(len(lines)):
			if lines[j][1] == target:
				return 1+getOrbits(lines,j)

total = 0

for i in range(len(lines)):
	total += getOrbits(lines,i)

print total
