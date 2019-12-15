#!/usr/bin/env python2

inv = {}
inv["ORE"] = 1000000000000
reactions = {}

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


run_out = False

def react(target):
	global run_out

	if target not in inv:
			inv[target] = 0

	for inp, inp_num in reactions[target]["inps"].items():
		if inp not in inv:
			inv[inp] = 0

		while inv[inp] < inp_num:
			if inp == "ORE":
				run_out = True
			if run_out:
				return
			react(inp)

		inv[inp] -= inp_num

	inv[target] += reactions[target]["out_num"]

i = 0
while not run_out:
	react("FUEL")
	i += 1
	if i % 1000 == 0:
		print i

print inv["FUEL"]