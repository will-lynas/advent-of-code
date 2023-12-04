with open("input") as f:
    lines = [line.split(":")[1] for line in f.read().strip().split("\n")]
out = 0
for line in lines:
    winning = [int(n) for n in line.split("|")[0].strip().split()]
    mine = [int(n) for n in line.split("|")[1].strip().split()]
    score = 1
    for n in mine:
        if n in winning:
            score *= 2
    if score != 1:
        out += score//2
print(out)
