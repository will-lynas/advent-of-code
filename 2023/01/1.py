with open("input.txt") as f:
    lines = f.read().strip().split("\n")
out = 0
for line in lines:
    left = 0
    right = len(line)-1
    while not line[left].isnumeric():
        left += 1
    while not line[right].isnumeric():
        right -= 1
    out += int(line[left])*10 + int(line[right])
print(out)
