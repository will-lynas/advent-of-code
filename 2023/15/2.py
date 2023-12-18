with open("input") as f:
    parts = f.read().strip().split(",")

def hash(s):
    cur = 0
    for c in s:
        cur += ord(c)
        cur *= 17
        cur %= 256
    return cur

boxes = [[] for _ in range(256)]
for part in parts:
    if part[-1] == "-":
        label = part[:-1]
        box = boxes[hash(label)]
        for i, lens in enumerate(box):
            if lens[0] == label:
                box.pop(i)
                break
    else:
        label, n = part.split("=")
        box = boxes[hash(label)]
        found = False
        for i, lens in enumerate(box):
            if lens[0] == label:
                found = True
                lens[1] = n
                break
        if not found:
            box.append([label, n])

count = 0
for i, box in enumerate(boxes):
    for j, lens in enumerate(box):
        count += (i+1) * (j+1) * int(lens[1])
print(count)
