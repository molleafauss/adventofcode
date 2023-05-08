from dataclasses import dataclass

from advent import Solver
import re

# https://adventofcode.com/2022/day/16
# TODO: INCOMPLETE
# this doesn't give me the result I was expecting; I was looking to open a valve that would give me the highest flow
# from where I started, but I believe I am missing the fact that while moving I may meet other valves to open.
# Need to rethink the algorithm not as the "shortest" path, but the "longest" and from every node.


RE_VALVE = re.compile(r"Valve (\S+) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
MAX_MINUTES = 30

@dataclass
class Valve:
    name: str
    flow: int
    connections: list[str]


@dataclass
class Cave:
    name: str
    opened: bool

@dataclass
class Path:
    visited: list[Cave]
    remaining_valves: set[str]
    elapsed: int
    total_flow: int


class Solution(Solver):
    def __init__(self):
        self.valves = {}
        self.minutes = 30

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
        print(f"Found {len(self.valves)} valves to open in {self.minutes} minutes")
        start = 'AA'
        valves_with_flow = {v.name for v in self.valves.values() if v.flow > 0}
        best_path = Path([Cave("", True)], set(), MAX_MINUTES, 0)
        paths = [Path([Cave(start, False), Cave(v, False)], valves_with_flow.copy(), 1, 0) for v in self.valves[start].connections]
        cycles = 0
        while paths:
            path = paths.pop(0)
            cycles += 1
            if (cycles % 100000) == 0:
                print(f"{cycles} paths checked, {len(paths)} remaining")
            if path.elapsed >= MAX_MINUTES:
                # can't move further
                best_path = self.check_best(path, best_path, paths)
                continue
            # don't move further if there's no way to beat the best (= sum flow as if you opened everything left now)
            max_flow = sum([self.valves[v].flow * (MAX_MINUTES - path.elapsed) for v in path.remaining_valves])
            if path.total_flow + max_flow < best_path.total_flow:
                continue
            # current cave
            valve = path.visited[-1]
            # previous caves visited after latest valve was opened, don't go back there again
            previous = set()
            i = len(path.visited) - 1
            while i >= 0 and not path.visited[i].opened:
                previous.add(path.visited[i].name)
                i -= 1
            # even if there's a valve to open, ignore and pass through, ignoring where you came from (avoid cycles)
            for v in self.valves[valve.name].connections:
                if v in previous:
                    continue
                paths.append(Path(path.visited + [Cave(v, False)], path.remaining_valves.copy(), path.elapsed + 1, path.total_flow))
            # no valve to open - visit next path
            if valve.name not in path.remaining_valves:
                continue
            # there's a valve to open
            path.remaining_valves.remove(valve.name)
            # stay 1 minute to open valve
            path.visited.append(Cave(valve.name, True))
            path.elapsed += 1
            path.total_flow += self.valves[valve.name].flow * (MAX_MINUTES - path.elapsed)
            # If all valves are open, stop and check if this is the best path so far
            if not path.remaining_valves:
                best_path = self.check_best(path, best_path, paths)
            else:
                # else add one new path per every outgoing path from this node, including where you came from
                for v in self.valves[valve.name].connections:
                    paths.append(Path(path.visited + [Cave(v, False)], path.remaining_valves.copy(), path.elapsed + 1, path.total_flow))

        print(f"[1] Found max flow is {best_path.total_flow}: {best_path.visited}")

    def check_best(self, path, best_path, paths):
        # check if it is bigger than best path
        if path.total_flow > best_path.total_flow:
            print(f"Found a better path: {path.total_flow} > {best_path.total_flow} {path.visited} (remaining: {len(paths)})")
            return path
        return best_path

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
        super().file_name()



