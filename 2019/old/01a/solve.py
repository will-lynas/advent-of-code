#!/usr/bin/env python2

with open("input.txt") as file:
	inputs =  [int(l.strip()) for l in file.readlines()]

total = 0
for n in inputs:
	fuel = n//3 - 2
	total += fuel

print total
