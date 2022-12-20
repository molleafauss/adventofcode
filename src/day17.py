from dataclasses import dataclass

from advent import Solver

# https://adventofcode.com/2022/day/17
# being strict on coordinates: 0 <= x <= 7; y starts at 0 and grows "up"; pieces height and width are 1-based.
# piece x,y is the bottom-left spot
# so, for part 2 I had to find some hint - seems if I can find a cycle. A tuple made of:
# (wind index, piece falling, relative position to the top height of the columns in the chamber) is the simplest
# this allows us to magically find the final solution directly just with some math on what will happen at every
# rock that falls


# pieces blocks in rows from top to bottom
PIECES = [
    ["####"],
    [".#.", "###", ".#."],
    ["..#", "..#", "###"],
    ["#", "#", "#", "#"],
    ["##", "##"]
]
MAX_ROCKS_P1 = 2022
MAX_ROCKS_P2 = 1000000000000
WIDTH = 7


@dataclass
class Piece:
    shape: list[str]
    width: int
    height: int
    x: int
    y: int


class Solution(Solver):
    def __init__(self):
        self.winds = None
        self.height = 0
        self.chamber = []
        self.max_height = 0
        self.rocks = 0
        self.wind_pos = 0
        self.status = {}
        self.results = {
            2022: None,
            1000000000000: None
        }

    def parse(self, line: str):
        self.winds = line.strip()

    def solve(self):
        piece = 0
        while None in self.results.values():
            self.rocks += 1
            self.drop_piece(piece)
            piece += 1
            if piece >= len(PIECES):
                piece = 0
            self.detect_cycle(piece)
            if self.rocks in self.results and self.results[self.rocks] is None:
                self.results[self.rocks] = self.max_height
        print(f"[1] Chamber height: {self.results[MAX_ROCKS_P1]}")
        print(f"[2] Chamber height: {self.results[MAX_ROCKS_P2]}")

    def drop_piece(self, pidx):
        piece = Piece(PIECES[pidx], 0, 0, 2, 0)
        piece.width = max([len(row) for row in piece.shape])
        piece.height = len(piece.shape)
        # top-left corner
        piece.x = 2
        piece.y = self.max_height + 3
        resting = False
        while not resting:
            wind = self.winds[self.wind_pos]
            self.wind_pos += 1
            if self.wind_pos >= len(self.winds):
                self.wind_pos = 0
            # move left/right if possible
            dir = 0
            if wind == '<' and self.clear_x(piece, -1):
                dir = -1
            elif wind == '>' and self.clear_x(piece, 1):
                dir = 1
            piece.x += dir
            # print(f"** Move {wind}: {dir != 0}")
            # self.plot(piece)
            # move down - flag resting if it can't move down
            down = 0
            if self.clear_y(piece):
                down = -1
            piece.y += down
            resting = down == 0
            # print(f"** Move down: {down != 0}")

        # self.plot(piece)
        self.place(piece)

    def clear_x(self, piece, dir):
        if piece.x + dir < 0 or piece.x + piece.width + dir > 7:
            return False
        # check if the shape is over the resting rocks
        if piece.y > self.max_height:
            return True
        # check if inside the chamber the shape can move in the expected dir / look only one of the two shape edges
        h = 0
        while h < piece.height and piece.y + h < self.max_height:
            row = self.chamber[piece.y + h]
            blk = piece.shape[-1 - h]
            h += 1
            for x in range(piece.width):
                if blk[x] == '#' and row[piece.x + x + dir] == '#':
                    return False
        return True

    def clear_y(self, piece):
        if piece.y - 1 > self.max_height:
            return True
        if piece.y == 0:
            return False
        # bottom row of the piece is near the bottom row of the chamber - see if there's any '#' touching
        h = 0
        while h < piece.height and piece.y + h <= self.max_height:
            rocks_below = self.chamber[piece.y + h - 1]
            blk = piece.shape[-1 - h]
            for w in range(piece.width):
                if rocks_below[piece.x + w] == '#' and blk[w] == '#':
                    return False
            h += 1
        return True

    def place(self, piece):
        # place bottom-up
        h = 0
        while h < piece.height:
            # reverse lookup
            blk = piece.shape[-1 - h]
            r = piece.y + h
            h += 1
            if r >= self.max_height:
                self.max_height += 1
                self.chamber.append(["."] * 7)
            row = self.chamber[r]
            for c in range(piece.width):
                if blk[c] == '#':
                    row[piece.x + c] = '#'

    def plot(self, piece, output=False):
        print(f"** Rock {self.rocks}")
        y = max(piece.y + piece.height, len(self.chamber))
        while y >= 0:
            row = ["."] * 7 if y >= self.max_height else list(self.chamber[y])
            if piece.y <= y < piece.y + piece.height:
                blk = piece.shape[-1 - (y - piece.y)]
                for x in range(piece.width):
                    assert blk[x] == '.' or row[piece.x + x] == '.'
                    row[piece.x + x] = "@" if blk[x] == '#' else row[piece.x + x]
            if output:
                print("".join(row))
            y -= 1
        if output:
            print("=======")

    def detect_cycle(self, piece):
        column_status = [0] * WIDTH
        y = self.max_height
        while 0 in column_status and y > 0:
            y -= 1
            row = self.chamber[y]
            for i in range(WIDTH):
                if row[i] == '#' and column_status[i] == 0:
                    column_status[i] = self.max_height - y
        key = f"{self.wind_pos}, {piece}, {column_status}"
        if key not in self.status:
            self.status[key] = (self.rocks, self.max_height)
        else:
            old_rocks, old_height = self.status[key]
            cycle = self.rocks - old_rocks
            print(f"Found cycle: {old_rocks} => {self.rocks}: {key}")
            height_diff = self.max_height - old_height
            for target in self.results:
                num_cycles = (target - self.rocks) // cycle
                rocks = self.rocks + num_cycles * cycle
                max_height = self.max_height + num_cycles * height_diff
                if rocks < target:
                    delta = target - rocks
                    _, delta_height = next(filter(lambda val: val[0] == old_rocks + delta, self.status.values()))
                    rocks += delta
                    assert rocks == target
                    max_height += delta_height - old_height
                self.results[target] = max_height

    def test_data(self):
        return """>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"""

    def file_name(self):
        return "../files/day17-wind.txt"
