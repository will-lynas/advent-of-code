with open("input") as f:
    blocks = [list(map(list, block.split("\n"))) for block in f.read().strip().split("\n\n")]

def find_row(block):
    out = []
    for i in range(1, len(block[0])):
        if all(all(line[i-j-1] == line[i+j] for line in block) for j in range(min(i, len(block[0])-i))):
            out.append(i)
    return out

def find_line(block):
    out = []
    out.extend(find_row(block))
    block_trans = list(map(list, zip(*block)))
    out.extend(map(lambda x: x*100, find_row(block_trans)))
    return out

def op(c):
    if c == "#":
        return "."
    return "#"

out = 0
for block in blocks:
    original = find_line(block)[0]
    print("\n".join("".join(line) for line in block))
    print(original)
    print()
    found = False
    for i in range(len(block)):
        if found:
            break
        for j in range(len(block[0])):
            prev = block[i][j]
            block[i][j] = op(prev)
            new = find_line(block)
            if original in new:
                new.remove(original)
            if new:
                out += new[0]
                found = True
                break
            block[i][j] = prev
    assert found
print(out)
