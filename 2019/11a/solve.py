#!/usr/bin/env python2

inputs = [3,8,1005,8,301,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,102,1,8,28,1006,0,98,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,101,0,8,54,2,1001,6,10,1,108,1,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,10,1002,8,1,84,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,101,0,8,105,1006,0,94,2,7,20,10,2,5,7,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,102,1,8,139,1006,0,58,2,1003,16,10,1,6,10,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,10,102,1,8,172,2,107,12,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,197,1006,0,34,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,102,1,8,223,1006,0,62,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,248,1,7,7,10,1006,0,64,2,1008,5,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,102,1,8,280,101,1,9,9,1007,9,997,10,1005,10,15,99,109,623,104,0,104,1,21102,1,387508351636,1,21101,318,0,0,1106,0,422,21102,1,838480007948,1,21101,0,329,0,1106,0,422,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,0,235190525123,1,21101,0,376,0,1105,1,422,21101,0,106505084123,1,21101,0,387,0,1106,0,422,3,10,104,0,104,0,3,10,104,0,104,0,21101,0,838324605292,1,21102,1,410,0,1105,1,422,21102,709496668940,1,1,21102,421,1,0,1105,1,422,99,109,2,22101,0,-1,1,21102,1,40,2,21101,0,453,3,21102,443,1,0,1106,0,486,109,-2,2105,1,0,0,1,0,0,1,109,2,3,10,204,-1,1001,448,449,464,4,0,1001,448,1,448,108,4,448,10,1006,10,480,1102,1,0,448,109,-2,2106,0,0,0,109,4,2101,0,-1,485,1207,-3,0,10,1006,10,503,21102,0,1,-3,22102,1,-3,1,21201,-2,0,2,21101,1,0,3,21102,1,522,0,1106,0,527,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,550,2207,-4,-2,10,1006,10,550,21202,-4,1,-4,1106,0,618,22102,1,-4,1,21201,-3,-1,2,21202,-2,2,3,21102,569,1,0,1106,0,527,21202,1,1,-4,21101,0,1,-1,2207,-4,-2,10,1006,10,588,21101,0,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,610,22101,0,-1,1,21101,0,610,0,106,0,485,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2106,0,0
]

inputs += [0 for i in range(10000)]
grid = [[None for j in range(1000)] for i in range(1000)]
pos = [499,499,0]


def pad(s):
	return "0"*(5-len(s))+s


def parseParams(pc):
	instruction = pad(str(inputs[pc]))

	if int(instruction[-3]) == 1:
		p1 = pc+1
	elif int(instruction[-3]) == 0:
		p1 = inputs[pc+1]
	elif int(instruction[-3]) == 2:
		p1 = rb+inputs[pc+1]
	else:
		print "Parsing error"

	if int(instruction[-4]) == 1:
		p2 = pc+2
	elif int(instruction[-4]) == 0:
		p2 = inputs[pc+2]
	elif int(instruction[-4]) == 2:
		p2 = rb+inputs[pc+2]
	else:
		print "Parsing error"

	if int(instruction[-5]) == 1:
		p3 = pc+3
	elif int(instruction[-5]) == 0:
		p3 = inputs[pc+3]
	elif int(instruction[-5]) == 2:
		p3 = rb+inputs[pc+3]
	else:
		print "Parsing error"

	return instruction,p1,p2,p3

def updatePos(pos,direction):
	if direction == 0:
		direction = -1
	pos[2] += direction
	pos[2] %= 4

	if pos[2] == 0:
		pos[1] += 1
	elif pos[2] == 1:
		pos[0] += 1
	elif pos[2] == 2:
		pos[1] += -1
	elif pos[2] == 3:
		pos[0] += -1

	return pos


pc = 0
rb = 0
input1 = True

while True:

	instruction,p1,p2,p3 = parseParams(pc)

	"""	
	raw_input("Continue?")
	print "instruction:",instruction
	print "params",p1,p2,p3
	print "pc:",pc
	print "rb:",rb
	print ""
	"""

	if inputs[pc] == 99:
		break

	elif int(instruction[-2:]) == 1:
		inputs[p3] = inputs[p1] + inputs[p2]
		pc += 4

	elif int(instruction[-2:]) == 2:
		inputs[p3] = inputs[p1] * inputs[p2]
		pc += 4

	elif int(instruction[-2:]) == 3:
		r = grid[pos[0]][pos[1]]
		if r == None:
			r = 0
		inputs[p1] = r
		pc += 2

	elif int(instruction[-2:]) == 4:
		if input1:
			input1 = False
			grid[pos[0]][pos[1]] = inputs[p1]
		else:
			input1 = True
			pos = updatePos(pos,inputs[p1])

		pc += 2

	elif int(instruction[-2:]) == 5:
		if inputs[p1] != 0:
			pc = inputs[p2]
		else:
			pc += 3

	elif int(instruction[-2:]) == 6:
		if inputs[p1] == 0:
			pc = inputs[p2]
		else:
			pc += 3

	elif int(instruction[-2:]) == 7:
		inputs[p3] = int(inputs[p1] < inputs[p2])
		pc += 4

	elif int(instruction[-2:]) == 8:
		inputs[p3] = int(inputs[p1] == inputs[p2])
		pc += 4

	elif int(instruction[-2:]) == 9:
		rb += inputs[p1]
		pc += 2

	else:
		print str(inputs[pc])+" is unknown"

print sum([row.count(0)+row.count(1) for row in grid])
