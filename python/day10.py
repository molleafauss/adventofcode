from advent import Solver


# https://adventofcode.com/2022/day/10

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

    def file_name(self):
        return "../inputs/day10-signal.txt"

    def test_data(self):
        return """addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"""
