# WARNING: very very inefficient. 30s runtime.

with open("input") as f:
    grid = list(map(list, f.read().strip().split("\n")))

def tilt_grid():
    for k in range(1, len(grid)):
        for i in reversed(range(1, k+1)):
            for j in range(len(grid[0])):
                if grid[i-1][j] == "." and grid[i][j] == "O":
                    grid[i-1][j] = "O"
                    grid[i][j] = "."
def cycle():
    global grid
    for _ in range(4):
        tilt_grid()
        grid = list(map(list, zip(*grid[::-1])))

def get_load():
    return sum(row.count("O")*(len(grid)-i) for i, row in enumerate(grid))

def get_s():
    return "".join("".join(line) for line in grid)

cache = []
front = 0
while True:
    cycle()
    s = get_s()
    load = get_load()
    if (s,load) in cache:
        while not cache[0] == (s,load):
            front += 1
            cache.pop(0)
        break
    cache.append((s,load))

print(cache[(1000000000-front-1)%len(cache)][1])
