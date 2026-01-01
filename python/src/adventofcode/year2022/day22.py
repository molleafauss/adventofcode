import logging
from dataclasses import dataclass

from adventofcode import Solver

# https://adventofcode.com/2022/day/22
# part 1 was approachable. Part 2 ... tough. Test input and real input had a different cube shape; I was unable to
# find a way to calculate faces, adjacency and resulting rotation between them all in code, so I added some
# pre-calculation to the input.
# Took me ages to get to the bottom of wrong results - most errors were in the end a bad data in the adjacency data I
# added.
# Also: this code is hacky af - I should have used some better structure for the various concepts involved (cube, faces,
# position, adjacency, ...)

log = logging.getLogger("day.22")


DIRS = [
    [0, 1],
    [1, 0],
    [0, -1],
    [-1, 0]
]
DIR_TEXT = ['>', 'v', '<', '^']


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


@dataclass
class Face:
    id: int
    row: int
    col: int
    size: int
    facing: list

    def contains(self, row, col):
        return self.row <= row <= self.row + self.size and self.col <= col <= self.col + self.size

    def relative(self, row, col):
        return row - self.row, col - self.col

    def cross(self, pos, dir, turn):
        # pos[0] is diff in rows, pos[1] is diff in columns - one should be not relevant
        match dir, turn:
            case 0, 'R':
                # crossing from the right to the top border
                dir = (dir + 1) % len(DIRS)
                return self.row, self.col + self.size - pos[0], dir
            case 0, 'L':
                # crossing from the right to the bottom border
                dir = (dir - 1) % len(DIRS)
                return self.row + self.size, self.col + pos[0], dir
            case 0, 'U':
                # crossing from the right to the right border (flipping)
                dir = (dir + 2) % len(DIRS)
                return self.row + self.size - pos[0], self.col + self.size, dir
            case 1, 'R':
                # crossing from the bottom to the right border
                dir = (dir + 1) % len(DIRS)
                return self.row + pos[1], self.col + self.size, dir
            case 1, 'L':
                # crossing from the bottom to the left border
                dir = (dir - 1) % len(DIRS)
                return self.row + pos[1], self.col, dir
            case 1, 'U':
                # crossing from the bottom to the bottom border
                dir = (dir + 2) % len(DIRS)
                return self.row + self.size, self.col + self.size - pos[1], dir
            case 1, '=':
                # bottom to top - no change in dir
                return self.row, self.col + pos[1], dir
            case 2, 'R':
                # crossing from the left to the bottom border
                dir = (dir + 1) % len(DIRS)
                return self.row + self.size, self.col + pos[0], dir
            case 2, 'L':
                # crossing from the left to the top border
                dir = (dir - 1) % len(DIRS)
                return self.row, self.col + pos[0], dir
            case 2, 'U':
                # crossing from the left to the left border
                dir = (dir + 2) % len(DIRS)
                return self.row + self.size - pos[0], self.col, dir
            case 3, 'R':
                # crossing from the top to the right border
                dir = (dir + 1) % len(DIRS)
                return self.row + pos[1], self.col, dir
            case 3, 'L':
                # crossing from the top to the left border
                dir = (dir - 1) % len(DIRS)
                return self.row + pos[1], self.col + self.size, dir
            case 3, 'U':
                # crossing from the top to the top border
                dir = (dir + 2) % len(DIRS)
                return self.row, self.col + self.size - pos[1], dir
            case 3, '=':
                # top to bottom - no change in dir
                return self.row + self.size, self.col + pos[1], dir
        raise ValueError(f"Cannot determine where to go for {dir}, {turn} / {pos}")


