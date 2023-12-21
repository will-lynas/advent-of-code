from collections import defaultdict, deque

with open("input") as f:
    part1, part2 = f.read().strip().split("\n\n")

routines = defaultdict(list)
for line in part1.split("\n"):
    name, instructions = line[:-1].split('{')
    '}' # My lsp doesn't understand brackets inside strings
    for ins in instructions.split(","):
        if ":" not in ins:
            routines[name].append(["True", ins])
        else:
            routines[name].append(ins.split(":"))

parts = []
for part in part2.split("\n"):
    parts.append([int(stat.split("=")[1])
                 for stat in part[1:-1].split(",")])

q = deque()
for part in parts:
    q.append((part, "in"))

accepted = []
while q:
    part, routine = q.popleft()
    x, m, a, s = part
    for condition, outcome in routines[routine]:
        if eval(condition):
            if outcome == "A":
                accepted.append(part)
            elif outcome == "R":
                pass
            else:
                q.append((part, outcome))
            break
print(sum(map(sum, accepted)))
