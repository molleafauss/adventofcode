from advent import Solver

# https://adventofcode.com/2022/day/14
# This was reasonably easy, no important optimizations needed even on the second part.
# I've used a dict, indexed by (x, y) of each spot to hold walls and resting sand and with the size of the input it was
# enough.

START = (500, 0)


class Solution(Solver):
    def __init__(self):
        self.scan = {}
        self.max_y = 0

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
        sand = 0
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
                    # print(f"Sand found resting place in {(x, y)}: {sand}")
                    sand += 1
                    self.scan[(x, y)] = 'o'
                    break
            # reached the abyss - stop
            keep_dripping = y <= self.max_y
        print(f"[1] Sand resting: {sand}")

        # remove all 'o' from the scan
        self.scan = {pos: val for pos, val in self.scan.items() if val == '#'}

        # part 2 - should be trying to refactor
        self.max_y += 2
        keep_dripping = True
        sand = 0
        while keep_dripping:
            x, y = START
            while y <= self.max_y - 1:
                if (x, y + 1) not in self.scan and y + 1 < self.max_y:
                    y += 1
                    continue
                elif (x - 1, y + 1) not in self.scan and y + 1 < self.max_y:
                    y += 1
                    x -= 1
                    continue
                elif (x + 1, y + 1) not in self.scan and y + 1 < self.max_y:
                    y += 1
                    x += 1
                    continue
                elif y + 1 == self.max_y:
                    # print(f"Sand found resting place in {(x, y)}: {sand}")
                    sand += 1
                    self.scan[(x, y)] = 'o'
                    break
                else:
                    # print(f"Sand found resting place in {(x, y)}: {sand}")
                    sand += 1
                    self.scan[(x, y)] = 'o'
                    break
            # if obstructs the start, will block
            keep_dripping = START not in self.scan

        print(f"[2] Sand resting: {sand}")
