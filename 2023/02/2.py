from collections import defaultdict
from functools import reduce

file_name = "input"
with open(file_name) as f:
    lines = [[[colour.strip().split(" ")
               for colour in
               group.strip().split(",")]
              for group in
              line.split(":")[1].strip().split(";")]
             for line in
             f.read().strip().split("\n")]

out = 0
for i,game in enumerate(lines):
    cubes = defaultdict(int)
    for group in game:
        for colour in group:
            cubes[colour[1]] = max(cubes[colour[1]], int(colour[0]))
    out += reduce(lambda x,y: x*y, cubes.values())
print(out)
