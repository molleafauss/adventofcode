package main

import (
	"aoc/aoc"
	"fmt"
	"strconv"
	"strings"
)

const UP = 1

const FLAT = 0

const DOWN = -1

const MAX_DELTA = 3

type day02 struct {
	part1Safe int
	part2Safe int
}

func Day02() *day02 {
	return &day02{}
}

func (solver *day02) Parse(line string) {
	direction := FLAT
	prev := 0
	tolerance := 1
	for i, val := range strings.Split(line, " ") {
		cur, err := strconv.Atoi(val)
		if err != nil {
			aoc.Error("Unable to parse line: %s", line)
			return
		}

		if i == 0 {
			prev = cur
			continue
		}
		if i == 1 && cur > prev {
			direction = UP
		} else if i == 1 && cur < prev {
			direction = DOWN
		}

		err = checkLevels(direction, prev, cur)
		if err != nil && tolerance > 0 {
			// don't move the previous "good" read and skip this bad one
			tolerance--
			continue
		} else if err != nil && tolerance == 0 {
			aoc.Warn("%s - %s", err.Error(), line)
			return
		}
		prev = cur
	}
	if tolerance > 0 {
		solver.part1Safe += 1
	}
	solver.part2Safe += 1
	aoc.Info("(safe - tolerance: %d) [%d]/[%d] %s", tolerance, solver.part1Safe, solver.part2Safe, line)
}

func checkLevels(direction int, prev int, cur int) error {
	delta := cur - prev
	if delta == 0 {
		return fmt.Errorf("(unsafe) Found flat increase: %d -> %d", prev, cur)
	}
	if delta < -MAX_DELTA || delta > MAX_DELTA {
		return fmt.Errorf("(unsafe) found excessive delta %d: %d -> %d", delta, prev, cur)
	}

	if direction == UP && delta < 0 {
		return fmt.Errorf("(unsafe) found conflicting directions, expected up got %d: %d -> %d",
			delta, prev, cur)
	} else if direction == DOWN && delta > 0 {
		return fmt.Errorf("(unsafe) found conflicting directions, expected down got %d: %d -> %d",
			delta, prev, cur)
	}
	return nil
}

func (solver *day02) Solve() (*string, *string) {
	part1 := strconv.Itoa(solver.part1Safe)
	part2 := strconv.Itoa(solver.part2Safe)
	return &part1, &part2
}
