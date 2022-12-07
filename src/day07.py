from adc import Solver


# https://adventofcode.com/2022/day/7

class Solution(Solver):
    def __init__(self):
        self.tree = {"name": "", "entries": {}}
        self.dirstack = [self.tree]

    def parse(self, line: str):
        if line.startswith("$ "):
            self.handle_command(line[2:])
        else:
            self.record_entry(line)

    def solve(self):
        pass

    def file_name(self):
        return "../files/day07-dirs.txt"

    def test_data(self):
        return """$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"""

    def handle_command(self, command):
        if command == "cd /":
            self.dirstack = [self.tree]
        elif command.startswith("cd .."):
            self.dirstack.pop()
        elif command.startswith("cd "):
            # push dir "X" in dirstack
            curdir = self.dirstack[-1]
            self.dirstack.append(curdir["entries"][command[3:].strip()])
        elif command.startswith("ls"):
            # nothing to do here
            ...
        else:
            raise Exception(f"unknown command {command}")

    def record_entry(self, entry):
        pass
