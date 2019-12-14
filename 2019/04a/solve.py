#!/usr/bin/env python2

doubles = [str(i)*2 for i in range(10)]

counter = 0

for i in range(353096,843212):
	n = str(i)
	last = 0
	for d in n:
		if last <= int(d):
			last = int(d)
		else:
			break

	else:
		for d in doubles:
			if d in n:
				counter += 1
				break

print counter
