from advent import Solver

# https://adventofcode.com/2022/day/21


def parse_path(text):
    path = []
    count = 0
    for ch in text:
        if ch.isdigit():
            count *= 10
            count += int(ch)
        else:
            path.append([count, ch])
            count = 0
    path.append([count, None])
    return path


class Solution(Solver):
    def __init__(self):
        self.map = []
        self.path = None
        self.is_map = True

    def parse(self, line: str):
        if not line:
            self.is_map = False
            return
        if self.is_map:
            self.map.append(line)
        else:
            self.path = parse_path(line)

    def solve(self):
        print(f"{self.path}")
        pos = [0, self.map[0].find("."), 0]
        print(f"Starting position: {pos}")
        for walk, turn in self.path:
            pos = self.walk(pos, walk)
            pos = self.turn(pos, turn)

        password = (pos[0] + 1) * 1000 + (pos[1] + 1) * 4 + pos[2]
        print(f"[1] final position: {pos} => password {password}")

    def walk(self, pos, walk):
        w = walk
        # rows are ordered top->bottom: v moves down = rows + 1; ^ moves up = rows -1
        while walk > 0:
            if pos[2] == 0 or pos[2] == 2:
                nr, nc = self.move_col(pos[0], pos[1], pos[2])
            elif pos[2] == 1 or pos[2] == 3:
                nr, nc = self.move_row(pos[0], pos[1], pos[2])
            if self.map[nr][nc] == '#':
                # stop
                break
            assert self.map[nr][nc] == '.'
            walk -= 1
            pos[0] = nr
            pos[1] = nc
        assert self.map[pos[0]][pos[1]] == '.'
        print(f"Walk {w} => {pos}")
        return pos

    def in_map(self, r, c):
        if r < 0 or r >= len(self.map):
            return False
        if c < 0 or c >= len(self.map[r]):
            return False
        if self.map[r][c] == ' ':
            return False
        return True

    def move_col(self, r, c, dir):
        dc = 1 if dir == 0 else -1
        c += dc
        row_len = len(self.map[r])
        while not self.in_map(r, c):
            if c < 0:
                c = row_len - 1
            elif c >= row_len:
                c = 0
            elif self.map[r][c] == ' ':
                c += dc
        return r, c

    def move_row(self, r, c, dir):
        dr = 1 if dir == 1 else -1
        r += dr
        while not self.in_map(r, c):
            if r < 0:
                r = len(self.map) - 1
            elif r >= len(self.map):
                r = 0
            elif c >= len(self.map[r]):
                # keep moving here until we find a row with compatible length
                r += dr
            elif self.map[r][c] == ' ':
                r += dr
        return r, c

    def turn(self, pos, turn):
        facing = pos[2]
        match turn:
            case 'R':
                facing += 1
            case 'L':
                facing -= 1
            case None:
                return pos
            case _:
                raise ValueError(f"Invalid turn: {turn}")
        if facing < 0:
            facing = 3
        elif facing > 3:
            facing = 0
        pos[2] = facing
        print(f"Turn {turn} => {pos}")
        return pos

    def file_name(self):
        return "../files/day22-path.txt"

    def test_data(self):
        return """        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
"""