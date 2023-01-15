import math

from advent import Solver

# https://adventofcode.com/2022/day/25


VALS = {
    "2": 2,
    "1": 1,
    "0": 0,
    "-": -1,
    "=": -2
}
INV_VAL = {v: k for k, v in VALS.items()}

DIGITS = [
    [0, 0],
    [0, 1],
    [0, 2],
    [1, -2],
    [1, -1]
]


def snafu_to_int(text):
    val = 0
    sz = len(text)
    for ch in text:
        sz -= 1
        v = VALS[ch]
        val += v * pow(5, sz)
    return val


def int_to_snafu(val):
    result = []
    while val > 0:
        rest = val % 5
        val = (val - rest) // 5
        d1, d0 = DIGITS[rest]
        if len(result) == 0:
            result.insert(0, d0)
            result.insert(0, d1)
            continue
        # add d0 to the most significant digit and change result. if a carry exist, increment d1
        d0 += result[0]
        if d0 > 2:
            d0 -= 5
            d1 += 1
        result[0] = d0
        result.insert(0, d1)
    while result[0] == 0:
        result.pop(0)

    text = ""
    while result:
        text += INV_VAL[result.pop(0)]
    return text


class Solution(Solver):
    def __init__(self):
        self.fuel = 0

    def parse(self, line: str):
        val = snafu_to_int(line)
        self.fuel += val
        print(f"{line} => {val} = {self.fuel}")

    def solve(self):
        fuel_base5 = int_to_snafu(self.fuel)
        print(f"{self.fuel} => to base 5 {fuel_base5}")

    def file_name(self):
        return "../files/day25-fuel.txt"

    def test_data(self):
        return """1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"""


