#!/usr/bin/env python2

inputs = []

inputs += [0 for i in range(10000)]

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

pc = 0
rb = 0
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
		inputs[p1] = int(raw_input("Input: "))
		pc += 2

	elif int(instruction[-2:]) == 4:
		print inputs[p1]
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
