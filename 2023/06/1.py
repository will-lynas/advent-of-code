with open("input") as f:
    lines = f.read().strip().split("\n")
times = [int(n) for n in lines[0].split()[1:]]
distances = [int(n) for n in lines[1].split()[1:]]

out = 1
for time, distance in zip(times, distances):
    count = 0
    for i in range(time):
        if i*(time-i)>distance:
            count += 1
    out *= count
print(out)
