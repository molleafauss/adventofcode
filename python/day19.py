import sys
import time

from advent import Solver
import re

# https://adventofcode.com/2022/day/19
# I had to use a lot of help here to find the result. Tbh I actually had kinda the right algorithm implemented, but I
# ended introducing a lot of bugs and missed optimizations that were causing the iterations to never end.
# I tried a non-recursive solution first, and then landed on the recursive as it's "cleaner".
# Thanks to hyper-neutrino I also discovered the for ... else construct in python which I didn't know before.


ORE = 0
CLAY = 1
OBSIDIAN = 2
GEODE = 3
MATERIALS = [ORE, CLAY, OBSIDIAN, GEODE]

RE_ID = re.compile(r"Blueprint (\d+): ")
RE_RECIPE = re.compile(r"Each ([a-z]+) robot costs ")
RE_COMP = re.compile(r"( and )?(\d+) ([a-z]+)")


def parse_type(text):
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
    def __init__(self, line: str):
        self.id = None
        self.iterations = 0
        self.cache = {}
        self.cache_hits = 0
        self.recipes = [None, None, None, None]
        self.max_materials = [0, 0, 0, 0]
        self._parse(line)

    def _parse(self, line: str):
        mo = RE_ID.match(line)
        if not mo:
            raise ValueError("Invalid blueprint definition: " +  line)
        self.id = int(mo.group(1))
        idx = mo.end()
        while idx < len(line):
            mo = RE_RECIPE.match(line, idx)
            if not mo:
                raise ValueError("Can't find recipe: " + line[idx:])
            recipe = parse_type(mo.group(1))
            idx = mo.end()
            comps = [0, 0, 0, 0]
            while line[idx] != ".":
                mo = RE_COMP.match(line, idx)
                if not mo:
                    raise ValueError("No component defs: " + line[idx:])
                num = int(mo.group(2))
                comp = parse_type(mo.group(3))
                comps[comp] = num
                self.max_materials[comp] = max(self.max_materials[comp], num)
                idx = mo.end()
            self.recipes[recipe] = comps
            idx += 2


class Solution(Solver):
    def __init__(self):
        self.blueprints = []

    def parse(self, line: str):
        self.blueprints.append(Blueprint(line))

    def solve(self):
        total1 = 0
        total2 = 1
        part2 = 0
        for bp in self.blueprints:
            print(f"Finding max geodes for blueprint {bp.id} => {bp.recipes}")
            robots = [1, 0, 0, 0]
            material = [0, 0, 0, 0]
            t0 = time.time()
            max_geodes = self.find_max_geodes(bp, 24, robots, material)
            total1 += max_geodes * bp.id
            t1 = time.time()
            print(f"[part 1] Blueprint {bp.id} => {max_geodes} ({total1}) [{t1 - t0:10.3f}sec {bp.iterations} total calls / {1000000 * (t1 - t0)/bp.iterations:10.3f} us/call / {bp.cache_hits} cache hits]")

            if part2 < 3:
                part2 += 1
                t0 = time.time()
                bp.iterations = 0
                bp.cache_hits = 0
                max_geodes = self.find_max_geodes(bp, 32, robots, material)
                total2 *= max_geodes
                t1 = time.time()
                print(f"[part 2] Blueprint {bp.id} => {max_geodes} ({total2}) [{t1 - t0:10.3f}sec {bp.iterations} total calls / {1000000 * (t1 - t0)/bp.iterations:10.3f} us/call / {bp.cache_hits} cache hits]")
        print(f"[1] result is {total1}")
        print(f"[2] result is {total2}")

    def find_max_geodes(self, bp: Blueprint, minutes_left, robots, materials):
        bp.iterations += 1
        if (bp.iterations % 500000) == 0:
            print(f"{bp.iterations} hits {bp.cache_hits} ...")
        assert minutes_left >= 0
        # if we're at time, just return what we have
        if minutes_left == 0:
            return materials[GEODE]
        # have we seen this?
        cache_key = (minutes_left, tuple(robots), (
            # if I have already more material (except GEODES) than how much I can spend in the remaining time,
            # will use the maximum that can be spent as cache key
            min(materials[ORE], bp.max_materials[ORE] * minutes_left),
            min(materials[CLAY], bp.max_materials[CLAY] * minutes_left),
            min(materials[OBSIDIAN], bp.max_materials[OBSIDIAN] * minutes_left),
            materials[GEODE]
        ))
        if cache_key in bp.cache:
            bp.cache_hits += 1
            return bp.cache[cache_key]
        # start with maximum that can be produced by the current status
        max_geodes = materials[GEODE] + (robots[GEODE] * minutes_left)
        for bot_type, recipe in enumerate(bp.recipes):
            if not recipe:
                continue
            if bot_type != GEODE and robots[bot_type] >= bp.max_materials[bot_type]:
                # culling - building robots of a type over the maximum consumption of a material is not necessary
                continue
            time_needed = 1
            for mat, required in enumerate(recipe):
                # if I lack robots for any of the required materials, ignore this recipe
                if robots[mat] == 0 and recipe[mat] > 0:
                    break
                # how many minutes to produce enough material for this robot type
                if recipe[mat] > 0:
                    time_needed = max(time_needed, -((materials[mat] - recipe[mat]) // robots[mat]) + 1)
            else:
                if minutes_left - time_needed <= 0:
                    # will exhaust the time - won't be able to add anything more to the max already calculated
                    continue
                new_robots = robots.copy()
                new_robots[bot_type] += 1
                new_materials = [materials[i] + (robots[i] * time_needed) - recipe[i] for i in MATERIALS]
                max_geodes = max(max_geodes, self.find_max_geodes(bp, minutes_left - time_needed, new_robots, new_materials))

        bp.cache[cache_key] = max_geodes
        return max_geodes

    def test_data(self):
        return """Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."""

    def file_name(self):
        return "../inputs/day19-blueprints.txt"
