#!/usr/bin/env python2

inv = {}
reactions = {}
ore = 0

def parseReaction(s):
	inps, out = tuple([p.strip() for p in s.split("=>")])

	old_inps = [inp.strip().split(" ") for inp in inps.split(",")]
	inps = {}
	for inp in old_inps:
		inps[inp[1]] = int(inp[0])
	
	out = out.split(" ")

	reaction = {
		out[1]: {
			"inps": inps,
			"out_num": int(out[0])
		}
	}

	return reaction


with open("input.txt") as f:
	lines = f.readlines()
	
	for line in lines:
		reactions.update(parseReaction(line))

def react(target):
	
	if target not in inv:
			inv[target] = 0

	if target == "ORE":
		inv["ORE"] += 1
		global ore 
		ore += 1
		return

	for inp, inp_num in reactions[target]["inps"].items():
		if inp not in inv:
			inv[inp] = 0

		while inv[inp] < inp_num:
			react(inp)

		inv[inp] -= inp_num


	inv[target] += reactions[target]["out_num"]


react("FUEL")

print ore