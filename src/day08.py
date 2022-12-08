from adc import Solver


# https://adventofcode.com/2022/day/8

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
        for row in range(1, self.height):
            t = self.matrix[row][col]
            if t["size"] > tree:
                visible += 1 if not t["visible"] else 0
                t["visible"] = True
                tree = t["size"]
        # visible from bottom
        tree = self.matrix[-1][col]["size"]
        for row in range(-1, -self.width, -1):
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
        for col in range(1, self.width):
            t = self.matrix[row][col]
            if t["size"] > tree:
                visible += 1 if not t["visible"] else 0
                t["visible"] = True
                tree = t["size"]
        # visible from right
        tree = self.matrix[row][-1]["size"]
        for col in range(-1, -self.height, -1):
            t = self.matrix[row][col]
            if t["size"] > tree:
                visible += 1 if not t["visible"] else 0
                t["visible"] = True
                tree = t["size"]

        return visible

    def solve(self):
        inside = 0
        self.height = len(self.matrix)

        # sweep columns (top and bottom)
        for col in range(1, self.width - 1):
            inside += self.check_column(col)
        for row in range(1, self.height - 1):
            inside += self.check_row(row)
        # borders: 2 x (width + height) - 4 (corners, to not count them multiple times)
        borders = self.width * 2 + self.height * 2 - 4
        print(f"[1] number of trees visible {inside + borders}")

    def file_name(self):
        return "../files/day08-trees.txt"

    def test_data(self):
        return """30373
25512
65332
33549
35390"""