from collections import deque

with open("input") as f:
    raw_lines = [line.split(":")[1] for line in f.read().strip().split("\n")]

lines = []
for line in raw_lines:
    winning = [int(n) for n in line.split("|")[0].strip().split()]
    mine = [int(n) for n in line.split("|")[1].strip().split()]
    lines.append((winning, mine))

q = deque(range(len(lines)))
count = len(lines)
while q:
    i = q.popleft()
    winning = lines[i][0]
    mine = lines[i][1]
    score = sum(1 for n in mine if n in winning)
    for j in range(score):
        q.append(i+j+1)
        count += 1
    print(len(q))
print(count)