class Solution(Solver):
    def __init__(self):
        self.map = []
        self.path = None
        self.is_map = True
        self.cube_size = None
        self.cube_faces = []

    def parse(self, line: str):
        if not line:
            self.is_map = False
            return
        if self.is_map:
            self.map.append(line)
        elif line.startswith("CUBE SIZE "):
            self.cube_size = int(line[10:])
        elif line.startswith("CUBE FACE "):
            self.add_facing(line[10:])
        else:
            self.path = parse_path(line)

    def find_faces(self):
        # there is (should be) always one face in row 0
        next_row = 0
        next_col = 0
        while len(self.cube_faces) < 6:
            if next_col < len(self.map[next_row]) and self.map[next_row][next_col] != ' ':
                face = Face(len(self.cube_faces) + 1, next_row, next_col, self.cube_size - 1, [])
                self.cube_faces.append(face)
                log.debug(f"Found face {face.id} at {face.row}, {face.col}")
            next_col += self.cube_size
            if len(self.map[next_row]) < next_col:
                next_row += self.cube_size
                next_col = 0

    def add_facing(self, text):
        if not self.cube_faces:
            self.find_faces()
        id = int(text[0])
        # parse facing into a list of pairs
        parts = text[2:].split(",")
        facing = [[int(parts[t * 2]), None if parts[t * 2 + 1] == "None" else parts[t * 2 + 1]] for t in range(len(parts) // 2)]
        for f in self.cube_faces:
            if f.id == id:
                f.facing = facing

    def solve(self):
        log.debug(f"Path: {len(self.path)} movements+turns")
        pos = [0, self.map[0].find("."), 0]
        log.debug(f"Starting position: {pos}")
        for walk, turn in self.path:
            pos = self.walk(pos, walk)
            # print(f"Walk {walk} => {pos}")
            pos = self.turn(pos, turn)
            # print(f"Turn {turn} => {pos}")
        password1 = (pos[0] + 1) * 1000 + (pos[1] + 1) * 4 + pos[2]
        log.info(f"[1] final position: {pos} => password {password1}")

        log.debug("==> Cube Walk")
        pos = [0, self.map[0].find("."), 0]
        log.debug(f"Starting position: {pos[0] + 1, pos[1] + 1, DIR_TEXT[pos[2]]}")
        for walk, turn in self.path:
            pos = self.walk(pos, walk, True)
            # print(f"Walk {walk} => {pos}")
            pos = self.turn(pos, turn)
            # print(f"Turn {turn} => {pos}")
        password2 = (pos[0] + 1) * 1000 + (pos[1] + 1) * 4 + pos[2]
        log.info(f"[2] final position: {pos} => password  {password2}")
        return str(password1), str(password2)

    def walk(self, pos, walk, cube_walk=False):
        # rows are ordered top->bottom: v moves down = rows + 1; ^ moves up = rows -1
        while walk > 0:
            if pos[2] == 0 or pos[2] == 2:
                nr, nc, npos = self.cube_walk(pos[0], pos[1], pos[2]) if cube_walk else self.move_col(pos[0], pos[1], pos[2])
            elif pos[2] == 1 or pos[2] == 3:
                nr, nc, npos = self.cube_walk(pos[0], pos[1], pos[2]) if cube_walk else self.move_row(pos[0], pos[1], pos[2])
            if self.map[nr][nc] == '#':
                # stop
                break
            assert self.map[nr][nc] == '.'
            walk -= 1
            pos[0] = nr
            pos[1] = nc
            pos[2] = npos
        assert self.map[pos[0]][pos[1]] == '.'
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
        dc = DIRS[dir][1]
        c += dc
        row_len = len(self.map[r])
        while not self.in_map(r, c):
            if c < 0:
                c = row_len - 1
            elif c >= row_len:
                c = 0
            elif self.map[r][c] == ' ':
                c += dc
        return r, c, dir

    def move_row(self, r, c, dir):
        dr = DIRS[dir][0]
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
        return r, c, dir

    def cube_walk(self, r0, c0, dir):
        # only move one step
        dr = DIRS[dir][0]
        dc = DIRS[dir][1]
        face = self.in_face(r0, c0)
        r = r0 + dr
        c = c0 + dc
        if face.contains(r, c):
            return r, c, dir
        # select expected adjacent based on direction
        adj = face.facing[dir]
        if adj[1] is None:
            log.debug(f"Crossing face {face.id} / {adj[0]}: {r0 + 1, c0 + 1, DIR_TEXT[dir]} => {r + 1, c + 1, DIR_TEXT[dir]}")
            return r, c, dir
        fadj = self.cube_faces[adj[0] - 1]
        # cross into new face based on old position
        pos = fadj.cross(face.relative(r0, c0), dir, adj[1])
        assert fadj.contains(pos[0], pos[1])
        log.debug(f"Crossing face {face.id} / {fadj.id}: {r0 + 1, c0 + 1, DIR_TEXT[dir]} => {pos[0] + 1, pos[1] + 1, DIR_TEXT[pos[2]]}")
        return pos

    def in_face(self, row, col):
        for f in self.cube_faces:
            if f.contains(row, col):
                return f
        raise ValueError(f"{row}, {col} not in any face?")

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
        return pos
