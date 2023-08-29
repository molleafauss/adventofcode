import logging

from advent import Solver


# https://adventofcode.com/2022/day/10

log = logging.getLogger("day.10")


CYCLES = [20, 60, 100, 140, 180, 220]


class Solution(Solver):
    def __init__(self):
        self.x = 1
        self.cycle = 0
        self.cpos = 0
        self.signal_strength = 0
        self.row = 0
        self.col = 0
        self.display =[[" " for _ in range(40)] for _ in range(6)]

    def parse(self, line: str):
        if line.startswith("noop"):
            self.draw()
            self.check_cycle(1)
            self.cycle += 1
        if line.startswith("addx"):
            self.draw()
            self.draw()
            self.check_cycle(2)
            self.x += int(line[5:])
            self.cycle += 2

    def check_cycle(self, ticks):
        if self.cpos >= len(CYCLES):
            return
        if self.cycle + ticks >= CYCLES[self.cpos]:
            s = self.x * CYCLES[self.cpos]
            self.signal_strength += s
            print(f"Signal strength at cycle {self.cycle}/{ticks}: {s} => {self.signal_strength}")
            self.cpos += 1

    def draw(self):
        draw = "#" if self.x - 1 <= self.col <= self.x + 1 else "."
        self.display[self.row][self.col] = draw
        self.col += 1
        if self.col >= 40:
            self.col = 0
            self.row += 1
        if self.row >= 6:
            self.show_display()
            self.row = 0

    def show_display(self):
        for l in self.display:
            print("".join(l))

    def solve(self):
        print(f"[1] Signal strength found: {self.signal_strength}")
        return str(self.signal_strength), ""
