import logging

from advent import Solver
from grid import *

# https://adventofcode.com/2022/day/23
# "simple" - the final solution is brute force - not sure if there is a "mathematical" way to calculate it

log = logging.getLogger("day.23")


SURROUNDING = [DIR_N, DIR_NE, DIR_E, DIR_SE, DIR_S, DIR_SW, DIR_W, DIR_NW]


class Elf:
    def __init__(self, id, row, col):
        self.id = id
        self.pos = GridPos(row, col)

    def __repr__(self):
        return f"Elf {self.id}: {self.pos}"


class Solution(Solver):
    def __init__(self):
        self.elves = []
        self.positions = set()
        self.width = 0
        self.height = 0
        self.moves = [
            [DIR_NW, DIR_N, DIR_NE],
            [DIR_SE, DIR_S, DIR_SW],
            [DIR_SW, DIR_W, DIR_NW],
            [DIR_NE, DIR_E, DIR_SE]
        ]

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
        log.debug(f"Will move around {len(self.elves)} elves")
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
                #     print(f"Elf {elf.id} will move {(elf.pos.col, elf.pos.row)} => {(new_pos.col, new_pos.row)}")
                # else:
                #     print(f"Elf {elf.id} won't move => {(elf.pos.col, elf.pos.row)}")
            for next_pos, elves in planned_moves.items():
                move = len(elves) == 1
                for elf in elves:
                    if move:
                        self.positions.remove(elf.pos)
                        elf.pos = next_pos
                        self.positions.add(elf.pos)
                        moves += 1
            # rotate moves
            self.moves.append(self.moves.pop(0))
            if rounds == 10:
                # self.print_elves()
                tl, br = self.find_grid()
                area = (br.row - tl.row + 1) * (br.col - tl.col + 1) - len(self.elves)
            if moves == 0:
                break
            log.debug(f"=> Round {rounds}: {moves} moves")
        log.info(f"[1] Empty area is {tl}, {br} / {len(self.elves)} => {area}")
        log.info(f"[2] Round {rounds} => no moves")
        return str(area), str(rounds)

    def should_move(self, elf):
        # no elf in surrounding: stay put
        if len([pos for pos in SURROUNDING if elf.pos + pos in self.positions]) == 0:
            return None

        # which direction?
        for move in self.moves:
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
