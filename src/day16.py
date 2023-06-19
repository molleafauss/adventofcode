import time
from dataclasses import dataclass

from advent import Solver
import re
import math

# https://adventofcode.com/2022/day/16
# this needed a lot of looking at other solutions. Of course this is the usual "dynamic programming" (quotes needed)
# problem, where you have to remember your previous steps

RE_VALVE = re.compile(r"Valve (\S+) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
PART1_MINUTES = 30
PART2_MINUTES = 26


@dataclass
class Valve:
    name: str
    flow: int
    connections: list[str]


@dataclass
class Path:
    visited: list[str]
    remaining_valves: set[str]
    elapsed: int
    total_flow: int


@dataclass
class Pos:
    visited: list[str]
    time: int


@dataclass
class BiPath:
    human: Pos
    elephant: Pos
    remaining_valves: set[str]
    elapsed: int
    total_flow: int


class Solution(Solver):
    def __init__(self):
        self.valves = {}
        self.cache = {}
        self.distances = {}
        self.cache_hits = 0

    def parse(self, line: str):
        if not line:
            return
        mo = RE_VALVE.match(line)
        if not mo:
            raise ValueError("Line doesn't match? " + line)
        connections = [n.strip() for n in mo.group(3).split(",")]
        valve = Valve(mo.group(1), int(mo.group(2)), connections)
        self.valves[valve.name] = valve

    def solve(self):
        print(f"Found {len(self.valves)} valves to open in {PART1_MINUTES} minutes")
        start = 'AA'
        valves_with_flow = {v.name for v in self.valves.values() if v.flow > 0}
        print(f"Valves with flow: {len(valves_with_flow)} => {math.factorial(len(valves_with_flow))} possible paths")
        # find distances between all valves with a flow, plus the starting valve (which can have flow also)
        self.calculate_distances({start} | valves_with_flow)

        t0 = time.time()
        best_path = self.find_path(Path(["AA"], valves_with_flow, 0, 0))
        t1 = time.time()
        print(f"[1] Found max flow is {best_path.total_flow}: {best_path.visited} ({self.cache_hits} cache hits) [{t1 - t0:10.3}sec]")

        self.cache_hits = 0
        self.cache = {}
        t0 = time.time()
        best_path = self.two_paths(BiPath(Pos(['AA'], 0), Pos(['AA'], 0), valves_with_flow, 0, 0))
        t1 = time.time()
        print(f"[2] Found max flow is {best_path.total_flow}: {best_path.human} / {best_path.elephant} ({self.cache_hits} cache hits) [{t1 - t0:10.3}sec]")

    def find_path(self, path):
        cave = path.visited[-1]
        cache_key = (cave, path.elapsed, tuple(sorted(path.remaining_valves)))
        if cache_key in self.cache:
            self.cache_hits += 1
            cached = self.cache[cache_key]
            # add the cached delta to the current status
            return Path(path.visited + cached.visited,
                        cached.remaining_valves,
                        path.elapsed + cached.elapsed,
                        path.total_flow + cached.total_flow)

        best_path = path
        for valve_name in path.remaining_valves:
            distance = self.distances[cave][valve_name]
            elapsed = path.elapsed + distance + 1
            if elapsed >= PART1_MINUTES:
                # would not be able to do anything in time
                continue
            valve = self.valves[valve_name]
            flow = (PART1_MINUTES - elapsed) * valve.flow
            sub_best = self.find_path(
                Path(path.visited + [valve_name], path.remaining_valves ^ {valve_name}, elapsed, path.total_flow + flow),
                )
            if sub_best.total_flow > best_path.total_flow:
                best_path = sub_best

        # I need to cache the "delta" from here to the end
        self.cache[cache_key] = Path(
            best_path.visited[len(path.visited):],
            best_path.remaining_valves,
            best_path.elapsed - path.elapsed,
            best_path.total_flow - path.total_flow
        )
        return best_path

    def two_paths(self, path):
        man_pos = path.human.visited[-1]
        ele_pos = path.elephant.visited[-1]

        # check cache
        cache_key = (
            tuple(path.human.visited),
            path.human.time,
            tuple(path.elephant.visited),
            path.elephant.time,
            tuple(sorted(path.remaining_valves))
        )
        if cache_key in self.cache:
            self.cache_hits += 1
            cached = self.cache[cache_key]
            # add the cached delta to the current status
            return BiPath(
                Pos(path.human.visited + cached.human.visited, path.human.time + cached.human.time),
                Pos(path.elephant.visited + cached.elephant.visited, path.elephant.time + cached.elephant.time),
                cached.remaining_valves,
                path.elapsed + cached.elapsed,
                path.total_flow + cached.total_flow
            )

        best_path = path
        for valve in path.remaining_valves:
            # try to move both human and elephant towards the next valve
            distance = self.distances[man_pos][valve]
            elapsed = path.human.time + distance + 1
            if elapsed < PART2_MINUTES:
                # if the human can make it and open this valve in time, do it
                flow = (PART2_MINUTES - elapsed) * self.valves[valve].flow
                sub_best = self.two_paths(
                    BiPath(
                        Pos(path.human.visited + [valve], elapsed),
                        path.elephant,
                        path.remaining_valves ^ {valve},
                        max(elapsed, path.elephant.time),
                        path.total_flow + flow
                    )
                )
                if sub_best.total_flow > best_path.total_flow:
                    best_path = sub_best

            distance = self.distances[ele_pos][valve]
            elapsed = path.elephant.time + distance + 1
            if elapsed < PART2_MINUTES:
                flow = (PART2_MINUTES - elapsed) * self.valves[valve].flow
                sub_best = self.two_paths(
                    BiPath(
                        path.human,
                        Pos(path.elephant.visited + [valve], elapsed),
                        path.remaining_valves ^ {valve},
                        max(elapsed, path.elephant.time),
                        path.total_flow + flow
                    )
                )
                if sub_best.total_flow > best_path.total_flow:
                    best_path = sub_best

        # I need to cache the "delta" from here to the end
        self.cache[cache_key] = BiPath(
            Pos(best_path.human.visited[len(path.human.visited):], best_path.human.time - path.human.time),
            Pos(best_path.elephant.visited[len(path.elephant.visited):], best_path.elephant.time - path.elephant.time),
            best_path.remaining_valves,
            best_path.elapsed - path.elapsed,
            best_path.total_flow - path.total_flow
        )
        return best_path

    def calculate_distances(self, valves):
        for valve in valves:
            self.distances[valve] = {valve: 0}
            visited = {valve}
            queue = [(0, valve)]
            while queue:
                distance, cave = queue.pop(0)
                for next in self.valves[cave].connections:
                    if next in visited:
                        continue
                    visited.add(next)
                    if next in valves:
                        self.distances[valve][next] = distance + 1
                    queue.append((distance + 1, next))

            # needed?
            del self.distances[valve][valve]

    def test_data(self):
        return """Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"""

    def file_name(self):
        return "../files/day16-caves.txt"
        # return None



