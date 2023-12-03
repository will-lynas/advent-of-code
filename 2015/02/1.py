with open("input") as f:
    lines = [sorted([int(n) for n in line.split("x")]) for line in f.read().strip().split("\n")]
print(sum(3*line[0]*line[1] + 2*line[0]*line[2] + 2*line[1]*line[2] for line in lines))
