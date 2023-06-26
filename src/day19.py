import time
from dataclasses import dataclass

from advent import Solver
import re

# https://adventofcode.com/2022/day/19
# TODO: INCOMPLETE
# Am currently blocked in finding the right algorithm. The idea is somehow starting from finding how to "produce"
# a geode robot in the least possible steps, which probably will - based on required material - feed into producing
# the other previous robots et cetera.
# 1199 P1 answer
# git@github.com:hyper-neutrino/advent-of-code.git


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
        self.recipes = [None, None, None, None]
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
                idx = mo.end()
            self.recipes[recipe] = comps
            idx += 2


@dataclass(frozen=True)
class State:
    time: int
    robots: tuple[int, int, int, int]
    material: tuple[int, int, int, int]


MAX_MINUTES = 24


def find_max_geodes(bp: Blueprint):
    # "exploratory" algorithm: calculate a state and the find any further states that can be reached, where state is
    # given by the new robot that can be built without exceeding the maximum time available, and a new state is when
    # a new robot is being built, provided that the available robots can collect the material needed
    robots = (1, 0, 0, 0)
    material = (0, 0, 0, 0)
    cycles = 0
    memo_hit = 0
    memo = set()
    states = [State(0, robots, material)]
    max_geodes = 0
    t0 = time.time()
    while states:
        cycles += 1
        state = states.pop(0)
        if state in memo:
            memo_hit += 1
            continue
        memo.add(state)
        for material in MATERIALS:
            recipe = bp.recipes[material]
            if not recipe:
                continue
            # if I lack robots for any of the required materials, ignore this recipe
            if any(map(lambda required, robot: required != 0 and robot == 0, recipe, state.robots)):
                continue
            # evaluate how many minutes I will need to build next robot
            time_needed = max(map(
                lambda available, robots, required: (required - available) // robots if robots > 0 else 0,
                state.material,
                state.robots,
                recipe))
            if state.time + time_needed > MAX_MINUTES:
                # evaluate how many geodes at MAX_MINUTES
                geodes = state.material[GEODE] + state.robots[GEODE] * (MAX_MINUTES - state.time)
                if geodes > max_geodes:
                    max_geodes = geodes
                continue
            # calculate next state and put it in queue
            # XXX: probably better make this explicit instead of using list iterator and tuple?
            new_robots = tuple([state.robots[i] + (1 if i == material else 0) for i in MATERIALS])
            new_material = tuple([
                state.material[i] + (state.robots[i] * time_needed) - recipe[i] for i in MATERIALS
            ])
            states.append(State(
                state.time + time_needed,
                new_robots,
                new_material
            ))
            # print(f"[{cycles}] {state} => {time_needed} => [{states[-1]}]")

    t1 = time.time()
    print(f"Blueprint {bp.id} => {max_geodes} [{cycles} cycles - {memo_hit} memo hits - {t1 - t0:10.3f}sec]")
    return max_geodes


class Solution(Solver):
    def __init__(self):
        self.blueprints = []

    def parse(self, line: str):
        self.blueprints.append(Blueprint(line))

    def solve(self):
        total = 0
        for bp in self.blueprints:
            print(f"Finding max geodes for blueprint {bp.id} => {bp.recipes}")
            m = find_max_geodes(bp)
            total += m * bp.id

    def test_data(self):
        return """Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."""