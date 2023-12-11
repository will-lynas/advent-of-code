with open("input") as f:
    lines = list(map(list, f.read().strip().split("\n")))

def transpose(l):
    return list(map(list,zip(*l)))

def count_gaps(l, multiplier=1_000_000):
    # insert blanks before current line
    out = [0]
    for line in l[1:]:
        if all(c == "." for c in line):
            n = multiplier
        else:
            n = 1
        out.append(out[-1]+n)
    return out


rows = count_gaps(lines)
lines = transpose(lines)
cols = count_gaps(lines)
lines = transpose(lines)

gals = []
for i, line in enumerate(lines):
    for j, c in enumerate(line):
        if c == "#":
            gals.append((i,j))

count = 0
for i, (x1,y1) in enumerate(gals):
    for (x2,y2) in gals[i+1:]:
        count += abs(rows[x2] - rows[x1]) + abs(cols[y2] - cols[y1])
print(count)
