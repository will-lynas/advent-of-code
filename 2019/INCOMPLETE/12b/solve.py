import itertools

def getPosVel(pos,vel):
	return ":".join([",".join([str(c) for c in p]) for p in pos])+"-"+":".join([",".join([str(c) for c in p]) for p in vel])

pos = [[3,-6,6],[10,7,-9],[-3,-7,9],[-8,0,4]]
"""
pos = [[-1, 0, 2],
[2, -10, -7],
[4, -8, 8],
[3, 5, -1]]
"""
vel = [[0,0,0] for i in range(len(pos))]

original = getPosVel(pos,vel)

for i in xrange(99999999999999):
	if i % 10000 == 0:
		print i

	if getPosVel(pos,vel) == original and i != 0:
		break

	energy = 0
	for j in range(len(pos)):
		pot = 0
		kin = 0

		for k in range(3):
			pot += abs(pos[j][k])
			kin += abs(vel[j][k])

		energy += pot * kin

	for pair in list(itertools.combinations(range(len(pos)), 2)):
		p1,p2 = pair

		for j in range(3):
			if pos[p1][j] > pos[p2][j]:
				vel[p1][j] -= 1
				vel[p2][j] += 1
			elif pos[p1][j] < pos[p2][j]:
				vel[p1][j] += 1
				vel[p2][j] -= 1

	for j in range(len(pos)):
		for k in range(3):
			pos[j][k] += vel[j][k]

print i
