from advent import Solver
import re

# https://adventofcode.com/2022/day/18
# TODO: INCOMPLETE
# Am currently blocked in finding the right algorithm. The idea is somehow starting from finding how to "produce"
# a geode robot in the least possible steps, which probably will - based on required material - feed into producing
# the other previous robots et cetera.


ORE = 0
CLAY = 1
OBSIDIAN = 2
GEODE = 3

RE_ID = re.compile(r"Blueprint (\d+): ")
RE_RECIPE = re.compile(r"Each ([a-z]+) robot costs ")
RE_COMP = re.compile(r"( and )?(\d+) ([a-z]+)")

def type(text):
    match text:
        case "ore":
            return ORE
        case "clay":
            return CLAY
        case "obsidian":
            return OBSIDIAN
        case "geode":
            return GEODE
        case _:
            raise ValueError("Invalid type: " + text)


class Blueprint:
    def __init__(self):
        self.id = None
        self.recipes = {}

    def parse(self, line: str):
        mo = RE_ID.match(line)
        if not mo:
            raise ValueError("Invalid blueprint: " +  line)
        self.id = int(mo.group(1))
        idx = mo.end()
        while idx < len(line):
            mo = RE_RECIPE.match(line, idx)
            if not mo:
                raise ValueError("Can't find recipe: " + line[idx:])
            recipe = type(mo.group(1))
            idx = mo.end()
            comps = {}
            while line[idx] != ".":
                mo = RE_COMP.match(line, idx)
                if not mo:
                    raise ValueError("No component defs: " + line[idx:])
                num = int(mo.group(2))
                comp = type(mo.group(3))
                comps[comp] = num
                idx = mo.end()
            self.recipes[recipe] = comps
            idx += 2


class Solution(Solver):
    def __init__(self):
        self.blueprints = []

    def parse(self, line: str):
        b = Blueprint()
        b.parse(line)
        self.blueprints.append(b)

    def solve(self):
        for b in self.blueprints:
            print(f"{b.id} => {b.recipes}")

    def test_data(self):
        return """Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."""