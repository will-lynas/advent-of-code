#!/usr/bin/env python2

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
		found = False
		for k in range(10):
			d = str(k)
			for j in range(len(n)-1):
				if j == 0:
					if n[j] == d and n[j+1] == d and n[j+2] != d:
						found = True
						break
				elif j == len(n)-2:
					if n[j-1] != d and n[j] == d and n[j+1] == d:
						found = True
						break
				else:
					if n[j-1] != d and n[j] == d and n[j+1] == d and n[j+2] != d:
						found = True
						break
			if found:
				counter += 1
				break

print counter
