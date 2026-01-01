import logging

from advent import Solver

# https://adventofcode.com/2022/day/3
# the first half of the line was put in a set only for helping with the second part of the puzzle where set.intersection
# was practically the solution to the puzzle.
# The first part was probably fine using "letter in line"; with the size of inputs, lookup in a set vs going through
# a sequence would have no perceivable difference in performance.
#
# score formulas for the letters according to the puzzle.
# uppercase letter: ord(ch) - 'A' + 27
UPPER = 27 - ord('A')
# uppercase letter: ord(ch) - 'a' + 1
LOWER = 1 - ord('a')

log = logging.getLogger("day.03")


class Solution(Solver):
    def __init__(self):
        self.count = 0
        self.count2 = 0
        self.possible_badges = None
        self.row = 0

    def parse(self, line):
        self.row += 1
        # assuming this is even
        mid = int(len(line) / 2)
        first_half_items = set(line[0:mid])
        failed_item = None
        for c in line[mid:]:
            if c in first_half_items:
                failed_item = c
                break
        self.count += ord(failed_item) + (LOWER if failed_item.islower() else UPPER)
        # possible badges in every triplet
        if self.row % 3 == 1:
            self.possible_badges = set(line)
        else:
            self.possible_badges.intersection_update(set(line))
        if self.row % 3 == 0:
            assert len(self.possible_badges) == 1
            badge = self.possible_badges.pop()
            self.count2 += ord(badge) + (LOWER if badge.islower() else UPPER)

    def solve(self):
        log.info(f"[1] Priority of item in both compartments {self.count}")
        log.info(f"[2] Overall priority of badges {self.count2}")
        return str(self.count), str(self.count2)

