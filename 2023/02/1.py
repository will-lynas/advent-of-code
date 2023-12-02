file_name = "input"
with open(file_name) as f:
    lines = [[[colour.strip().split(" ")
               for colour in
               group.strip().split(",")]
              for group in
              line.split(":")[1].strip().split(";")]
             for line in
             f.read().strip().split("\n")]

wanted = {"red": 12, "blue": 14, "green": 13}
out = 0
for i,game in enumerate(lines):
    possible = True
    for group in game:
        if not possible:
            break
        for colour in group:
            if int(colour[0]) > wanted[colour[1]]:
                possible = False
                break
    if possible:

        out += i+1
print(out)
