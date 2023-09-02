import logging
import sys
import pathlib


# reads a file line by line, passes each line to the parser and then call solve on the parser
import time


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
        logging.error(f"ERROR - part 1 result is incorrect: expected {expected_part_1}, actual {result[0]}")
    if expected_part_2 and result[1] == expected_part_2:
        logging.info(f"PART 2 - found expected result: {expected_part_2} = {result[1]}")
    elif expected_part_2 and result[1] != expected_part_2:
        logging.error(f"ERROR - part 2 result is incorrect: expected {expected_part_2}, actual {result[1]}")


def solve_all():
    for i in range(1, 26):
        solve_day(f"day{i:02}")


def solve_day(day):
    logging.basicConfig(stream=sys.stdout, format="%(levelname)s | %(name)s | %(message)s", level=logging.INFO)
    logging.getLogger("day").setLevel(logging.WARN)
    logging.info(f"== Solving {day} ==")
    module = __import__(day)
    # test input
    test_data = pathlib.Path(__file__).parent / f"../inputs/2022/{day}/test.txt"
    solve(test_data, module.Solution())
    # real input
    input_data = pathlib.Path(__file__).parent / f"../inputs/2022/{day}/input.txt"
    solve(input_data, module.Solution())


if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("Please specify a day to resolve like 'day03'")
    day = sys.argv[1]
    if day == "all":
        solve_all()
    else:
        solve_day(day)

