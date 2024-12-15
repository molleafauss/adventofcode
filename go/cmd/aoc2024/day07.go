package main

import (
	"aoc/aoc"
	"fmt"
	"strconv"
	"strings"
)

type day07 struct {
	part1 int
	part2 int
}

func Day07() aoc.Solver {
	return &day07{}
}

func (solver *day07) Parse(line string) {
	before, after, found := strings.Cut(line, ": ")
	if !found {
		panic("Error parsing input: " + line)
	}
	test, err := strconv.Atoi(before)
	if err != nil {
		panic("test value not a number: " + line)
	}
	ops := []int{}
	for _, op := range strings.Split(after, " ") {
		val, err := strconv.Atoi(op)
		if err != nil {
			panic("found invalid operator number: " + line)
		}
		ops = append(ops, val)
	}
	if len(ops) < 2 {
		panic(fmt.Sprintf("Too few operands? %s", ops))
	}

	var opsPart1 = []func(int, int) int{add, mul}
	var opsPart2 = []func(int, int) int{add, mul, join}
	if checkOp("[1]", ops, test, opsPart1) {
		solver.part1 += test
		solver.part2 += test
		// no need to check with concat, what works for part 1 works for part 2
		return
	}
	if checkOp("[2]", ops, test, opsPart2) {
		solver.part2 += test
	}
}

func checkOp(phase string, vals []int, expected int, ops []func(int, int) int) bool {
	for _, op := range ops {
		if recurseOp(op(vals[0], vals[1]), 2, vals, ops, expected) {
			aoc.Info("%s %d found %d", phase, vals, expected)
			return true
		}
	}
	return false
}

func recurseOp(accum int, pos int, vals []int, ops []func(int, int) int, expected int) bool {
	// already bigger? bail
	if accum > expected {
		return false
	}
	// we at end? then return anyway, check if value is expected
	if pos == len(vals) {
		return accum == expected
	}
	for _, op := range ops {
		if recurseOp(op(accum, vals[pos]), pos+1, vals, ops, expected) {
			return true
		}
	}
	return false
}

func add(a int, b int) int { return a + b }

func mul(a int, b int) int { return a * b }

func join(left int, right int) int {
	val, err := strconv.Atoi(fmt.Sprintf("%d%d", left, right))
	if err != nil {
		panic(err)
	}
	return val
}

func (solver *day07) Solve() (*string, *string) {
	part1 := strconv.Itoa(solver.part1)
	part2 := strconv.Itoa(solver.part2)
	return &part1, &part2
}
