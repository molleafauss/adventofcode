import logging

from adventofcode import Solver


# https://adventofcode.com/2022/day/6
# This was actually "simple" once one realises that flinging a slice into a set would create a set of the expected
# length only if all letters were different.
# one could actually use a single dictionary containing frequency of letters and add/remove as the sliding window moves
# but the code would end up being more involved.
# the `solve()` is an empty method here as the input is a single line

log = logging.getLogger("day.06")

def all_different(string):
    return len(set(string)) == len(string)


class Solution(Solver):
    def __init__(self):
        self.start_of_packet = None
        self.start_of_message = None

    def parse(self, line: str):
        start_of_packet = False
        start_of_message = False
        line = line.strip()
        i = 0
        while i + 14 <= len(line):
            # start_of_packet : 4 chars
            if not self.start_of_packet and all_different(line[i:i+4]):
                self.start_of_packet = i + 4
                log.info(f"Found start-of-packet at {self.start_of_packet}")
            if not self.start_of_message and all_different(line[i:i+14]):
                self.start_of_message = i + 14
                log.info(f"Found start-of-message at {self.start_of_message}")
            if start_of_packet and start_of_message:
                return
            i += 1

    def solve(self):
        return str(self.start_of_packet), str(self.start_of_message)
