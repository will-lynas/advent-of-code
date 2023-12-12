from itertools import groupby

with open("input") as f:
    raw_lines = [line.split(" ") for line in f.read().strip().split("\n")]
lines = []
for line in raw_lines:
    nums = list(map(int, line[1].split(",")))
    lines.append((line[0], nums))

def get_counts(s):
    return [sum(1 for _ in group) for label, group in groupby(s) if label == "#"]

def dfs(s, c):
    if "?" not in s:
        if get_counts(s) == c:
            return 1
        return 0
    i = s.index("?")
    return dfs(s[:i] + "#" + s[i+1:], c) + dfs(s[:i] + "." + s[i+1:], c)

print(sum(dfs(line[0], line[1]) for line in lines))
