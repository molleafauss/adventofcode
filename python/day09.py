import logging

from advent import Solver


# https://adventofcode.com/2022/day/9
# was hoping that set.add([x, y]) would be simple but no: "Unhashable type: list". The solution is actually pretty
# simple (thanks SO - https://stackoverflow.com/a/19371472) - set keys need to be immutable.
# of course, I had to refactor for solving part 2 as the rope is longer, so I just keep instruction and do all moves
# starting from a new state (self.rope could also be passed around btw).
# part 2 highlighted the need of handling a (2, 2) difference

log = logging.getLogger("day.09")


MOVES = {
    "R": [1, 0],
    "L": [-1, 0],
    "U": [0, 1],
    "D": [0, -1]
}


class Solution(Solver):
    def __init__(self):
        self.movements = []
        self.rope = [[0, 0], [0, 0]]

    def parse(self, line: str):
        parts = line.strip().split(" ", maxsplit=1)
        self.movements.append((parts[0], int(parts[1])))

    def solve(self):
        visited1 = self.move_rope(2)
        log.info(f"[1] Tail visited {visited1} places")
        visited2 = self.move_rope(10)
        log.info(f"[2] Tail visited {visited2} places")
        return str(visited1), str(visited2)

    def move_rope(self, rope_length):
        log.debug(f"Moving rope with length {rope_length}")
        self.rope = [[0,0] for _ in range(rope_length)]
        visited = set()
        for m in self.movements:
            dir = m[0]
            steps = m[1]
            # log.debug(f"{dir} {steps}")
            while steps > 0:
                self.move_head(dir)
                for i in range(rope_length - 1):
                    self.adjust_rope(i)
                tail = self.rope[-1]
                visited.add((tail[0], tail[1]))
                steps -= 1
        return len(visited)

    def move_head(self, dir):
        self.rope[0][0] += MOVES[dir][0]
        self.rope[0][1] += MOVES[dir][1]

    def adjust_rope(self, pos):
        head = self.rope[pos]
        tail = self.rope[pos + 1]
        delta_x = head[0] - tail[0]
        delta_y = head[1] - tail[1]
        if abs(delta_x) > 2 or abs(delta_y) > 2:
            raise ValueError(f"Invalid head-tail distance ({pos}): {head} => {tail} : {(delta_x, delta_y)}")

        if abs(delta_x) <= 1 and abs(delta_y) <= 1:
            # they're close, all good
            return
        # if tail on same row/column as head, just follow line
        if delta_x == 0:
            tail[1] += 1 if delta_y > 0 else -1
        elif delta_y == 0:
            tail[0] += 1 if delta_x > 0 else -1
        elif abs(delta_x) == 2 and abs(delta_y) == 2:
            # both diagonal, I assume
            tail[0] += 1 if delta_x > 0 else -1
            tail[1] += 1 if delta_y > 0 else -1
        elif abs(delta_x) > abs(delta_y):
            # will pair y, but still be 1 behind on x
            tail[0] += 1 if delta_x > 0 else -1
            tail[1] = head[1]
        elif abs(delta_y) > abs(delta_x):
            # will pair x, but still be 1 behind on y
            tail[0] = head[0]
            tail[1] += 1 if delta_y > 0 else -1
        else:
            raise Exception("WTF?")
