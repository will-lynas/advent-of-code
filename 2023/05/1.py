with open("input") as f:
    groups = f.read().strip().split("\n\n")

seeds = [int(n) for n in groups[0].split(":")[1].strip().split(" ")]
maps = []
for group in groups[1:]:
    out = []
    for line in group.split("\n")[1:]:
        out.append([int(n) for n in line.strip().split(" ")])
    maps.append(out)

for map in maps:
    for i,seed in enumerate(seeds):
        for submap in map:
            if 0 <= (diff := seed-submap[1]) < submap[2]:
                seeds[i] = submap[0] + diff
                break
print(min(seeds))
