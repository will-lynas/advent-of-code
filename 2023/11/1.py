with open("input") as f:
    lines = list(map(list, f.read().strip().split("\n")))

def transpose(l):
    return list(map(list,zip(*l)))

def duplicate_rows(l):
    for i in reversed(range(len(l))):
        if all(c == "." for c in l[i]):
            l.insert(i, ["."] * len(l[0]))

duplicate_rows(lines)
lines = transpose(lines)
duplicate_rows(lines)
lines = transpose(lines)

gals = []
for i, line in enumerate(lines):
    for j, c in enumerate(line):
        if c == "#":
            gals.append((i,j))

count = 0
for i, (x1,y1) in enumerate(gals):
    for (x2,y2) in gals[i+1:]:
        count += abs(x2 - x1) + abs(y2 - y1)
print(count)
