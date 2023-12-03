file_name = "input"
with open(file_name) as f:
    line = f.read().strip()
floor = 0
for i, c in enumerate(line):
    if c == "(":
        floor += 1
    else:
        floor -= 1
    if floor < 0:
        print(i+1)
        break
