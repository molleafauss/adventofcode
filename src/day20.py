from advent import Solver

# https://adventofcode.com/2022/day/20
# "easy" when you just remember to use the modulo list size (-1) when moving items. Handling the encryption key could
# probably also have used some modul-ing but python can handle those integers
FINAL = [1000, 2000, 3000]
ENCRYPTION_KEY = 811589153


class Solution(Solver):
    def __init__(self):
        self.data = []
        self.initial_order = 0
        self.size = 0

    def parse(self, line: str):
        self.data.append([int(line.strip()), self.initial_order])
        self.initial_order += 1
        self.size += 1

    def solve(self):
        # part 1
        result = self.mix(self.data.copy())
        total = self.coordinates(result)
        print(f"[1] Final coordinates: {total}")

        # part 2
        data = [[val * ENCRYPTION_KEY, pos] for val, pos in self.data]
        for i in range(10):
            data = self.mix(data)
        total = self.coordinates(data)
        print(f"[2] Final coordinates: {total}")

    def mix(self, data):
        original_idx = 0
        # print(f"Initial list: {data}")
        while original_idx < self.size:
            pos = self.find_original(data, original_idx)
            original_idx += 1
            if data[pos][0] == 0:
                # don't move
                continue
            new_pos = pos + data[pos][0]
            if new_pos < 0 or new_pos >= self.size:
                # one less because pop() will shorten the list
                new_pos %= (self.size - 1)
            val = data.pop(pos)
            if new_pos == 0 or new_pos == self.size:
                data.append(val)
            else:
                data.insert(new_pos, val)
            # print(f"{original_idx - 1}: {val[0]} moves {pos} -> {new_pos}: {data}")
        # print(f"Final list: {data}")
        return data

    def coordinates(self, data):
        zero = None
        for i in range(self.size):
            if data[i][0] == 0:
                zero = i
                break
        total = 0
        for pos in FINAL:
            val = data[(zero + pos) % self.size][0]
            total += val
            print(f"{pos}: {val} -> {total}")
        return total

    def find_original(self, data, idx):
        for i in range(self.size):
            if data[i][1] == idx:
                return i
        raise ValueError(f"{idx} not found in list?")

    def file_name(self):
        return "../files/day20-encrypted.txt"

    def test_data(self):
        return """1
2
-3
3
-2
0
4"""
