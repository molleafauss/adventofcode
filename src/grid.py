from dataclasses import dataclass


@dataclass(frozen=True)
class GridPos:
    row: int
    col: int

    def __add__(self, other):
        return GridPos(self.row + other.row, self.col + other.col)


DIR_N = GridPos(-1, 0)
DIR_NE = GridPos(-1, 1)
DIR_E = GridPos(0, 1)
DIR_SE = GridPos(1, 1)
DIR_S = GridPos(1, 0)
DIR_SW = GridPos(1, -1)
DIR_W = GridPos(0, -1)
DIR_NW = GridPos(-1, -1)