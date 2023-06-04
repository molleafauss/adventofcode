from dataclasses import dataclass

from advent import Solver
import re
import math

# https://adventofcode.com/2022/day/16
# this needed a lot of looking at other solutions. Of course this is the usual "dynamic programming" (quotes needed)
# problem, where you have to remember your previous steps

RE_VALVE = re.compile(r"Valve (\S+) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
MAX_MINUTES = 30


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


class Solution(Solver):
    def __init__(self):
        self.valves = {}
        self.cache = {}

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
        print(f"Found {len(self.valves)} valves to open in {MAX_MINUTES} minutes")
        start = 'AA'
        valves_with_flow = {v.name for v in self.valves.values() if v.flow > 0}
        print(f"Valves with flow: {len(valves_with_flow)} => {math.factorial(len(valves_with_flow))} possible paths")
        # find distances between all valves with a flow, plus the starting valve (which can have flow also)
        distances = self.calculate_distances({start} | valves_with_flow)

        best_path = self.find_path(Path(["AA"], valves_with_flow, 0, 0), distances)

        print(f"[1] Found max flow is {best_path.total_flow}: {best_path.visited}")

    def find_path(self, path, distances):
        cave = path.visited[-1]
        cache_key = (cave, path.elapsed, tuple(sorted(path.remaining_valves)))
        if cache_key in self.cache:
            cached = self.cache[cache_key]
            # add the cached delta to the current status
            return Path(path.visited + cached.visited,
                        cached.remaining_valves,
                        path.elapsed + cached.elapsed,
                        path.total_flow + cached.total_flow)

        best_path = path
        for valve in path.remaining_valves:
            distance = distances[cave][valve]
            elapsed = path.elapsed + distance + 1
            if elapsed >= MAX_MINUTES:
                # would not be able to do anything in time
                continue
            flow = (MAX_MINUTES - elapsed) * self.valves[valve].flow
            sub_best = self.find_path(
                Path(path.visited + [valve], path.remaining_valves ^ {valve}, elapsed, path.total_flow + flow),
                distances)
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

    def calculate_distances(self, valves):
        distances = {}
        for valve in valves:
            distances[valve] = {valve: 0}
            visited = {valve}
            queue = [(0, valve)]
            while queue:
                distance, cave = queue.pop(0)
                for next in self.valves[cave].connections:
                    if next in visited:
                        continue
                    visited.add(next)
                    if next in valves:
                        distances[valve][next] = distance + 1
                    queue.append((distance + 1, next))

            del distances[valve][valve]
        return distances

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



