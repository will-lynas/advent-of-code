with open("input") as f:
     lines = f.read().strip().split("\n")

directions = {
        "R": (0, 1),
        "L": (0, -1),
        "U": (-1, 0),
        "D": (1, 0)
        }

b = 0
corners = [(0,0)]
for line in lines:
    split_line = line.split(" ")
    direction = directions[split_line[0]]
    distance = int(split_line[1])
    dx = direction[0] * distance
    dy = direction[1] * distance
    x, y = corners[-1]
    corners.append((x + dx, y + dy))
    b += abs(dx) + abs(dy)

area = 0
# Shoelace
for i in range(len(corners)-1):
    area += (corners[i][1] * corners[i+1][0] -
                corners[i][0] * corners[i+1][1])
area //= 2
# Pick's theorem
interior = (area - b//2 + 1)
points = interior + b
print(points)
