from typing import Protocol


# Parser "interface" used by the solver classes
class Solver(Protocol):
    def parse(self, line: str):
        ...

    def solve(self):
        ...

