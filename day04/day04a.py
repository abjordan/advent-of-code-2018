#!/usr/bin/env python

from collections import defaultdict

import numpy as np

import json
import re
import sys

time_finder = re.compile(r'\[(\d\d\d\d-\d\d-\d\d) \d\d:(\d\d)\]')
guard_finder = re.compile(r'\] Guard (\#\d+) ')

lines = None
with open(sys.argv[1], 'r') as infile:
    lines = sorted(infile.readlines())

guard = None
nap_start = None
nap_stop = None
naps = {}
for line in lines:
    if "Guard" in line:
        m = guard_finder.search(line)
        guard = m.group(1)
        if not guard in naps:
            naps[guard] = defaultdict(list)
    elif 'falls asleep' in line:
        m = time_finder.search(line)
        nap_start = int(m.group(2))
    elif 'wakes up' in line:
        m = time_finder.search(line)
        day = m.group(1)
        nap_stop = int(m.group(2))
        naps[guard][day].append((nap_start, nap_stop))

minutes_asleep = defaultdict(int)
nap_distribution = {}
for guard in naps.keys():
    nap_distribution[guard] = np.zeros(60, dtype=int)
    for day in naps[guard]:
        for nap in naps[guard][day]:
            duration = nap[1] - nap[0]
            minutes_asleep[guard] += duration
            for i in range(nap[0], nap[1]):
                nap_distribution[guard][i] += 1

sleepiest = None
for g in sorted(minutes_asleep, key=minutes_asleep.get, reverse=True):
    if sleepiest is None:
        sleepiest = g
    #print g, minutes_asleep[g]
    break

print "-------------------------"
print "Strategy 1:"
print "Sleepiest:", sleepiest
print "Sleepiest at:", np.argmax(nap_distribution[sleepiest])
print "Key: ", int(sleepiest[1:]) * np.argmax(nap_distribution[sleepiest])

maximum = -1
max_guard = None
max_minute = None
for guard in nap_distribution.keys():
    max_sleep = np.max(nap_distribution[guard])
    argmax = np.argmax(nap_distribution[guard])
    if max_sleep > maximum:
        maximum = max_sleep
        max_guard = guard
        max_minute = argmax

print "-------------------------"
print "Strategy 2:"
print "Sleeper:", max_guard
print "Minute :", max_minute
print "Key    :", int(max_guard[1:]) * max_minute
print "-------------------------"
