with open("input") as f:
    grid = list(map(list, f.read().strip().split("\n")))

for k in range(1, len(grid)):
    for i in reversed(range(1, k+1)):
        for j in range(len(grid[0])):
            if grid[i-1][j] == "." and grid[i][j] == "O":
                grid[i-1][j] = "O"
                grid[i][j] = "."

print(sum(row.count("O")*(len(grid)-i) for i, row in enumerate(grid)))
