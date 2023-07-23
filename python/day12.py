from advent import Solver

# https://adventofcode.com/2022/day/12
# this was a bloody fun ride. For some reason I started some "Dynamic Programming" shenanigans which - of course - led
# me nowhere. Then I managed to get someone suggesting Djikstra. Of course, it was the right suggestion, but took me
# quite a while to nail it.
# The reality is that the first idea I had was on the good track - I needed to track the visited places and make sure I
# never circled back to my steps.
# The Djikstra here doesn't build the Vertex-edges matrix, but builds it "dynamically" by using the neighbouring rule,
# adding the fact that the neighbour can be visited only if it hasn't alread (removing from the cost matrix does exactly
# that - we visited the position, added cost to all neighbours thus no other step can come back.
# Fun part the second one: I didn't notice there were (of course) spots which could not reach the top (predictable,
# though); initializing the cost to an impossible value made me add an extra condition to signal that.
# The current result is slow, as walk is called once per starting point. Once I have a set of parents, though, ideally
# I can start finding paths for every element in the possible_starts and remove them if found.

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

        def walk(start):
            print(f"=== Finding path {start} => {self.end}")
            costs = {}
            max = self.width * self.height + 1
            for row in range(self.height):
                for col in range(self.width):
                    costs[(row, col)] = 0 if (row, col) == start else max
            parents = {}
            next_node = start
            visited = 0
            while next_node != self.end:
                visited += 1
                for neighbor, _ in self.neighbours(next_node, costs):
                    if costs[next_node] + 1 < costs[neighbor]:
                        costs[neighbor] = costs[next_node] + 1
                        parents[neighbor] = next_node
                    else:
                        assert neighbor in parents
                del costs[next_node]
                to_visit = min(costs, key=costs.get)
                if costs[to_visit] == max:
                    # no path to end - bail
                    return []
                next_node = to_visit
            print(f"Found end, visited {visited}")
            return parents

        def walk_back(parents, start):
            print(f"backtracking {self.end} => {start}: {len(parents)}")
            path = []
            if self.end not in parents:
                return path
            n = self.end
            while n != start:
                path.insert(0, (n, self.map[n]))
                n = parents[n]
            return path

        parents = walk(self.start)
        path = walk_back(parents, self.start)
        print(f"[1] Min length found: {len(path)}: {path}")

        # now try to walk from all 'a'
        possible_starts = [pos for pos in self.map if self.map[pos] == 'a']
        min_length = len(path)
        min_start = self.start
        for start in possible_starts:
            path = walk_back(walk(start), start)
            if not path:
                continue
            if len(path) < min_length:
                min_length = len(path)
                min_start = start
        print(f"[2] Shortest path from {min_start}: {min_length}")

    def neighbours(self, node, costs):
        cur_height = self.map[node]
        # surrounding pos:
        pos = [(node[0] - 1, node[1]), (node[0] + 1, node[1]), (node[0], node[1] - 1), (node[0], node[1] + 1)]

        def valid(pos):
            return pos in self.map and pos in costs and ord(self.map[pos]) - ord(cur_height) <= 1

        steps = [(s, self.map[s]) for s in pos if valid(s)]
        # print(f"{node}:{self.map[node]} => {steps}")
        return steps

    def file_name(self):
        return "../inputs/day12-map.txt"

    def test_data(self):
        return """Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"""
