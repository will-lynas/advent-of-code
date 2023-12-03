with open("input") as f:
    lines = f.read().strip().split("\n")
vowels = "aeiou"
doubles = [l*2 for l in "abcdefghijklmnopqrstuvwxyz"]
bads = ["ab", "cd", "pq", "xy"]
out = 0
for line in lines:
    if sum(line.count(vowel) for vowel in vowels) < 3:
        continue
    if not any(double in line for double in doubles):
        continue
    if any(bad in line for bad in bads):
        continue
    out += 1
print(out)
