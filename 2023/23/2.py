with open("input") as f:
    lines = [["#" if c == "#" else " " for c in line]
             for line in f.read().strip().split("\n")]

directions = ((0, 1), (0, -1), (-1, 0), (1, 0))
exit_x, exit_y = len(lines) - 1, len(lines[0]) - 2

def valid_directions(x, y):
    out = []
    for dx, dy in directions:
        tx = x + dx
        ty = y + dy
        if (0 <= tx < len(lines) and
            0 <= ty < len(lines[0]) and
            lines[tx][ty] == " "):
            out.append((tx, ty))
    return out

def clean_path(path):
    for x, y in path:
        lines[x][y] = " "

def print_lines_coloured():
    for line in lines:
        for c in line:
            if c == "O":
                print(f"\033[91m{c}\033[0m", end="")
                "]]" # lsp doesnt understand brackets
            else:
                print(c, end="")
        print()
    print()


def debug(f):
    def wrapper(*args, **kwargs):
        print(f"{f.__name__}({args}) -> ?")
        out = f(*args, **kwargs)
        print(f"{f.__name__}({args}) -> {len(out)}")
        return out
    return wrapper

#@debug
def dfs(x, y):
    path = [(x, y)]
    lines[x][y] = "O"
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


x, y = 0, 1
path = dfs(x, y)
print(len(path)-1)
