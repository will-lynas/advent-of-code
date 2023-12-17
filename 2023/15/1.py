with open("input") as f:
    parts = f.read().strip().split(",")

def hash(s):
    cur = 0
    for c in s:
        cur += ord(c)
        cur *= 17
        cur %= 256
    return cur

print(sum(hash(part) for part in parts))
