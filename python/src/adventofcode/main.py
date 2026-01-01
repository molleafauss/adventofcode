import argparse
from datetime import datetime
import logging
import pathlib

import sys
import time
from importlib import import_module


# reads a file line by line, passes each line to the parser and then call solve on the parser
def solve(filename: pathlib.Path, parser):
    expected_part_1 = None
    expected_part_2 = None
    with open(filename) as f:
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
    logging.info(f"File {filename}: {t1 - t0:.3f}sec")
    if not result:
        logging.warning("==> No result given")
        return
    if expected_part_1 and result[0] == expected_part_1:
        logging.info(f"PART 1 - found expected result: {expected_part_1} = {result[0]}")
    elif expected_part_1 and result[0] != expected_part_1:
        logging.error(
            f"ERROR - part 1 result is incorrect: expected {expected_part_1}, actual {result[0]}"
        )
    if expected_part_2 and result[1] == expected_part_2:
        logging.info(f"PART 2 - found expected result: {expected_part_2} = {result[1]}")
    elif expected_part_2 and result[1] != expected_part_2:
        logging.error(
            f"ERROR - part 2 result is incorrect: expected {expected_part_2}, actual {result[1]}"
        )


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
        data = day[:5]
        logging.info(f"== Solving {self.year} / {day} ==")
        module = import_module(f"adventofcode.year{self.year}.{day}")
        # test input
        test_data = pathlib.Path(self.inputs_dir) / f"{self.year}/{data}/test.txt"
        solve(test_data, module.Solution())
        # real input
        input_data = pathlib.Path(self.inputs_dir) / f"{self.year}/{data}/input.txt"
        solve(input_data, module.Solution())


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
