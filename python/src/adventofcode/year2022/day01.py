import logging

from advent import Solver

log = logging.getLogger("day.01")


# https://adventofcode.com/2022/day/1
# quite simple, of course just go through it in one pass of the file.
# both solutions can be calculated while reading the file
class Solution(Solver):
    def __init__(self):
        self.elf = 0
        self.elf_calories = 0
        self.calories = []

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
        part1 = self.calories[0][1]
        log.info(f"[1] Saw {self.elf} elves: maximum: {self.calories[0]}")
        top3 = [x[1] for x in self.calories[0:3]]
        part2 = sum(top3)
        log.info(f"[2] First 3 elves: {part2}")
        return str(part1), str(part2)
