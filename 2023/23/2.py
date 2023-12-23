with open("example") as f:
    lines = list(map(list, f.read().strip().split("\n")))

class Node:
    def __init__(self):
        self.links = []

directions = ((0, 1), (0, -1), (-1, 0), (1, 0))
exit_x, exit_y = len(lines) - 1, len(lines[0]) - 2

def valid_directions(x, y):
    out = []
    for dx, dy in directions:
        tx = x + dx
        ty = y + dy
        if (0 <= tx < len(lines) and
            0 <= ty < len(lines[0]) and
            lines[tx][ty] != "#"):
            out.append((tx, ty))
    return out

def dfs(x, y, parent):
    lines[x][y] = "#"
    cur_directions = valid_directions(x, y)
    while len(cur_directions) == 1:
        x, y = cur_directions[0]
        path.append((x, y))
        lines[x][y] = "O"
        cur_directions = valid_directions(x, y)

    if len(cur_directions) == 0:
        # print_lines_coloured()
        clean_path(path)
        if (x, y) == (exit_x, exit_y):
            return path
        return []

    results = []
    for tx,ty in cur_directions:
        result = dfs(tx, ty)
        if result:
            results.append(result)

    clean_path(path)

    if not results:
        return []
    return max(results, key=len) + path


nodes = {}
x, y = 0, 1
nodes[(x, y)] = Node()
path = dfs(x, y)
print(len(path)-1)
