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

tx = 14
ty = 17

inputs = [list(r) for r in inputs.strip().split("\n")]

def simplify(x,y):
	hcf = int(math.gcd(x,y))
	return int(x/hcf), int(y/hcf), hcf

def heading(p):
	r = (math.atan2(-(ty-p[1]),tx-p[0]) + math.pi/2 ) % (2*math.pi)
	if r == 0:
		r = 2*math.pi
	return r

latest = [0,0,0]

while latest[2] < 200:
	points = []
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
					inputs[y][x] = "#"
					points.append([x,y])
	
	points.sort(key=heading, reverse=True)

	"""
	#Visual debugging
	inputs[ty][tx] = "X"
	for p in points:
		inputs[p[1]][p[0]] = "?"

	for i in range(10):
		p = points[i]
		inputs[p[1]][p[0]] = str(i)

	print("\n".join(["".join(row) for row in inputs]))
	break
	"""

	for p in points:
		if latest[2] < 200:
			latest[0] = p[0]
			latest[1] = p[1]
			latest[2] += 1
			inputs[p[1]][p[0]] = "."
		else:
			break

print (latest[0]*100+latest[1])
