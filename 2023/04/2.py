with open("input") as f:
    raw_lines = [line.split(":")[1] for line in f.read().strip().split("\n")]

lines = []
for line in raw_lines:
    winning = [int(n) for n in line.split("|")[0].strip().split()]
    mine = [int(n) for n in line.split("|")[1].strip().split()]
    lines.append((winning, mine))

occurences = [1 for _ in range(len(lines))]
for i, line in enumerate(lines):
    winning = line[0]
    mine = line[1]
    score = sum(1 for n in mine if n in winning)
    for j in range(score):
        occurences[i+j+1] += occurences[i]
print(sum(occurences))
