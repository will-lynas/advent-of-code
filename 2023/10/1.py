with open("input") as f:
    lines = ["." + line + "." for line in f.read().strip().split("\n")]
    extra_line = "." * len(lines[0])
    lines.insert(0, extra_line)
    lines.append(extra_line)

directions = {
        "-": ((0,-1), (0,1)),
        "|": ((-1,0), (1,0)),
        "J": ((0,-1), (-1,0)),
        "F": ((0,1), (1,0)),
        "L": ((0,1), (-1,0)),
        "7": ((0,-1), (1,0)),
        }

# Find S
i,j = 0,0
for i,line in enumerate(lines):
    if "S" in line:
        j = line.index("S")
        break
path = [(i,j)]

# Find first part of loop
for direction in ((0,1), (0,-1), (1,0), (-1,0)):
    x,y = direction
    val = lines[i+x][j+y]
    if val in directions and direction in directions[val]:
        path.append((i+x, j+y))
        break

is_start = False
while not is_start:
    x,y = path[-1]
    prev = path[-2]
    val = lines[x][y]
    for dx,dy in directions[val]:
        tx = x+dx
        ty = y+dy
        if (tx, ty) == prev:
            continue
        if lines[tx][ty] == "S":
            is_start = True
        else:
            path.append((tx,ty))

print(len(path)//2)
