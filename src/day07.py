from adc import Solver


# https://adventofcode.com/2022/day/7

LIMIT = 100000

class Solution(Solver):
    def __init__(self):
        self.tree = {"name": "", "entries": {}}
        self.dirstack = [self.tree]
        # total size of dirs smaller than limit
        self.total_size = 0

    def parse(self, line: str):
        if line.startswith("$ "):
            self.handle_command(line[2:])
        else:
            self.record_entry(line)

    def solve(self):
        # lovely tree walk
        def dir_size(dir):
            my_size = 0
            subdirs = []
            for e in dir["entries"].values():
                if e["dir"]:
                    my_size += dir_size(e)
                else:
                    my_size += e["size"]
            if my_size < LIMIT:
                self.total_size += my_size
            return my_size

        size = dir_size(self.tree)
        if size < LIMIT:
            self.total_size += size
        print(f"Found size for root: {size}")
        print(f"[1] Found small dir sizes: {self.total_size}")

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
        (size, name) = entry.split(" ", maxsplit=2)
        if size == "dir":
            entry = {"name": name, "dir": True, "entries": {}}
        else:
            entry = {"name": name, "dir": False, "size": int(size)}
        self.dirstack[-1]["entries"][name] = entry
