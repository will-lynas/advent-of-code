with open("input.txt") as f:
    lines = f.read().strip().split("\n")

nums = {"one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6, "seven": 7, "eight": 8, "nine": 9}

out = 0
for line in lines:
    indices = {}
    for num in nums:
        base = 0
        while base < len(line):
            if (i := line[base:].find(num)) >= 0:
                indices[base+i] = int(nums[num])
                base += i + 1
            else:
                break
    for i,c in enumerate(line):
        if c.isnumeric():
            indices[i] = int(c)
    left = indices[min(indices)]
    right = indices[max(indices)]
    s = left * 10 + right
    out += s
print(out)
