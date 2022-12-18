from advent import Solver
import re

# https://adventofcode.com/2022/day/15
RE_SENSOR = re.compile(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")


def m_distance(sensor, beacon):
    return abs(sensor[0] - beacon[0]) + abs(sensor[1] - beacon[1])


class Solution(Solver):
    def __init__(self):
        self.sensors = []
        self.y = 2000000
        self.area = 4000000

    def parse(self, line: str):
        if not line:
            return
        if line.startswith("part 1: "):
            self.y = int(line[8:])
            return
        if line.startswith("part 2: "):
            self.area = int(line[8:])
            return
        mo = RE_SENSOR.match(line)
        assert mo
        sensor = {
            "index": len(self.sensors) + 1,
            "position": (int(mo.group(1)), int(mo.group(2))),
            "beacon": (int(mo.group(3)), int(mo.group(4)))
        }
        sensor["distance"] = m_distance(sensor["position"], sensor["beacon"])
        self.sensors.append(sensor)

    def solve(self):
        print(f"Finding invalid beacon positions at line {self.y} over {len(self.sensors)} sensors")
        invalid = set()
        # if line 10 crosses beacon range then mark invalid positions
        for sensor in self.sensors:
            t_distance = m_distance(sensor["position"], (sensor["position"][0], self.y))
            if t_distance > sensor["distance"]:
                continue
            print(f"Sensor {sensor} crosses line {self.y} ({t_distance})")
            i = 0
            while t_distance + i <= sensor["distance"]:
                invalid.add(sensor["position"][0] + i)
                invalid.add(sensor["position"][0] - i)
                i += 1
            if sensor["beacon"][1] == self.y:
                invalid.remove(sensor["beacon"][0])

        print(f"[1] invalid set contains {len(invalid)} elements")

    def file_name(self):
        return "../files/day15-sensors.txt"

    def test_data(self):
        return """Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
part 1: 10
part 2: 20
"""