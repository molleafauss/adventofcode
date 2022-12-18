from advent import Solver


# https://adventofcode.com/2022/day/1
# quite simple, of course just go through it in one pass of the file.
# both solutions can be calculated while reading the file
class Solution(Solver):
    def __init__(self):
        self.elf = 0
        self.elf_calories = 0
        self.calories = []

    def file_name(self):
        return "../files/day01-calories.txt"

    def test_data(self):
        return """1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"""

    def parse(self, line: str):
        if not line:
            # print(f"Elf n. {self.elf} holding {self.elf_calories} calories")
            self.calories.append([self.elf, self.elf_calories])
            self.elf_calories = 0
            self.elf += 1
            return
        self.elf_calories += int(line)

    def solve(self):
        self.calories.append([self.elf, self.elf_calories])
        self.calories.sort(key=lambda x: x[1], reverse=True)
        print("Saw {} elves: maximum: {}".format(self.elf, self.calories[0]))
        top3 = [x[1] for x in self.calories[0:3]]
        print("First 3 elves: {}".format(sum(top3)))
