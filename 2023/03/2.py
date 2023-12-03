file_name = "input"
with open(file_name) as f:
    lines = ["."+line+".." for line in f.read().strip().split("\n")]
blank_line = "."*len(lines[0])
lines.append(blank_line)
lines.insert(0, blank_line)
directions = ((0,1),(0,-1),(1,0),(-1,0),(1,1),(-1,-1),(1,-1),(-1,1))

def enumerate_number(i,j):
    out_num = []
    out_indices = []
    while lines[i][j].isnumeric():
        j -= 1
    j += 1
    while lines[i][j].isnumeric():
        out_num.append(lines[i][j])
        out_indices.append((i,j))
        j += 1
    return int("".join(out_num)), out_indices

out = 0
for i in range(1,len(lines)-1):
    for j in range(1,len(lines[0])-1):
        if lines[i][j] == "*":
            numbers = []
            numbers_indices = []
            for di, dj in directions:
                if (i+di,j+dj) not in numbers_indices and lines[i+di][j+dj].isnumeric():
                    res = enumerate_number(i+di,j+dj)
                    numbers.append(res[0])
                    numbers_indices.extend(res[1])
            if len(numbers) == 2:
                out += numbers[0] * numbers[1]
print(out)


