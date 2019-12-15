#!/usr/bin/env python2

with open("input.txt") as file:
	lines = [l.strip().split(")") for l in file.readlines()]

def getOrbits(lines,i):
	path = []
	if lines[i][0] == "COM":
		path.append(lines[i][1])
		path.append("COM")
		return path
	else:
		path.append(lines[i][1])
		target = lines[i][0]
		for j in range(len(lines)):
			if lines[j][1] == target:
				path += getOrbits(lines,j)
				return path

total = 0

for i in range(len(lines)):
	if lines[i][1] == "YOU":
		you =  getOrbits(lines,i)[::-1]
	elif lines[i][1] == "SAN":
		san = getOrbits(lines,i)[::-1]

for i in range(min(len(you),len(san))):
	if you[i] != san[i]:
		point = i
		break

print len(san)-point + len(you)-point - 2