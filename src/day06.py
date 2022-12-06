from utils import Parser, loader


# https://adventofcode.com/2022/day/6
def all_different(string):
    return len(set(string)) == len(string)


class Day06(Parser):

    def parse(self, line: str):
        start_of_packet = False
        start_of_message = False
        line = line.strip()
        i = 0
        while i + 14 <= len(line):
            # start_of_packet : 4 chars
            if not start_of_packet and all_different(line[i:i+4]):
                print(f"Found start-of-packet at {i + 4}")
                start_of_packet = True
            if not start_of_message and all_different(line[i:i+14]):
                print(f"Found start-of-message at {i + 14}")
                start_of_message = True
            if start_of_packet and start_of_message:
                return
            i += 1

    def solve(self):
        pass

    def file_name(self):
        return "../files/day06-signal.txt"

    def test_data(self):
        return """mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"""


if __name__ == '__main__':
    Day06().run_test()
    Day06().run()