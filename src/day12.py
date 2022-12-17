from adc import Solver
import re

# https://adventofcode.com/2022/day/12
# --- Day 12: Hill Climbing Algorithm ---
#
# You try contacting the Elves using your handheld device, but the river you`re following must be too low to get a decent signal.
#
# You ask the device for a heightmap of the surrounding area (your puzzle input). The heightmap shows the local area from
# above broken into a grid; the elevation of each square of the grid is given by a single lowercase letter, where a is the
# lowest elevation, b is the next-lowest, and so on up to the highest elevation, z.
#
# Also included on the heightmap are marks for your current position (S) and the location that should get the best signal #
# (E). Your current position (S) has elevation a, and the location that should get the best signal (E) has elevation z.
#
# You'd like to reach E, but to save energy, you should do it in as few steps as possible. During each step, you can move
# exactly one square up, down, left, or right. To avoid needing to get out your climbing gear, the elevation of the
# destination square can be at most one higher than the elevation of your current square; that is, if your current
# elevation is m, you could step to elevation n, but not to elevation o. (This also means that the elevation of the
# destination square can be much lower than the elevation of your current square.)
#
# For example:
#
# Sabqponm
# abcryxxl
# accszExk
# acctuvwj
# abdefghi
#
# Here, you start in the top-left corner; your goal is near the middle. You could start by moving down or right, but
# eventually you'll need to head toward the e at the bottom. From there, you can spiral around to the goal:
#
# v..v<<<<
# >v.vv<<^
# .>vv>E^^
# ..v>>>^^
# ..>>>>>^
#
# In the above diagram, the symbols indicate whether the path exits each square moving up (^), down (v), left (<),
# or right (>). The location that should get the best signal is still E, and . marks unvisited squares.
#
# This path reaches the goal in 31 steps, the fewest possible.
#
# What is the fewest steps required to move from your current position to the location that should get the best signal?


class Solution(Solver):
    def __init__(self):
        self.map = {}
        self.width = None
        self.height = 0
        self.start = None
        self.end = None
        self.path_length = None

    def parse(self, line: str):
        if not self.width:
            self.width = len(line)
        else:
            assert len(line) == self.width

        def letter(i, ch):
            if ch == 'S':
                self.start = (self.height, i)
                return 'a'
            elif ch == 'E':
                self.end = (self.height, i)
                return 'z'
            else:
                return ch

        for i in range(len(line)):
            self.map[(self.height, i)] = letter(i, line[i])
        self.height += 1

    def solve(self):
        assert self.start and self.end
        print(f"Finding path {self.start} => {self.end}")

        def walk():
            costs = {}
            max = self.width * self.height + 1
            for row in range(self.height):
                for col in range(self.width):
                    costs[(row, col)] = max
            costs[self.start] = 0
            parents = {}
            next_node = self.start
            while next_node != self.end:
                print(f"Visiting: {next_node}")
                for neighbor, _ in self.neighbours(next_node, costs):
                    if costs[next_node] + 1 < costs[neighbor]:
                        costs[neighbor] = costs[next_node] + 1
                        parents[neighbor] = next_node
                del costs[next_node]
                next_node = min(costs, key=costs.get)
            assert next_node == self.end
            return parents

        def walk_back(parents):
            path = []
            n = self.end
            while n != self.start:
                path.insert(0, (n, self.map[n]))
                n = parents[n]
            return path

        parents = walk()
        path = walk_back(parents)

        print(f"Min length found: {len(path)}: {path}")

    def neighbours(self, node, costs):
        cur_height = self.map[node]
        # surrounding pos:
        pos = [(node[0] - 1, node[1]), (node[0] + 1, node[1]), (node[0], node[1] - 1), (node[0], node[1] + 1)]

        def valid(pos):
            return pos in self.map and pos in costs and ord(self.map[pos]) - ord(cur_height) <= 1

        steps = [(s, self.map[s]) for s in pos if valid(s)]
        print(f"{node}:{self.map[node]} => {steps}")
        return steps

    def file_name(self):
        return "../files/day12-map.txt"

    def test_data(self):
        return """Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"""
