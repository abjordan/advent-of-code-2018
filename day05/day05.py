#!/usr/bin/env python

import sys
		
def reacts(s1, s2):
	if s1.upper() == s2.upper():
		if s1 != s2:
			return True
		else:
			return False
	else:
		return False

def reduce(text):
	while True:
		new_text = None
		for i in range(1, len(text)):
			if reacts(text[i-1], text[i]):
				if (i-1) <= 0:
					if (i+1) >= len(text):
						new_text = ""
					else:
						new_text = text[i+1:]
					break
				elif (i+1) >= len(text):
					#print text[0:i-1], "**", text[i-1], text[i], "**"
					new_text = text[0:i-1]
					break
				else:
					#print "!!!"
					#print text[0:i-1], "**", text[i-1], text[i], "**", text[i+1:len(text)]
					new_text = text[0:i-1] + text[i+1:len(text)]
					break
		if new_text is not None:
			text = new_text
		else:
			break

	return len(text)

with open(sys.argv[1], 'r') as infile:
	orig_text = infile.read()

part_one = reduce(orig_text)
print "Naive reduction (part one):", part_one

# Hold mah beer...

tokens = set()
[tokens.add(s) for s in orig_text.upper()]
all_reductions = {}
shortest = len(orig_text)
for token in sorted(tokens):
	tmp_text = [s for s in orig_text if s.upper() != token]
	r = reduce(tmp_text)
	if r < shortest:
		shortest = r 
	print token, '-->', r
	all_reductions[token] = r

print "---"
print "Part Two:", shortest