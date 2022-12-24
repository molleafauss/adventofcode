from dataclasses import dataclass

from advent import Solver
import re

# https://adventofcode.com/2022/day/16
# TODO: INCOMPLETE
# this doesn't give me the result I was expecting; I was looking to open a valve that would give me the highest flow
# from where I started, but I believe I am missing the fact that while moving I may meet other valves to open.
# Need to rethink the algorithm not as the "shortest" path, but the "longest" and from every node.


RE_VALVE = re.compile(r"Valve (\S+) has flow rate=(\d+); tunnels? leads? to valves? (.*)")


@dataclass
class Valve:
    name: str
    flow: int
    open: bool
    connections: list[str]
    times: dict


def backtrack(source, target, parents):
    node = target
    backpath = [target]
    path = []
    while node != source:
        backpath.append(parents[node])
        node = parents[node]
    for i in range(len(backpath)):
        path.append(backpath[-i - 1])
    return path


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
        valve = Valve(mo.group(1), int(mo.group(2)), False, connections, {})
        self.valves[valve.name] = valve

    def solve(self):
        print(f"Found {len(self.valves)} valves to open in {self.minutes} minutes")
        # idea: calculate all the paths from each node to every other one, so that we have the number of steps/minutes
        # to move everywhere else.
        # Hello again djikstra ;)
        for source in self.valves.values():
            for dest in self.valves:
                if source.name == dest:
                    continue
                time = 1 if dest in source.connections else self.find_path(source.name, dest)
                source.times[dest] = time

        # ok, now let's try to move through the tunnels, maximizing at every step what I can open
        start = 'AA'
        total_flow = 0
        while self.minutes >= 0:
            cur_cavern = self.valves[start]
            if cur_cavern.flow > 0 and not cur_cavern.open:
                self.minutes -= 1
                total_flow += cur_cavern.flow * self.minutes
                cur_cavern.open = True
                print(f"{self.minutes}' Opening valve at {start}: {cur_cavern.flow * self.minutes} => {total_flow}")
            next_step = None
            max_flow = 0
            for dest, time in cur_cavern.times.items():
                if time + 1 >= self.minutes:
                    continue
                if self.valves[dest].open:
                    continue
                flow = self.valves[dest].flow * (self.minutes - 1 - time)
                print(f"opening {dest} would flow {flow}")
                if flow > max_flow:
                    next_step = dest
                    max_flow = flow
            if not next_step:
                break
            self.minutes -= cur_cavern.times[next_step]
            start = next_step

        print(f"Found max flow is {total_flow}")

    def find_path(self, source, target):
        graph = {src.name: {dest: 1 for dest in src.connections} for src in self.valves.values()}
        costs = {valve: len(self.valves) + 1 for valve in self.valves}
        costs[source] = 0
        parents = {}
        next_node = source
        while next_node != target:
            for neighbor in graph[next_node]:
                if graph[next_node][neighbor] + costs[next_node] < costs[neighbor]:
                    costs[neighbor] = graph[next_node][neighbor] + costs[next_node]
                    parents[neighbor] = next_node
                del graph[neighbor][next_node]
            del costs[next_node]
            next_node = min(costs, key=costs.get)
        return len(backtrack(source, target, parents))

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

