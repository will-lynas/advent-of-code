with open("input") as f:
    blocks = [block.split("\n") for block in f.read().strip().split("\n\n")]

count = 0
for block in blocks:
    found = False
    for i in range(1, len(block[0])):
        if all(all(line[i-j-1] == line[i+j] for line in block) for j in range(min(i, len(block[0])-i))):
            count += i
            found = True
            break
    if found:
        continue
    block_trans = list(map(list, zip(*block)))
    for i in range(1, len(block_trans[0])):
        if all(all(line[i-j-1] == line[i+j] for line in block_trans) for j in range(min(i, len(block_trans[0])-i))):
            count += 100*i
            break
print(count)
