from functools import cmp_to_key

from advent import Solver

# https://adventofcode.com/2022/day/13
# blessed be `eval`. Parsing the input would have been a nightmare
# Good thing I implemented basically a "cmp" function returning -1, 0, or 1 - the part 2 was a breeze to do.


class Solution(Solver):
    def __init__(self):
        self.right_order = 0
        self.lines = []
        self.pairs = 0

    def parse(self, line: str):
        if line != "":
            self.lines.append(eval(line))
            return

        if self.compare(self.lines[self.pairs * 2], self.lines[self.pairs * 2 + 1]) == 1:
            print(f"pairs {self.pairs} - right order")
            self.right_order += (self.pairs + 1)
        self.pairs += 1

    def compare(self, left, right):
        if type(left) != list or type(right) != list:
            raise ValueError(f"Invalid input: {left} <=> {right}")
        for i in range(len(left)):
            if i >= len(right):
                return -1
            lval = left[i]
            rval = right[i]
            if type(lval) == int and type(rval) == int:
                if lval == rval:
                    continue
                else:
                    return 1 if lval < rval else -1
            if type(lval) == int:
                lval = [lval]
            if type(rval) == int:
                rval = [rval]
            # they both should be lists here
            assert type(lval) == list and type(rval) == list
            cmp = self.compare(lval, rval)
            if cmp == 0:
                continue
            else:
                return cmp
        # here just compare list length
        return 0 if len(left) == len(right) else 1

    def solve(self):
        print(f"[1] Found right order: {self.right_order}")

        # part 2
        decoder_key = 1
        divider_packets = 0
        self.lines.append([[2]])
        self.lines.append([[6]])
        packets = sorted(self.lines, key=cmp_to_key(self.compare), reverse=True)
        for i in range(len(packets)):
            if self.compare(packets[i], [[2]]) == 0 or self.compare(packets[i], [[6]]) == 0:
                divider_packets += 1
                decoder_key *= (i + 1)
        assert divider_packets == 2
        print(f"[2] decoder key: {decoder_key}")
