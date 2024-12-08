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
}

func Day05() *day05 {
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
	// walk froward on each number and check backward if any number should be after this one
	for i := range numbers {
		rules := solver.rules[numbers[i]]
		if rules == nil {
			aoc.Info("No rules for %d", numbers[i])
			continue
		}
		for j := i - 1; j >= 0; j-- {
			prev := numbers[j]
			if slices.Index(rules, prev) != -1 {
				aoc.Warn("%s violates rules %d before %d", line, numbers[i], prev)
				return
			}
		}
	}
	// ensure number count is odd
	if len(numbers)%2 == 0 {
		panic(fmt.Sprintf("Line has even numbers? %s (%d)", line, len(numbers)))
	}
	middle := numbers[len(numbers)/2]
	solver.part1 += middle
	aoc.Info("Sequence is ok: %s - middle %d - part 1 %d", line, middle, solver.part1)
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
	return &part1, nil
}
