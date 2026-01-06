import argparse
from datetime import datetime
import logging
import pathlib

import sys
import time
from importlib import import_module

from adventofcode import Solver


def get_last_available_year():
    t = datetime.now()
    return t.year if t.month >= 12 else t.year - 1


class Aoc:
    def __init__(self, inputs_dir, year=None):
        self.inputs_dir = inputs_dir
        self.year = year if year else get_last_available_year()

    def run(self, day):
        if day == "all":
            self.solve_all()
        elif day.startswith("day"):
            self.solve_day(day)
        else:
            logging.error(f"Invalid day specification: {day}")

    def solve_all(self):
        for i in range(1, 26):
            self.solve_day(f"day{i:02}")

    def solve_day(self, day):
        puzzle = day[:5]
        logging.info(f"== Solving {self.year}/{puzzle} ==")
        try:
            module = import_module(f"adventofcode.year{self.year}.{day}")
            # test input
            self.solve(puzzle, "test.txt", module.Solution())
            # real input
            self.solve(puzzle, "input.txt", module.Solution())
        except ImportError:
            logging.warning(f"{self.year}/{day} | no solution implemented")

    def solve(self, day: str, datafile: str, parser: Solver):
        filename = pathlib.Path(self.inputs_dir) / f"{self.year}/{day}/{datafile}"
        if not filename.exists():
            logging.warning(f"{self.year}/{day} | missing file: {datafile}")
            return
        expected_part_1 = None
        expected_part_2 = None
        with filename.open() as f:
            for line in f:
                if line.startswith("result part 1: "):
                    expected_part_1 = line[15:].strip()
                elif line.startswith("result part 2: "):
                    expected_part_2 = line[15:].strip()
                else:
                    parser.parse(line.rstrip())
        t0 = time.time()
        result = parser.solve()
        t1 = time.time()
        logging.info(f"{self.year}/{day} | {datafile} solved in {t1 - t0:.3f}sec")
        if not result:
            logging.warning(f"{self.year}/{day} | {datafile} | no result returned?")
            return
        self.verify_result(day, datafile, 1, result[0], expected_part_1)
        self.verify_result(day, datafile, 2, result[1], expected_part_2)

    def verify_result(
        self, day: str, datafile: str, part: int, result: str, expected: str | None
    ):
        if expected and result == expected:
            logging.info(
                f"{self.year}/{day} | {datafile} | RESULT PART {part} - correct: {expected}"
            )
        elif expected and result != expected:
            logging.error(
                f"{self.year}/{day} | {datafile} | RESULT PART {part} - expected {expected}, actual {result}"
            )


def main():
    p = argparse.ArgumentParser(prog="adventofcode")
    p.add_argument(
        "day",
        default=False,
        help="Day to solve (specified as 'dayNN' or 'all' to solve all days in sequence).",
    )
    p.add_argument("--debug", action="store_true", help="Enable debug log.")
    p.add_argument(
        "--year", help="Year of the Advent of Code event - default last available year."
    )
    p.add_argument(
        "--inputs",
        default=".",
        help="Directory to read input files from, default current directory.",
    )
    args = p.parse_args()

    logging.basicConfig(
        stream=sys.stdout,
        format="%(levelname)s | %(message)s",
        level=logging.DEBUG if args.debug else logging.INFO,
    )

    aoc = Aoc(args.inputs, args.year)
    aoc.run(args.day)


if __name__ == "__main__":
    main()
