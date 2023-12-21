from collections import defaultdict, deque
import copy
from pprint import pprint

with open("example") as f:
    part1, part2 = f.read().strip().split("\n\n")

routines = defaultdict(list)
for line in part1.split("\n"):
    name, instructions = line[:-1].split('{')
    '}' # My lsp doesn't understand brackets inside strings
    routines[name] = instructions.split(",")

parts = []
for part in part2.split("\n"):
    parts.append([int(stat.split("=")[1])
                 for stat in part[1:-1].split(",")])

q = deque()
part = {
        k: {
            "min": 0,
            "max": 4000
            } for k in "xmas"
        }
q.append((part, "in", 0))
accepted = []

def handle_outcome(part, outcome):
    if outcome == "A":
        accepted.append(part)
    elif outcome == "R":
        pass
    else:
        q.append((part, outcome, 0))

def check_range(d):
    return d["min"] <= d["max"]

while q:
    part, routine, index = q.popleft()
    for instruction in routines[routine][index:]:
        if ":" not in instruction:
            handle_outcome(part, instruction)
        else:
            condition, outcome = instruction.split(":")
            letter = condition[0]
            sign = condition[1]
            value = int(condition[2:])
            if condition == "<":
                if part[letter]["max"] < value:
                    handle_outcome(part, outcome)
                else:
                    part_copy = copy.deepcopy(part)
                    part_copy[letter]["max"] = value - 1
                    if check_range(part_copy[letter]):
                        handle_outcome(part_copy, outcome)
                    part[letter]["min"] = value
                    q.append((part, routine, index+1))
            else:
                if part[letter]["min"] > value:
                    handle_outcome(part, outcome)
                else:
                    part_copy = copy.deepcopy(part)
                    part_copy[letter]["min"] = value + 1
                    handle_outcome(part_copy, outcome)
                    part[letter]["max"] = value
                    q.append((part, routine, index+1))
pprint(accepted)
