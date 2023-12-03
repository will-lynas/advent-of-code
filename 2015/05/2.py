from collections import defaultdict

with open("input") as f:
    lines = f.read().strip().split("\n")
n = len(lines[0])
out = 0
for line in lines:
    pairs = defaultdict(list)
    triple_good = False
    for i in range(len(line)):
        if i < n-1:
            pairs[line[i:i+2]].append(i)
        if not triple_good and i >= 2 and line[i] == line[i-2]:
            triple_good = True
    if not triple_good:
        continue
    pairs_good = False
    for pair in pairs.values():
        if len(pair) >= 2 and pair[-1]-pair[0] >= 2:
            pairs_good = True
            break
    if not pairs_good:
        continue
    out += 1
print(out)
