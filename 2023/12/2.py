from functools import cache

with open("input") as f:
    raw_lines = [line.split(" ") for line in f.read().strip().split("\n")]
lines = []
for line in raw_lines:
    big = "?".join([line[0]]*5)
    parts = tuple(filter(None, big.split(".")))
    nums = tuple(map(int, line[1].split(","))) * 5
    lines.append((parts, nums))

def debugger(func):
    def inner(*args, **kwargs):
        print("|---"*kwargs["depth"], func.__name__, args, "-->")
        out = func(*args, **kwargs)
        print("|---"*kwargs["depth"], func.__name__, args, out)
        return out
    return inner

@cache
@debugger
def ways_to_consume(s: str, nums: list[int], depth):
    if not nums:
        return 1
    count = 0
    for i in range(len(s)-nums[0]+1):
        if i > 0 and s[i-1] == "#":
            continue
        if (next_start := i + nums[0] + 1) - 1 < len(s) and s[next_start-1] == "#":
            continue
        trial = s[next_start:]
        count += ways_to_consume(trial, nums[1:], depth=depth+1)
    return count

@cache
@debugger
def big_consumer(parts, nums, depth):
    if not nums:
        return 1
    if not parts:
        return 0
    count = 0
    for i in range(len(nums)+1):
        ways = ways_to_consume(parts[0], nums[:i], depth=depth+1)
        if ways == 0:
            break
        count += ways * big_consumer(parts[1:], nums[i:], depth=depth+1)
    return count


# print(big_consumer(['?#???#?', '#??', '#', '?#'], [4, 1, 1, 1, 1, 1], depth=0))
for parts, nums in lines:
    print(parts, nums)
    print(big_consumer(parts, nums, depth=0))

print(sum(big_consumer(parts, nums, depth=0) for parts, nums in lines))


# @cache
# def get_middle(s):
#     n = len(s)//2
#     indices = [i for i, x in enumerate(s) if x == "?"]
#     return min(indices, key=lambda x: abs(x-n))
#
# @cache
# def possible(s, c):
#     if not s:
#         return [[]]
#     if "?" not in s:
#         return [[len(s)]]
#     i = get_middle(s)
#     # Case 1: this is #
#     c1 = possible(s[:i] + "#" + s[i+1:])
#     # Case 2: this is .
#     c2_1 = possible(s[:i])
#     c2_2 = possible(s[i+1:])
#     return c1 + list(map(lambda x: x[0]+x[1], product(c2_1, c2_2)))
#
# for line in lines:
#     for part in line[0]:
#         print(part)
#         if len(part) > 60:
#             count += 1
#             continue
#         print(len(possible(part)))
#
# print(count)
#
#
#
#
#
# def check_valid_so_far(s, c):
#     i = s.index("?")
#     return all(a == b for a,b in zip(c, get_counts(s[:i])[:-1])) # Don't include the last group, because it will be incomplete
#
# def dfs(s, c):
#     if "?" not in s:
#         if get_counts(s) == c:
#             return 1
#         return 0
#     if not check_valid_so_far(s, c):
#         return 0
#     i = s.index("?")
#     return dfs(s[:i] + "#" + s[i+1:], c) + dfs(s[:i] + "." + s[i+1:], c)
#
# count = 0
# for line in lines:
#     res = dfs(line[0], line[1])
#     print(line, res)
