with open("input") as f:
    lines = [[int(n) for n in line.split()] for line in f.read().strip().split("\n")]

out = 0
for line in lines:
    big_lines = [line]
    while len(set(big_lines[-1])) > 1:
        big_lines.append([big_lines[-1][i+1]-big_lines[-1][i] for i in range(len(big_lines[-1])-1)])
    for i in reversed(range(len(big_lines)-1)):
        big_lines[i].insert(0, big_lines[i][0]-big_lines[i+1][0])
    out += big_lines[0][0]
print(out)
