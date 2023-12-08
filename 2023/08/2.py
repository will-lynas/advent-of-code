from math import lcm

with open("input") as f:
    lines = f.read().strip().split("\n")
instructions = lines[0]
network = {}
for line in lines[2:]:
    network[line[0:3]] = (line[7:10], line[12:15])

cur = [node for node in network if node[2] == "A"]
out = []
for node in cur:
    steps = 0
    while node[2] != "Z":
        ins = instructions[steps%len(instructions)]
        if ins == "L":
            node = network[node][0]
        else:
            node = network[node][1]
        steps += 1
    out.append(steps)
print(lcm(*out))
