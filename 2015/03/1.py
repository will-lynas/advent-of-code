from collections import defaultdict

with open("input") as f:
    dirs = f.read().strip()
dir_trans = {">": (0,1), "<": (0, -1), "^": (-1,0), "v": (1,0)}

visited = defaultdict(int)
count = 0
x,y = 0,0
visited[(x,y)] += 1
for d in dirs:
    dx, dy = dir_trans[d]
    x += dx
    y += dy
    visited[(x,y)] += 1
print(len(visited))
