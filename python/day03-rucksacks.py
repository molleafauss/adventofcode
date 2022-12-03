# upper score: ord(ch) - 'A' + 27
UPPER = 27 - ord('A')
LOWER = 1 - ord('a')


class Day03:
    def __init__(self):
        self.count = 0
        self.count2 = 0
        self.possible_badges = None
        self.row = 0

    def parse(self, line):
        self.row += 1
        # assuming this is even
        mid = int(len(line) / 2)
        first_half_items = set(line[0:mid])
        failed_item = None
        for c in line[mid:]:
            if c in first_half_items:
                failed_item = c
                break
        self.count += ord(failed_item) + (LOWER if failed_item.islower() else UPPER)
        # possible badges in every triplet
        if self.row % 3 == 1:
            self.possible_badges = set(line)
        else:
            self.possible_badges.intersection_update(set(line))
        if self.row % 3 == 0:
            assert len(self.possible_badges) == 1
            badge = self.possible_badges.pop()
            self.count2 += ord(badge) + (LOWER if badge.islower() else UPPER)

    def solve(self):
        print(f"[1] Priority of item in both compartments {self.count}")
        print(f"[2] Overall priority of badges {self.count2}")


if __name__ == '__main__':
    parser = Day03()
    with open("../files/day03-rucksacks.txt") as f:
        for l in f:
            l = l.strip()
            parser.parse(l)
    parser.solve()
