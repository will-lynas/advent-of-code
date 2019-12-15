#!/usr/bin/env python3

import math

inputs= """
##.##..#.####...#.#.####
##.###..##.#######..##..
..######.###.#.##.######
.#######.####.##.#.###.#
..#...##.#.....#####..##
#..###.#...#..###.#..#..
###..#.##.####.#..##..##
.##.##....###.#..#....#.
########..#####..#######
##..#..##.#..##.#.#.#..#
##.#.##.######.#####....
###.##...#.##...#.######
###...##.####..##..#####
##.#...#.#.....######.##
.#...####..####.##...##.
#.#########..###..#.####
#.##..###.#.######.#####
##..##.##...####.#...##.
###...###.##.####.#.##..
####.#.....###..#.####.#
##.####..##.#.##..##.#.#
#####..#...####..##..#.#
.##.##.##...###.##...###
..###.########.#.###..#.
"""


inputs = inputs.strip().split("\n")

def simplify(x,y):
	hcf = int(math.gcd(x,y))
	return int(x/hcf), int(y/hcf), hcf

maximum = 0

for ty in range(len(inputs)):
	for tx in range(len(inputs[0])):
		if inputs[ty][tx] == "#":
			count = 0

			for y in range(len(inputs)):
				for x in range(len(inputs[0])):

					if inputs[y][x] == "#" and not (ty == y and tx == x):
						found = True
						diffy = y-ty
						diffx = x-tx

						dx, dy, hcf = simplify(diffx,diffy)

						for i in range(hcf-1):
							if inputs[ty+dy*(i+1)][tx+dx*(i+1)] == "#":
								found = False
								break

						if found:
							count += 1

			if count > maximum:
				maximum = count

print (maximum)