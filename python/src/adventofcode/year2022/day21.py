import logging
import re
from dataclasses import dataclass

from adventofcode import Solver

# https://adventofcode.com/2022/day/21
# reasonably simple. I did calculate using recursion and was worried that it could hit a stack overflow (too many tree
# levels, but it didn't
# Inversion was also simple - I only had to visualize the equations as I was inverting them wrongly :facepalm:

log = logging.getLogger("day.21")


RE_OP = re.compile(r"(\S+) ([+\-*/]) (\S+)")
HUMAN = "humn"

@dataclass
class Monkey:
    name: str
    number: int
    operation: list


class Solution(Solver):

    def __init__(self):
        self.monkeys = {}

    def parse(self, line: str):
        if not line:
            return
        name, op = line.split(": ", maxsplit=1)
        if mo := RE_OP.match(op):
            self.monkeys[name] = Monkey(name, None, [mo.group(1), mo.group(2), mo.group(3)])
        else:
            self.monkeys[name] = Monkey(name, int(op), None)

    def solve(self):
        assert "root" in self.monkeys

        # part 1
        result = self.calculate(self.monkeys["root"])
        log.info(f"[1] Result is {result}")

        # part 2
        value = self.balance(self.monkeys["root"])
        log.info(f"[2] HUMN {value}")
        return str(int(result)), str(int(value))

    def balance(self, monkey):
        balance = None
        value = None
        # need to find which branch is the one missing data
        while monkey.name != HUMAN:
            human = 2
            try:
                value = self.calculate(self.monkeys[monkey.operation[0]], True)
            except ValueError:
                human = 0
                value = self.calculate(self.monkeys[monkey.operation[2]], True)
            if not balance:
                log.debug(f"Found root branch value: {value}")
                balance = value
            else:
                balance = self.invert_op(monkey, balance, value, human == 0)
            monkey = self.monkeys[monkey.operation[human]]
        return int(balance)

    def invert_op(self, monkey, balance, value, first):
        log.debug(f"Inverting {'x' if first else value} {monkey.operation[1]} {value if first else 'x'} = {balance}")
        match monkey.operation[1]:
            case "+":
                return balance - value
            case "-":
                return balance + value if first else value - balance
            case "*":
                return balance / value
            case "/":
                return balance * value if first else value / balance
            case "_":
                raise ValueError(f"Invalid operation: {monkey.operation}")

    def calculate(self, monkey, part2=False):
        if part2 and monkey.name == HUMAN:
            raise ValueError("Hooman do not know number")
        if monkey.number:
            return monkey.number
        left = self.calculate(self.monkeys[monkey.operation[0]], part2)
        right = self.calculate(self.monkeys[monkey.operation[2]], part2)
        match monkey.operation[1]:
            case "+":
                return left + right
            case "-":
                return left - right
            case "*":
                return left * right
            case "/":
                return left / right
            case "_":
                raise ValueError(f"Invalid operation: {monkey.operation}")
