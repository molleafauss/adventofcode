from dataclasses import dataclass

from advent import Solver
from grid import GridPos, pos_from_char

# https://adventofcode.com/2022/day/24
# TODO - INCOMPLETE
# this is interesting - I've no idea how to solve it...


@dataclass
class Blizzard:
    pos: GridPos
    dir: GridPos


class Solution(Solver):
    def __init__(self):
        self.map = []
        self.blizzards = []
        self.height = 0
        self.width = 0

    def parse(self, line: str):
        if not self.width:
            self.width = len(line)
        row = list(line)
        for col in range(len(row)):
            if row[col] != '.' and row[col] != '#':
                self.blizzards.append(Blizzard(GridPos(self.height, col), pos_from_char(row[col])))
        self.map.append(row)
        self.height += 1

    def solve(self):
        e = [i for i in range(self.width) if self.map[0][i] == '.']
        assert len(e) == 1
        entry = GridPos(0, e[0])
        e = [i for i in range(self.width) if self.map[-1][i] == '.']
        assert len(e) == 1
        out = GridPos(self.height - 1, e[0])
        print(f"Tracing path from {entry} => {out}")

    def test_data(self):
        return """#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"""