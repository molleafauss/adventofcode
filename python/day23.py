from advent import Solver
from grid import *

# https://adventofcode.com/2022/day/23
# "simple" - the final solution is brute force - not sure if there is a "mathematical" way to calculate it

SURROUNDING = [DIR_N, DIR_NE, DIR_E, DIR_SE, DIR_S, DIR_SW, DIR_W, DIR_NW]
MOVE_N = [DIR_NW, DIR_N, DIR_NE]
MOVE_E = [DIR_NE, DIR_E, DIR_SE]
MOVE_S = [DIR_SE, DIR_S, DIR_SW]
MOVE_W = [DIR_SW, DIR_W, DIR_NW]


class Elf:
    def __init__(self, id, row, col):
        self.id = id
        self.pos = GridPos(row, col)
        self.dir = [MOVE_N, MOVE_S, MOVE_W, MOVE_E]

    def __repr__(self):
        return f"Elf {self.id}: {self.pos}"


class Solution(Solver):
    def __init__(self):
        self.elves = []
        self.positions = set()
        self.width = 0
        self.height = 0

    def parse(self, line: str):
        if not self.width:
            self.width = len(line)
        p = -1
        while True:
            p = line.find("#", p + 1)
            if p == -1:
                break
            elf = Elf(len(self.elves), self.height, p)
            self.elves.append(elf)
            self.positions.add(elf.pos)
        self.height += 1

    def solve(self):
        print(f"Will move around {len(self.elves)} elves")
        rounds = 0
        # self.print_elves()
        while True:
            moves = 0
            rounds += 1
            planned_moves = {}
            for elf in self.elves:
                if new_pos := self.should_move(elf):
                    if new_pos not in planned_moves:
                        planned_moves[new_pos] = [elf]
                    else:
                        planned_moves[new_pos].append(elf)
            for next_pos, elves in planned_moves.items():
                move = len(elves) == 1
                for elf in elves:
                    if move:
                        # print(f"Elf {elf.id} will move {elf.pos} => {next_pos}")
                        self.positions.remove(elf.pos)
                        elf.pos = next_pos
                        self.positions.add(elf.pos)
                        moves += 1
                    else:
                        # print(f"{elf} won't move: {next_pos} => {elves}")
                        pass
            for elf in self.elves:
                elf.dir.append(elf.dir.pop(0))
            if round == 10:
                # self.print_elves()
                tl, br = self.find_grid()
                area = (br.row - tl.row + 1) * (br.col - tl.col + 1) - len(self.elves)
                print(f"[1] Empty area is {tl}, {br} / {len(self.elves)} => {area}")
            if moves == 0:
                print(f"[2] Round {rounds} => no moves")
                return
            else:
                print(f"[2] Round {rounds} => {moves}")

    def should_move(self, elf):
        # no elf in surrounding: stay put
        if len([pos for pos in SURROUNDING if elf.pos + pos in self.positions]) == 0:
            return None

        # which direction?
        for move in elf.dir:
            should_move = True
            for pos in move:
                if elf.pos + pos in self.positions:
                    should_move = False
                    break
            if should_move:
                new_pos = elf.pos + move[1]
                # print(f"{elf} would like to move => {new_pos}")
                return new_pos

        return None

    def find_grid(self):
        elf = self.elves[0]
        min_row = max_row = elf.pos.row
        min_col = max_col = elf.pos.col
        for elf in self.elves:
            if elf.pos.row < min_row:
                min_row = elf.pos.row
            elif elf.pos.row > max_row:
                max_row = elf.pos.row
            if elf.pos.col < min_col:
                min_col = elf.pos.col
            elif elf.pos.col > max_col:
                max_col = elf.pos.col
        return GridPos(min_row, min_col), GridPos(max_row, max_col)

    def print_elves(self):
        print("== Elves status ==")
        ul, br = self.find_grid()
        map = []
        sz = br.col - ul.col + 1
        for r in range(ul.row, br.row + 1):
            map.append(["."] * sz)

        for elf in self.elves:
            r = elf.pos.row - ul.row
            c = elf.pos.col - ul.col
            map[r][c] = chr(ord('a') + elf.id % 26)

        for row in map:
            print("".join(row))

        print("---------")
