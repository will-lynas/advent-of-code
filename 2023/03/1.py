file_name = "input"
with open(file_name) as f:
    lines = ["."+line+".." for line in f.read().strip().split("\n")]
blank_line = "."*len(lines[0])
lines.append(blank_line)
lines.insert(0, blank_line)
directions = ((0,1),(0,-1),(1,0),(-1,0),(1,1),(-1,-1),(1,-1),(-1,1))

out = 0
for i in range(1,len(lines)-1):
    current_num = []
    valid = False
    for j in range(1,len(lines[0])-1):
        if lines[i][j].isnumeric():
            current_num.append(lines[i][j])
            if not valid:
                for di, dj in directions:
                    cell = lines[i+di][j+dj]
                    if not cell.isnumeric() and cell != ".":
                        valid = True
                        break
        else:
            if valid:
                number_to_add = int("".join(current_num))
                out += number_to_add
                valid = False
            current_num = []
print(out)
