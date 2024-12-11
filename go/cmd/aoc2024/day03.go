package main

import (
	"aoc/aoc"
	"regexp"
	"strconv"
	"strings"
)

type day03 struct {
	enabled bool
	part1   int
	part2   int
}

func Day03() aoc.Solver {
	return &day03{
		enabled: true,
	}
}

func (solver *day03) Parse(line string) {
	aoc.Info("Parsing line: (%d) %s", len(line), line)
	for _, match := range MATCHER.FindAllString(line, -1) {
		if match == "do()" {
			aoc.Info("Enabling parser")
			solver.enabled = true
			continue
		} else if match == "don't()" {
			aoc.Info("Disabling parser")
			solver.enabled = false
			continue
		}
		// for sure this should be a "mul(X,Y)"
		val := multiply(match)
		solver.part1 += val
		if solver.enabled {
			solver.part2 += val
		}
		aoc.Info("Found multiplication %s -> %d (enabled? %v) | part1 %d | part2 %d",
			match, val, solver.enabled, solver.part1, solver.part2)
	}
}

var MATCHER = regexp.MustCompile("mul\\(\\d+,\\d+\\)|do\\(\\)|don't\\(\\)")

func multiply(match string) int {
	if !strings.HasPrefix(match, "mul(") {
		aoc.Warn("Wrong mul match? %s", match)
		return 0
	} else if !strings.HasSuffix(match, ")") {
		aoc.Warn("Wrong mul match? %s", match)
		return 0
	}
	parts := strings.Split(match[4:len(match)-1], ",")
	if len(parts) != 2 {
		aoc.Warn("Wrong mul match - no 2 parts? %s", match)
		return 0
	}
	x, err := strconv.Atoi(parts[0])
	if err != nil || x > 1000 {
		aoc.Warn("Invalid 1st operand? %s", match)
		return 0
	}
	y, err := strconv.Atoi(parts[1])
	if err != nil || y > 1000 {
		aoc.Warn("Invalid 2st operand? %s", match)
		return 0
	}
	return x * y
}

func (solver *day03) Solve() (*string, *string) {
	part1 := strconv.Itoa(solver.part1)
	part2 := strconv.Itoa(solver.part2)
	return &part1, &part2
}
