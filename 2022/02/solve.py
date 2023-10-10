#!/usr/bin/env python3
with open("input.txt") as f:
    lines = [line.strip() for line in f.readlines()]

# key beats value
win_map = {
        "A": "C",
        "B": "A",
        "C": "B"
        }

value_map = {
        "A": 1,
        "B": 2,
        "C": 3
        }

def p1():
    conversion_map = {
            "X": "A",
            "Y": "B",
            "Z": "C"
            }
    score = 0
    for line in lines:
        round_score = 0
        op, me = line.split(" ")
        me = conversion_map[me]
        round_score += value_map[me]
        if me == op:
            round_score += 3
        elif op == win_map[me]:
            round_score += 6
        score += round_score
    print(score)

def p2():
    options = {"A", "B", "C"}
    score = 0
    for line in lines:
        round_score = 0
        op, res = line.split(" ")
        if res == "X":
            # lose
            round_score += 0
            choice = win_map[op]
        elif res == "Y":
            # draw
            round_score += 3
            choice = op
        else:
            # win
            round_score += 6
            choice = list(options.difference({op, win_map[op]}))[0]
        round_score += value_map[choice]
        score += round_score
    print(score)


p1()
p2()
