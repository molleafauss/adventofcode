from adc import Solver


# https://adventofcode.com/2022/day/4
def inside(pos, range):
    return range[0] <= pos <= range[1]


class Solution(Solver):
    def __init__(self):
        self.full_overlaps = 0
        self.partial_overlaps = 0

    def file_name(self):
        return "../files/day04-cleanup.txt"

    def test_data(self):
        return """2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"""

    def parse(self, line: str):
        pairs = line.split(",")
        assert len(pairs) == 2
        elf1 = [int(x) for x in pairs[0].split("-")]
        assert len(elf1) == 2
        elf2 = [int(x) for x in pairs[1].split("-")]
        assert len(elf2) == 2
        # full overlap when both ends of one of the ranges are fully inside the other
        if (elf1[0] <= elf2[0] and elf1[1] >= elf2[1]) or (elf1[0] >= elf2[0] and elf1[1] <= elf2[1]):
            self.full_overlaps += 1
        # partial overlap if either end of each range is inside the other (one check is enough?)
        if inside(elf1[0], elf2) or inside(elf1[1], elf2) or inside(elf2[0], elf1) or inside(elf2[1], elf1):
            self.partial_overlaps += 1

    def solve(self):
        print(f"Fully overlapping sections: {self.full_overlaps}")
        print(f"Partially overlapping sections: {self.partial_overlaps}")
