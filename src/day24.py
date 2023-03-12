from dataclasses import dataclass

from advent import Solver
from grid import GridPos, dir_from_char, DIR_N, DIR_E, DIR_S, DIR_W, char_from_dir


# https://adventofcode.com/2022/day/24
# once the first was done, it was a matter of refactoring it to do the back and forth

@dataclass
class Blizzard:
    pos: GridPos
    dir: GridPos


def move_wrap(val, move, bounds):
    val += move
    w = bounds[1] - bounds[0] + 1
    while val < bounds[0]:
        val += w
    while val > bounds[1]:
        val -= w
    return val


class Solution(Solver):
    def __init__(self):
        self.blizzards = []
        self.height = 0
        self.width = 0
        self.entry = GridPos(0, 0)
        self.exit = GridPos(0, 0)
        self.bliz_time = []

    def parse(self, line: str):
        if self.height == 0:
            self.width = len(line)
            self.entry = GridPos(0, line.find("."))
            assert 0 < self.entry.col < self.width
        for col in range(self.width):
            if line[col] != '.' and line[col] != '#':
                self.blizzards.append(Blizzard(GridPos(self.height, col), dir_from_char(line[col])))
        # get first available space in current line - will be the exit on last line
        self.exit = GridPos(self.height, line.find("."))
        self.height += 1

    def solve(self):
        print(f"Tracing path from {self.entry} => {self.exit}")
        self.blizzards_at_time(0)
        t = self.find_path(self.entry, self.exit, 0)
        print(f"[1] Found exit in: {t}")
        t = self.find_path(self.exit, self.entry, t)
        t = self.find_path(self.entry, self.exit, t)
        print(f"[2] Total time: {t}")

    def find_path(self, entry, exit, t):
        exit_reached = False
        steps = [(entry, t)]
        visited = set()
        while not exit_reached:
            pos, t = steps.pop(0)
            if pos == exit:
                return t
            if (pos, t) in visited:
                continue
            visited.add((pos, t))
            blizzards = self.blizzards_at_time(t + 1)
            # try all direction, or stay
            for dir in [DIR_N, DIR_E, DIR_S, DIR_W]:
                new_pos = pos + dir
                if new_pos in blizzards:
                    continue
                valid = new_pos == self.entry or new_pos == self.exit
                valid |= (0 < new_pos.row < self.height - 1) and (0 < new_pos.col < self.width - 1)
                if not valid:
                    continue
                # print(f"{pos}, {t} -> {new_pos}")
                steps.append([new_pos, t + 1])
            if pos not in blizzards:
                steps.append([pos, t + 1])
                # print(f"{pos}, {t} -> {pos} [wait]")

    def blizzards_at_time(self, t):
        if t < len(self.bliz_time):
            return self.bliz_time[t]
        assert t == len(self.bliz_time)
        bliz_pos = set()
        for b in self.blizzards:
            row = move_wrap(b.pos.row, (b.dir.row * t), [1, self.height - 2])
            col = move_wrap(b.pos.col, (b.dir.col * t), [1, self.width - 2])
            bliz_pos.add(GridPos(row, col))
        self.bliz_time.append(bliz_pos)
        return bliz_pos

    def print_blizzards(self, t):
        blizzards = self.blizzards_at_time(t)
        grid = [['.'] * self.width for _ in range(self.height)]
        for b in blizzards:
            rc = b[0]
            grid[rc.row][rc.col] = char_from_dir(b[1])
        for row in grid:
            print("".join(row))

    def file_name(self):
        return "../files/day24-blizzards.txt"

    def test_data(self):
        return """#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"""


if __name__ == '__main__':
    d = Solution()
    for l in d.test_data().split("\n"):
        d.parse(l.rstrip())

    for t in range(10):
        print(f"Time {t}")
        d.print_blizzards(t)
