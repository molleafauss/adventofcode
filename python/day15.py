import logging

from advent import Solver
from grid import Point
from dataclasses import dataclass
import re
import time

# https://adventofcode.com/2022/day/15
# The "very bovine" first method was slow for solving the 1st part but not working at all for the second due to the
# large numbers involved.
# In the end is still an iterative method, but solves the problem in a reasonable amount of time (~25 sec).
#
# Test input had different constraints - hence the extra parse lines to override them.
#
# bonus: I discovered python dataclasses. I will have to refactor everything


log = logging.getLogger("day.15")


RE_SENSOR = re.compile(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")


@dataclass
class Sensor:
    index: int
    position: Point
    beacon: Point
    distance: int = None


@dataclass
class Invalid:
    segments: list
    beacons: set


@dataclass
class Segment:
    start: int
    end: int


def m_distance(sensor, beacon):
    return abs(sensor.x - beacon.x) + abs(sensor.y - beacon.y)


def merge(segments, x_start, x_end):
    assert x_start <= x_end
    i = 0
    # insert segment
    while i < len(segments) and segments[i].start < x_start:
        i += 1
    if i == len(segments):
        segments.append(Segment(x_start, x_end))
    else:
        segments.insert(i, Segment(x_start, x_end))

    # now merge segments if they overlap
    i = 0
    while i < len(segments) - 1:
        s0 = segments[i]
        s1 = segments[i + 1]
        if s1.start <= s0.end + 1:
            s0.end = max(s0.end, s1.end)
            segments.pop(i + 1)
            assert s0.end >= s0.start
        else:
            i += 1


def segment_length(segments):
    return sum([seg.end - seg.start + 1 for seg in segments])


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
        sensor = Sensor(
            len(self.sensors) + 1,
            Point(int(mo.group(1)), int(mo.group(2))),
            Point(int(mo.group(3)), int(mo.group(4)))
        )
        sensor.distance = m_distance(sensor.position, sensor.beacon)
        self.sensors.append(sensor)

    def solve(self):
        log.debug(f"We have {len(self.sensors)} sensors")
        log.debug(f"Finding invalid beacon positions at line {self.y}")
        segments, beacons = self.check_line(self.y)
        segment_size = segment_length(segments) - len(beacons)
        log.info(f"[1] invalid set contains {segment_size} elements")

        log.debug(f"Finding possible real beacon positions in area 0-{self.area}")
        t0 = time.time()
        frequency = 0
        for y in range(self.area + 1):
            if (y % 100000) == 0:
                print(f"Checking line {y}/{self.area}")
            segments, beacons = self.check_line(y)
            # ignore beacons - mark them as not valid
            for b in beacons:
                merge(segments, b, b)
            if len(segments) == 1 and segments[0].start <= 0 and segments[0].end >= self.area:
                continue
            log.debug(f"Found something at y: {y} - {segments}?")
            x = segments[0].end + 1
            assert x == segments[1].start - 1
            t1 = time.time()
            frequency = x * 4000000 + y
            log.info(f"[2] Found frequency: {frequency} in {t1 - t0} sec")
            break
        return str(segment_size), str(frequency)

    def check_line(self, y):
        segments = []
        beacons = set()
        # find all invalid segment in line self.y
        for sensor in self.sensors:
            if y < sensor.position.y - sensor.distance or sensor.position.y + sensor.distance < y:
                # not in range
                continue
            delta = sensor.distance - abs(y - sensor.position.y)
            merge(segments, sensor.position.x - delta, sensor.position.x + delta)
            if sensor.beacon.y == y:
                beacons.add(sensor.beacon.x)
        return segments, beacons
