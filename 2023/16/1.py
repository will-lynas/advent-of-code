from enum import Enum
from collections import deque

with open("input") as f:
    lines = f.read().strip().split("\n")

class Direction(Enum):
    LEFT = (0, -1)
    RIGHT = (0, 1)
    UP = (-1, 0)
    DOWN = (1, 0)

direction_map = {
        ".": {},
        "-": {
            Direction.UP: [Direction.LEFT, Direction.RIGHT],
            Direction.DOWN: [Direction.LEFT, Direction.RIGHT]
            },
        "|": {
            Direction.LEFT: [Direction.UP, Direction.DOWN],
            Direction.RIGHT: [Direction.UP, Direction.DOWN]
            },
        "\\": {
            Direction.RIGHT: [Direction.DOWN],
            Direction.LEFT: [Direction.UP],
            Direction.DOWN: [Direction.RIGHT],
            Direction.UP: [Direction.LEFT]
            },
        "/": {
            Direction.RIGHT: [Direction.UP],
            Direction.LEFT: [Direction.DOWN],
            Direction.DOWN: [Direction.LEFT],
            Direction.UP: [Direction.RIGHT]
            }
        }

grid = [[[] for _2 in lines[0]] for _ in lines]
stack = deque()
stack.append((0, 0, Direction.RIGHT))

while stack:
    x, y, direction = stack.popleft()
    if (not 0 <= y < len(lines[0]) or
        not 0 <= x < len(lines) or
        direction in grid[x][y]):
        continue
    grid[x][y].append(direction)
    next_map = direction_map[lines[x][y]]
    next_directions = next_map[direction] if direction in next_map else [direction]
    for next_direction in next_directions:
        dx, dy = next_direction.value
        stack.append((x + dx, y + dy, next_direction))

# print("\n".join(
#     "".join(
#         "#" if c else "." for c in line)
#     for line in grid))
print(sum(sum(1 for c in line if c) for line in grid))
