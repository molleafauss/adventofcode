import logging
import re
from adventofcode import Solver

# https://adventofcode.com/2022/day/5
# This was a fun one - I accumulated stacks and instructions and then built the stacks from the input after parsing
# entire file. I could not figure a way to build stacks while reading the file. Glad I didn't, as second part had to
# start from the initial status again. I know parsing isn't the most elegant.
# But we had regexp for the instructions!

log = logging.getLogger("day.05")


RE_INSTRUCTION = re.compile(r"move (\d+) from (\d+) to (\d+)")


class Solution(Solver):
    def __init__(self):
        self.stack_defs = []
        self.stacks = []
        self.instructions = []
        self.parse_instructions = False

    def parse(self, line: str):
        if not line:
            self.parse_instructions = True
            return
        if self.parse_instructions:
            self.add_instructions(line)
        else:
            self.stack_defs.append(line)

    def solve(self):
        self.stack_defs.reverse()
        self.build_stacks()
        self.move_singles()
        result1 = "".join([st.pop() for st in self.stacks])
        log.info(f"[1] Top stacks values: {result1}")
        self.build_stacks()
        self.move_multiples()
        result2 = "".join([st.pop() for st in self.stacks])
        log.info(f"[2] Top stacks values: {result2}")
        return result1, result2

    def add_instructions(self, line):
        mo = RE_INSTRUCTION.match(line)
        assert mo is not None
        self.instructions.append({"num": int(mo.group(1)), "from": int(mo.group(2)), "to": int(mo.group(3))})

    def build_stacks(self):
        num_stacks = None
        for level in self.stack_defs:
            if not num_stacks:
                num_stacks = int(level.rsplit(" ", maxsplit=1)[-1].strip())
                log.debug(f"Found {num_stacks} stacks")
                self.stacks = [[] for _ in range(num_stacks)]
                continue

            i = 0
            while i < num_stacks:
                start = i*4
                end = i*4+3
                if end > len(level):
                    break
                if level[start:end] != "   ":
                    self.stacks[i].append(level[start+1])
                i += 1

    def move_singles(self):
        for i in self.instructions:
            moves = i["num"]
            while moves > 0:
                self.stacks[i["to"] - 1].append(self.stacks[i["from"] - 1].pop())
                moves -= 1

    def move_multiples(self):
        for i in self.instructions:
            for k in range(i["num"]):
                self.stacks[i["to"] - 1].append(self.stacks[i["from"] - 1][-i["num"] + k])
            del self.stacks[i["from"] - 1][-i["num"]:]

