#!/usr/bin/env python

import re
import sys

import numpy as np

r = re.compile(r'#(\d+?) @ (\d+?),(\d+?): (\d+?)x(\d*)')

claims = {}

board = np.zeros( (1500, 1500) )
with open(sys.argv[1], 'r') as f:
	text = f.readlines()
	for line in text:
		l = line.strip()
		m = r.match(l)

		if not m:
			print "WTF???"
			print line
			sys.exit(1)

		x = int(m.group(2))
		y = int(m.group(3))
		xs = int(m.group(4))
		ys = int(m.group(5))
		#print x, y, xs, ys
		claims[m.group(1)] = (x, y, xs, ys)

		for i in range(x, (x+xs)):
			for j in range(y, (y+ys)):
				board[i][j] += 1

count = 0	
for i in board.flat:
	if i > 1:
		count += 1

print "Count:", count

for c,v  in claims.iteritems():
	x, y, xs, ys = v

	if np.all(board[x:x+xs, y:y+ys] == 1):
		print "Non-overlapping:", c