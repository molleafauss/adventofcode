from typing import Protocol


# Parser "interface" used by the loader
class Parser(Protocol):
    def parse(self, line: str):
        ...

    def solve(self):
        ...


# reads a file line by line, passes each line to the parser and then call solve on the parser
def loader(filename: str, parser: Parser):
    with open(filename) as f:
        for l in f:
            parser.parse(l.rstrip())
    parser.solve()
