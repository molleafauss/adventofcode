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


def dir_from_char(ch):
    match ch:
        case '>':
            return DIR_E
        case '<':
            return DIR_W
        case '^':
            return DIR_N
        case 'v':
            return DIR_S
        case _:
            raise ValueError("Unsupported direction for: " + ch)


def char_from_dir(dir):
    if dir == DIR_E:
        return '>'
    elif dir == DIR_W:
        return '<'
    elif dir == DIR_N:
        return '^'
    elif dir == DIR_S:
        return 'v'
    raise ValueError(f"Unsupported direction: {dir}")
