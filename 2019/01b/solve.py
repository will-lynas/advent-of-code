#!/usr/bin/env python2

with open("input.txt") as file:
	inputs =  [int(l.strip()) for l in file.readlines()]

def getFuel(n):
	fuel = n//3 - 2
	if fuel <= 0:
		return 0
	else:
		return fuel+getFuel(fuel)

total = 0
for n in inputs:
	total += getFuel(n)

print total
