with open("input") as f:
     lines = f.read().strip().split("\n")

directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]

b = 0
corners = [(0,0)]
for line in lines:
    hex = line.split(" ")[2]
    direction = directions[int(hex[7])]
    distance = int(hex[2:7], 16)
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
