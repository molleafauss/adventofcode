package main

import (
	"aoc/aoc"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

type day05 struct {
	rules   map[int][]int
	updates bool
	part1   int
	part2   int
}

func Day05() aoc.Solver {
	return &day05{
		rules:   make(map[int][]int),
		updates: false,
	}
}

func (solver *day05) Parse(line string) {
	if solver.updates {
		checkUpdate(solver, line)
	} else if line == "" {
		solver.updates = true
	} else {
		addRule(solver.rules, line)
	}
}

func addRule(rules map[int][]int, line string) {
	parts := strings.Split(line, "|")
	if len(parts) != 2 {
		panic("bad line: " + line)
	}
	before, err := strconv.Atoi(parts[0])
	if err != nil {
		panic("bad before: " + line)
	}
	after, err := strconv.Atoi(parts[1])
	if err != nil {
		panic("bad after: " + line)
	}
	list := rules[before]
	if list == nil {
		list = []int{after}
	} else {
		list = append(list, after)
	}
	rules[before] = list
}

func checkUpdate(solver *day05, line string) {
	numbers := convertToNumbers(line)
	part1 := findMiddle(numbers)
	swaps := 0
	// walk froward on each number and check backward if any number should be after this one
	for i := 0; i < len(numbers); {
		current := numbers[i]
		rules := solver.rules[numbers[i]]
		if rules == nil {
			aoc.Debug("pos [%d] No rules for %d", i, current)
			i++
			continue
		}
		for j := i - 1; j >= 0; j-- {
			prev := numbers[j]
			if slices.Index(rules, prev) != -1 {
				// if a rule is violated, rearrange the numbers by placing the current in the expected order, and reset
				// i to the swapped position
				aoc.Debug("%s violates rules %d before %d", line, current, prev)
				numbers[j], numbers[i] = numbers[i], numbers[j]
				i = j - 1
				swaps++
				break
			}
		}
		i++
	}
	if swaps == 0 {
		solver.part1 += part1
		aoc.Info("[1] Sequence is ok: %s - middle %d - part 1 %d", line, part1, solver.part1)
	} else {
		middle := findMiddle(numbers)
		solver.part2 += findMiddle(numbers)
		aoc.Info("[2] Sequence had %d swaps - new middle is %d - final sequence %v", swaps, middle, numbers)
	}
}

func findMiddle(numbers []int) int {
	// ensure number count is odd
	if len(numbers)%2 == 0 {
		panic(fmt.Sprintf("Line has even numbers? %v (%d)", numbers, len(numbers)))
	}
	return numbers[len(numbers)/2]
}

func convertToNumbers(line string) []int {
	parts := strings.Split(line, ",")
	numbers := make([]int, len(parts))
	for i, val := range parts {
		num, err := strconv.Atoi(val)
		if err != nil {
			panic("bad num: " + val)
		}
		numbers[i] = num
	}
	return numbers
}

func (solver *day05) Solve() (*string, *string) {
	part1 := strconv.Itoa(solver.part1)
	part2 := strconv.Itoa(solver.part2)
	return &part1, &part2
}
