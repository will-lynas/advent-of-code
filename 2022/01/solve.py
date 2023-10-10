#!/usr/bin/env python3
with open("input.txt") as f:
    lines = [line.strip() for line in f.readlines()]

elves = []
current_elf = 0
for line in lines:
    if line == "":
        elves.append(current_elf)
        current_elf = 0
    else:
        current_elf += int(line)

def part1():
    print(max(elves))

def part2():
    print(sum(sorted(elves)[-3:]))

part1()
part2()
