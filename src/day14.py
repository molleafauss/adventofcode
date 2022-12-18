from adc import Solver

# https://adventofcode.com/2022/day/14

START = [500, 0]


class Solution(Solver):
    def __init__(self):
        self.scan = {}
        self.max_y = 0
        self.sand = 0

    def parse(self, line: str):
        if not line:
            return
        parts = [p for p in map(lambda x: [int(p) for p in x.split(",")], line.split(" -> "))]
        for i in range(len(parts) - 1):
            x0, y0 = parts[i]
            x1, y1 = parts[i + 1]
            print(f"Tracing line: ({x0}, {y0}) <=> ({x1}, {y1})")
            if x0 == x1:
                end = y1 + (1 if y1 > y0 else -1)
                step = 1 if y1 > y0 else -1
                for y in range(y0, end, step):
                    self.scan[(x0, y)] = '#'
            elif y0 == y1:
                end = x1 + (1 if x1 > x0 else -1)
                step = 1 if x1 > x0 else -1
                for x in range(x0, end, step):
                    self.scan[(x, y0)] = '#'
            else:
                raise ValueError(f"Invalid line: ({x0}, {y0}) <=> ({x1}, {y1})")
            self.max_y = max(self.max_y, y0, y1)

    def solve(self):
        print(f"Max y: {self.max_y}")
        keep_dripping = True
        while keep_dripping:
            x, y = START
            while y <= self.max_y:
                if (x, y + 1) not in self.scan:
                    y += 1
                    continue
                elif (x - 1, y + 1) not in self.scan:
                    y += 1
                    x -= 1
                    continue
                elif (x + 1, y + 1) not in self.scan:
                    y += 1
                    x += 1
                    continue
                else:
                    print(f"Sand found resting place in {(x, y)}: {self.sand}")
                    self.sand += 1
                    self.scan[(x, y)] = 'o'
                    break
            # reached the abyss - stop
            keep_dripping = y <= self.max_y
        print(f"[1] Sand resting: {self.sand}")

    def file_name(self):
        super().file_name()

    def test_data(self):
        return """498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"""
