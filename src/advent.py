from typing import Protocol


# Parser "interface" used by the solver classes
class Solver(Protocol):
    def parse(self, line: str):
        ...

    def solve(self):
        ...

    def file_name(self):
        return None

    def test_data(self):
        return ""

    def run_test(self):
        for l in self.test_data().split("\n"):
            self.parse(l.rstrip())
        self.solve()

    def run(self):
        file_name = self.file_name()
        if not file_name:
            return
        with open(file_name) as f:
            for l in f:
                self.parse(l.rstrip())
        self.solve()


# reads a file line by line, passes each line to the parser and then call solve on the parser
def loader(filename: str, parser: Solver):
    with open(filename) as f:
        for l in f:
            parser.parse(l.rstrip())
    parser.solve()
