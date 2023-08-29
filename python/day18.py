from dataclasses import dataclass

from advent import Solver

# https://adventofcode.com/2022/day/18
# Was fun finding the right way to do it - in the end I just added a "layer" of air all around the 3D volume and
# started propagating the air until it touched the lava.

LAVA = 1
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
        assert self.voxels.get(pos, AIR) != LAVA
        visible = 6
        # find voxel neighbours
        for pos in neighbours(pos):
            if pos in self.voxels:
                self.faces -= 1
                visible -= 1
        self.voxels[(x, y, z)] = LAVA
        self.ranges.add_voxel(x, y, z)
        self.faces += visible
        self.cubes += 1

    def solve(self):
        print(f"[1] {len(self.voxels)} cubes: {self.faces} visible")

        print(f"Ranges: {self.ranges}")
        # add AIR all in the layer around and then grow it until it touches the lava. Count the faces this way
        # top layer
        self.add_air(range(self.ranges.x[0] - 1, self.ranges.x[1] + 1), range(self.ranges.y[0] - 1, self.ranges.y[1] + 1), range(self.ranges.z[0] - 1, self.ranges.z[0]))
        self.add_air(range(self.ranges.x[0] - 1, self.ranges.x[1] + 1), range(self.ranges.y[0] - 1, self.ranges.y[1] + 1), range(self.ranges.z[1] + 1, self.ranges.z[1] + 2))
        self.add_air(range(self.ranges.x[0] - 1, self.ranges.x[1] + 1), range(self.ranges.y[0] - 1, self.ranges.y[0]), range(self.ranges.z[0] - 1, self.ranges.z[1] + 1))
        self.add_air(range(self.ranges.x[0] - 1, self.ranges.x[1] + 1), range(self.ranges.y[1] + 1, self.ranges.y[1] + 2), range(self.ranges.z[0] - 1, self.ranges.z[1] + 1))
        self.add_air(range(self.ranges.x[0] - 1, self.ranges.x[0]), range(self.ranges.y[0] - 1, self.ranges.y[1] + 1), range(self.ranges.z[0] - 1, self.ranges.z[1] + 1))
        self.add_air(range(self.ranges.x[1] + 1, self.ranges.x[1] + 2), range(self.ranges.y[0] - 1, self.ranges.y[1] + 1), range(self.ranges.z[0] - 1, self.ranges.z[1] + 1))

        # expand air and count faces touched
        faces = set()
        air = [pos for pos, _ in filter(lambda v: v[1] == AIR, self.voxels.items())]
        for pos in air:
            self.voxels[pos] = AIR
            for n in neighbours(pos):
                if not self.ranges.contains(n):
                    continue
                if n not in self.voxels:
                    air.append(n)
                elif self.voxels[n] == LAVA:
                    # add the position of the voxel and the direction of the face touched
                    faces.add(self.face(pos, n))

        print(f"[2] Found {len(faces)} outside facing faces")
        return str(self.faces), str(len(faces))

    def add_air(self, xrange, yrange, zrange):
        print(f"Add air: {xrange}, {yrange}, {zrange}")
        for x in xrange:
            for y in yrange:
                for z in zrange:
                    self.voxels[(x, y, z)] = AIR

    def face(self, pos, air):
        x, y, z = pos
        dx = pos[0] - air[0]
        dy = pos[1] - air[1]
        dz = pos[2] - air[2]
        return (x, y, z, dx, dy, dz)
