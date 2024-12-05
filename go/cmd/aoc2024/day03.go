package main

import (
	"aoc/aoc"
	"fmt"
	"regexp"
	"slices"
	"sort"
	"strconv"
	"strings"
)

type day03 struct {
	lastEnabled bool
	part1       int
	part2       int
}

func Day03() *day03 {
	return &day03{
		lastEnabled: true,
	}
}

func (solver *day03) Parse(line string) {
	aoc.Info("Parsing line: (%d) %s", len(line), line)
	n := 0
	switches := findSwitches(line, solver.lastEnabled)
	aoc.Info("switches: %s", switches)
	for n != -1 {
		next, val := findNextMul(n, line)
		if next == -1 {
			break
		}
		isEnabled := isEnabled(switches, next)
		solver.part1 += val
		if isEnabled {
			solver.part2 += val
		}
		aoc.Info("Found %d @ %d [add? %v]", val, next, isEnabled)
		n = next
	}
	solver.lastEnabled = switches[len(switches)-1].flag
}

var MATCHER = regexp.MustCompile("mul\\((\\d+),(\\d+)\\)")

func findNextMul(n int, line string) (int, int) {
	result := MATCHER.FindStringSubmatchIndex(line[n:])
	if result == nil {
		return -1, 0
	}
	if len(result) != 6 {
		aoc.Error("Parsing error - no 3 values found: %s (%s)", line, result)
		return n + result[1], 0
	}
	// these should not fail, as regexp looks for digits
	a, _ := strconv.Atoi(line[n+result[2] : n+result[3]])
	b, _ := strconv.Atoi(line[n+result[4] : n+result[5]])
	if a >= 1000 || b >= 1000 {
		// ignoring too big numbers
		return -1, 0
	}
	return n + result[1], a * b
}

type toggle struct {
	pos  int
	flag bool
}

func (me toggle) String() string {
	if me.flag {
		return fmt.Sprintf("enabled@%d ", me.pos)
	} else {
		return fmt.Sprintf("disabled@%d ", me.pos)
	}
}

func isEnabled(switches []toggle, next int) bool {
	for i := range switches {
		if switches[i].pos < next && i+1 == len(switches) {
			return switches[i].flag
		} else if switches[i].pos <= next && next < switches[i+1].pos {
			return switches[i].flag
		}
	}
	panic(fmt.Sprintf("Can't find %d in switches?? %s", next, switches))
}

func findSwitches(line string, lastEnabled bool) []toggle {
	switches := make([]toggle, 0)
	i := 0
	for {
		n := strings.Index(line[i:], "do()")
		if n == -1 {
			break
		}
		switches = append(switches, toggle{i + n, true})
		i += n + 4
	}

	i = 0
	for {
		n := strings.Index(line[i:], "don't()")
		if n == -1 {
			break
		}
		switches = append(switches, toggle{i + n, false})
		i += n + 7
	}

	sort.Slice(switches, func(i, j int) bool {
		return switches[i].pos < switches[j].pos
	})

	if len(switches) == 0 {
		// return a one-entry list with the last enabled
		return append(switches, toggle{0, lastEnabled})
	} else if switches[0].pos > 0 && switches[0].flag != lastEnabled {
		// add the last enabled at pos 0
		return slices.Insert(switches, 0, toggle{0, lastEnabled})
	} else if switches[0].pos > 0 && switches[0].flag == lastEnabled {
		// bring back initial position to 0
		switches[0].pos = 0
	}

	return switches
}

func (solver *day03) Solve() (*string, *string) {
	part1 := strconv.Itoa(solver.part1)
	part2 := strconv.Itoa(solver.part2)
	return &part1, &part2
}
