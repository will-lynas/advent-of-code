from functools import cmp_to_key

with open("input") as f:
    raw_lines = f.read().strip().split("\n")
lines = []
for line in raw_lines:
    hand, bid = line.split()
    lines.append((hand, int(bid)))

cards = "AKQJT98765432"
def hand_score(hand):
    counts = [hand.count(c) for c in cards if hand.count(c)]
    # 5 of a kind
    if len(counts) == 1:
        return 0
    if len(counts) == 2:
        # 4 of a kind
        if 4 in counts:
            return 1
        # Full house
        return 2
    if len(counts) == 3:
        # 3 of a kind
        if 3 in counts:
            return 3
        # two pair
        return 4
    # One pair
    if len(counts) == 4:
        return 5
    # High card
    return 6

def cmp(hand1, hand2):
    score1 = hand_score(hand1[0])
    score2 = hand_score(hand2[0])
    if score1 != score2:
        return score1-score2
    for c1,c2 in zip(hand1[0], hand2[0]):
        i1 = cards.index(c1)
        i2 = cards.index(c2)
        if i1 != i2:
            return i1-i2
    return 0

sorted_lines = list(reversed(sorted(lines, key=cmp_to_key(cmp))))
print(sum((i+1)*hand[1] for i,hand in enumerate(sorted_lines)))
