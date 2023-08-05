import sys
import pathlib


# reads a file line by line, passes each line to the parser and then call solve on the parser
def solve(filename: pathlib.Path, parser):
    with open(filename) as f:
        for l in f:
            parser.parse(l.rstrip())
    parser.solve()


if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("Please specify a day to resolve like 'day03'")
    day = sys.argv[1]
    print(f"== Solving {day} ==")
    module = __import__(day)
    # test input
    test_data = pathlib.Path(__file__).parent / f"../inputs/{day}/test.txt"
    solve(test_data, module.Solution())
    # real input
    input_data = pathlib.Path(__file__).parent / f"../inputs/{day}/input.txt"
    solve(input_data, module.Solution())

