import math

with open("input") as f:
    lines = f.read().strip().split("\n")
time = int("".join(lines[0].split()[1:]))
distance = int("".join(lines[1].split()[1:]))

# -i^2 + i*time - distance > 0
right_root = -1/2 * (-time - math.sqrt(time**2 - 4*distance))
left_root = -1/2 * (-time + math.sqrt(time**2 - 4*distance))
print(math.floor(right_root)-math.ceil(left_root)+1)
