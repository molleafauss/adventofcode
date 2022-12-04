from utils import Parser, loader


def inside(pos, range):
    return range[0] <= pos <= range[1]


class Day04(Parser):
    def __init__(self):
        self.full_overlaps = 0
        self.partial_overlaps = 0

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


if __name__ == '__main__':
    loader("../files/day04-cleanup.txt", Day04())
