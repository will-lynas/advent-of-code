with open("input") as f:
    lines = [["."] + list(line) + ["."] for line in f.read().strip().split("\n")]
    extra_line = ["."] * len(lines[0])
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
pairs = {
        "J": "L",
        "7": "F"
        }

# Find S
i,j = 0,0
for i,line in enumerate(lines):
    if "S" in line:
        j = line.index("S")
        break
path = [(i,j)]

def backwards(d):
    return (d[0]*-1, d[1]*-1)

# Replace S with the correct part
s_parts = []
for direction in ((0,1), (0,-1), (1,0), (-1,0)):
    x,y = direction
    val = lines[i+x][j+y]
    if val in directions and backwards(direction) in directions[val]:
        s_parts.append((x,y))

for pipe, dirs in directions.items():
    if dirs == tuple(s_parts):
        lines[i][j] = pipe
        break

# Start the path
path.append((s_parts[0][0]+i, s_parts[0][1]+j)) # Either one; doesn't matter

# Generate the path
found = True
while found:
    x,y = path[-1]
    val = lines[x][y]
    found = False
    for dx,dy in directions[val]:
        tx = x+dx
        ty = y+dy
        if (tx, ty) in path:
            continue
        else:
            found = True
            path.append((tx,ty))


count = 0
for i in range(len(lines)):
    outside = True
    wall_start = ""
    for j in range(len(lines[0])):
        val = lines[i][j]
        if (i,j) in path:
            if val == "|":
                outside = not outside
            elif val in pairs.values():
                wall_start = val
            elif val in pairs:
                if wall_start != pairs[val]:
                    outside = not outside
        elif not outside:
            count += 1
print(count)
