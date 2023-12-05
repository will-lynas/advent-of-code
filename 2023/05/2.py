from collections import deque

with open("input") as f:
    groups = f.read().strip().split("\n\n")

raw_seeds = [int(n) for n in groups[0].split(":")[1].strip().split(" ")]
seeds = deque()
while raw_seeds:
    start = raw_seeds.pop(0)
    length = raw_seeds.pop(0)
    seeds.append([start, length])

maps = []
for group in groups[1:]:
    out = []
    for line in group.split("\n")[1:]:
        out.append([int(n) for n in line.strip().split(" ")])
    maps.append(out)

for map in maps:
    new_seeds = deque()
    while seeds:
        start, length = seeds.popleft()
        match = False
        for submap in map:
            submap_dst_start, submap_src_start, submap_length = submap
#            print(f"[{start},{start+length}), "
#                  f"[{submap_src_start},{submap_src_start+submap_length})->"
#                  f"[{submap_dst_start},{submap_dst_start+submap_length})")
            if 0 <= (diff := start-submap_src_start) < submap_length:
                match = True
                if length > (length_to_end := submap_length-diff):
                    new_seeds.append([submap_dst_start+diff, length_to_end])
                    seeds.append([start+length_to_end, length-length_to_end])
                else:
                    new_seeds.append([submap_dst_start+diff, length])
                break
        if not match:
            new_seeds.append([start, length])
    seeds = new_seeds

print(min(seeds)[0])
