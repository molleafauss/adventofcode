from dataclasses import dataclass

from advent import Solver

# https://adventofcode.com/2022/day/17

EMPTY = 0
VOXEL = 1
AIR = 2

@dataclass
class Ranges:
    x: [int, int]
    y: [int, int]
    z: [int, int]

    def add_voxel(self, x, y, z):
        if self.x[0] is None or self.x[0] > x:
            self.x[0] = x
        if self.x[1] is None or self.x[1] < x:
            self.x[1] = x
        if self.y[0] is None or self.y[0] > y:
            self.y[0] = y
        if self.y[1] is None or self.y[1] < y:
            self.y[1] = y
        if self.z[0] is None or self.z[0] > z:
            self.z[0] = z
        if self.z[1] is None or self.z[1] < z:
            self.z[1] = z

    def contains(self, pos):
        return self.x[0] <= pos[0] <= self.x[1] and self.y[0] <= pos[1] <= self.y[1] and self.z[0] <= pos[2] <= self.z[1]

    def volume(self):
        return abs(self.x[1] - self.x[0] + 1) * abs(self.y[1] - self.y[0] + 1) * abs(self.z[1] - self.z[0] + 1)


def neighbours(pos):
    x, y, z = pos
    yield from [(x + 1, y, z), (x - 1, y, z), (x, y + 1, z), (x, y - 1, z), (x, y, z + 1), (x, y, z - 1)]


class Solution(Solver):
    def __init__(self):
        self.faces = 0
        self.voxels = {}
        self.cubes = 0
        self.ranges = Ranges([None, None], [None, None], [None, None])

    def parse(self, line: str):
        x, y, z = [int(i) for i in line.strip().split(",")]
        pos = (x, y, z)
        assert self.voxels.get(pos, AIR) != VOXEL
        visible = 6
        # find voxel neighbours
        for pos in neighbours(pos):
            if pos in self.voxels:
                self.faces -= 1
                visible -= 1
        self.voxels[(x, y, z)] = VOXEL
        self.ranges.add_voxel(x, y, z)
        self.faces += visible
        self.cubes += 1

    def solve(self):
        print(f"[1] {len(self.voxels)} cubes: {self.faces} visible")

        print(f"Ranges: {self.ranges}")

    def file_name(self):
        return "../files/day18-cubes.txt"

    def test_data(self):
        return """2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"""
