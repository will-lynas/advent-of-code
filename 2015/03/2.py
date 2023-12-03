from collections import defaultdict

with open("input") as f:
    dirs = f.read().strip()
dir_trans = {">": (0,1), "<": (0, -1), "^": (-1,0), "v": (1,0)}

visited = defaultdict(int)
count = 0
x,y = 0,0
rx,ry = 0,0
visited[(x,y)] += 1
visited[(rx,ry)] += 1
for i,d in enumerate(dirs):
    dx, dy = dir_trans[d]
    if i%2:
        rx += dx
        ry += dy
        visited[(rx,ry)] += 1
    else:
        x += dx
        y += dy
        visited[(x,y)] += 1
print(len(visited))
