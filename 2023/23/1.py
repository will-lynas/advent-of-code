import sys
sys.setrecursionlimit(10000)

with open("input") as f:
    lines = f.read().strip().split("\n")

visited = [[False for _ in range(len(lines[0]))] for _ in range(len(lines))]
directions = {
    ">": [(0, 1)],
    "<": [(0, -1)],
    "^": [(-1, 0)],
    "v": [(1, 0)],
    ".": [(0, 1), (0, -1), (-1, 0), (1, 0)],
    "#": []
    }
exit_x, exit_y = len(lines) - 1, len(lines[0]) - 2

def dfs(x, y):
    if (x, y) == (exit_x, exit_y):
        return 0, True
    visited[x][y] = True
    results = []
    for dx, dy in directions[lines[x][y]]:
        if (0 <= x + dx < len(lines) and
            0 <= y + dy < len(lines[0]) and
            not visited[x + dx][y + dy]):
            result, end = dfs(x + dx, y + dy)
            if end:
                results.append(result)
    visited[x][y] = False
    if not results:
        return 0, False
    return max(results) + 1, True

x = 0
y = 1
print(dfs(x, y)[0])
