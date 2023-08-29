import logging

from advent import Solver


# https://adventofcode.com/2022/day/8
# quite verbose. There's a lot of "factorizable" code. And I also enjoyed debugging a minefield of off-by-one errors

log = logging.getLogger("day.08")


class Solution(Solver):
    def __init__(self):
        self.matrix = []
        self.width = None
        self.height = 0

    def parse(self, line: str):
        row = [{"size": int(x), "visible": False} for x in line]
        if self.width:
            assert len(row) == self.width
        else:
            self.width = len(row)
        self.matrix.append(row)

    def check_column(self, col):
        # view from top
        visible = 0
        tree = self.matrix[0][col]["size"]
        for row in range(1, self.height - 1):
            t = self.matrix[row][col]
            if t["size"] > tree:
                visible += 1 if not t["visible"] else 0
                t["visible"] = True
                tree = t["size"]
        # visible from bottom
        tree = self.matrix[-1][col]["size"]
        for row in range(self.height - 1, 0, -1):
            t = self.matrix[row][col]
            if t["size"] > tree:
                visible += 1 if not t["visible"] else 0
                t["visible"] = True
                tree = t["size"]

        return visible

    def check_row(self, row):
        # view from left
        visible = 0
        tree = self.matrix[row][0]["size"]
        for col in range(1, self.width - 1):
            t = self.matrix[row][col]
            if t["size"] > tree:
                visible += 1 if not t["visible"] else 0
                t["visible"] = True
                tree = t["size"]
        # visible from right
        tree = self.matrix[row][-1]["size"]
        for col in range(self.width - 1, 0, -1):
            t = self.matrix[row][col]
            if t["size"] > tree:
                visible += 1 if not t["visible"] else 0
                t["visible"] = True
                tree = t["size"]

        return visible

    def solve(self):
        # part 1
        inside = 0
        self.height = len(self.matrix)

        # sweep columns (top and bottom)
        for col in range(1, self.width - 1):
            inside += self.check_column(col)
        for row in range(1, self.height - 1):
            inside += self.check_row(row)
        # borders: 2 x (width + height) - 4 (corners, to not count them multiple times)
        borders = self.width * 2 + self.height * 2 - 4
        trees_visible = inside + borders
        print(f"[1] number of trees visible {inside}, {borders} => {trees_visible}")

        # part 2
        scenic_max = 0
        for row in range(1, self.height - 1):
            for col in range(1, self.width - 1):
                score = self.tree_score(row, col)
                if score > scenic_max:
                    scenic_max = score
        print(f"[2] max scenic score {scenic_max}")
        return str(trees_visible), str(scenic_max)

    def tree_score(self, row, col):
        score = 1
        tree = self.matrix[row][col]
        # go north
        r = row-1
        while r > 0 and self.matrix[r][col]["size"] < tree["size"]:
            r -= 1
        score *= abs(row - r)
        # go south
        r = row+1
        while r < self.height - 1 and self.matrix[r][col]["size"] < tree["size"]:
            r += 1
        score *= abs(row - r)
        # go east
        c = col+1
        while c < self.width - 1 and self.matrix[row][c]["size"] < tree["size"]:
            c += 1
        score *= abs(col - c)
        # go west (young man)
        c = col-1
        while c > 0 and self.matrix[row][c]["size"] < tree["size"]:
            c -= 1
        score *= abs(col - c)

        return score
