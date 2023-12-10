#!/usr/bin/env python2

import itertools

original = [3,8,1001,8,10,8,105,1,0,0,21,38,63,76,89,106,187,268,349,430,99999,3,9,1001,9,5,9,102,3,9,9,1001,9,2,9,4,9,99,3,9,101,4,9,9,102,3,9,9,101,4,9,9,1002,9,3,9,101,2,9,9,4,9,99,3,9,101,5,9,9,1002,9,4,9,4,9,99,3,9,101,2,9,9,1002,9,5,9,4,9,99,3,9,1001,9,5,9,1002,9,5,9,1001,9,5,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,99]


def pad(s):
	return "0"*(4-len(s))+s


def parseParams(instruction, pc):

	if int(instruction[-3]) == 1:
			p1 = inputs[pc+1]
	elif int(instruction[-3]) == 0:
		p1 = inputs[inputs[pc+1]]
	else:
		print "Parsing error"

	if int(instruction[-4]) == 1:
		p2 = inputs[pc+2]
	elif int(instruction[-4]) == 0:
		p2 = inputs[inputs[pc+2]]
	else:
		print "Parsing error"

	return p1,p2

settings = [5,6,7,8,9]
maximum = 0
for setting in itertools.permutations(settings):
	states = [{"inputs": original[:], "pc": 0, "first_input": True, "finished": False} for i in range(5)]
	inp = 0
	while not states[4]["finished"]:
		for i in range(5):

			inputs = states[i]["inputs"]
			pc = states[i]["pc"]

			while True:
				instruction = pad(str(inputs[pc]))

				if inputs[pc] == 99:
					states[i]["finished"] = True
					break

				elif int(instruction[-2:]) == 1:
						
					p1,p2 = parseParams(instruction,pc)
					inputs[inputs[pc+3]] = p1 + p2
					pc += 4

				elif int(instruction[-2:]) == 2:
					
					p1,p2 = parseParams(instruction,pc)
					inputs[inputs[pc+3]] = p1 * p2
					pc += 4

				elif int(instruction[-2:]) == 3:
					if states[i]["first_input"]:
						inputs[inputs[pc+1]] = setting[i]
						states[i]["first_input"] = False
					else:
						inputs[inputs[pc+1]] = inp

					pc += 2

				elif int(instruction[-2:]) == 4:

					inp = inputs[inputs[pc+1]]
					pc += 2
					states[i]["inputs"] = inputs
					states[i]["pc"] = pc
					break

				elif int(instruction[-2:]) == 5:

					p1,p2 = parseParams(instruction,pc)

					if p1 != 0:
						pc = p2
					else:
						pc += 3

				elif int(instruction[-2:]) == 6:
					
					p1,p2 = parseParams(instruction,pc)

					if p1 == 0:
						pc = p2
					else:
						pc += 3

				elif int(instruction[-2:]) == 7:
					
					p1,p2 = parseParams(instruction,pc)

					inputs[inputs[pc+3]] = int(p1 < p2)
					pc += 4


				elif int(instruction[-2:]) == 8:

					p1,p2 = parseParams(instruction,pc)

					inputs[inputs[pc+3]] = int(p1 == p2)
					pc += 4


				else:
					print str(inputs[pc])+" is unknown"

	if inp > maximum:
		maximum = inp

print maximum