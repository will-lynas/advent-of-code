with open("input") as f:
    lines = f.read().strip().split("\n")
instructions = lines[0]
network = {}
for line in lines[2:]:
    network[line[0:3]] = (line[7:10], line[12:15])
cur = "AAA"
steps = 0
while cur != "ZZZ":
    for ins in instructions:
        if ins == "L":
            cur = network[cur][0]
        else:
            cur = network[cur][1]
        steps += 1
print(steps)
