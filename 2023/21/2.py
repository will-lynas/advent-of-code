steps = 5000
filename = "example"
with open(filename) as f:
    lines = list(map(list, f.read().strip().split("\n")))

cur = set()
found = False
for i,line in enumerate(lines):
    if found:
        break
    for j,c in enumerate(line):
        if c == "S":
            cur.add((i,j))
            lines[i][j] = "."
            found = True
            break

directions = [(0,1), (0,-1), (1,0), (-1,0)]

m = len(lines)
n = len(lines[0])
for step in range(steps):
    next = set()
    for x, y in cur:
        for dx, dy in directions:
            tx = x + dx
            ty = y + dy
            try:
                if lines[tx%m][ty%n] != "#":
                    next.add((tx,ty))
            except IndexError:
                pass
    cur = next
    print(step, len(cur))
