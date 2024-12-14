package main

import (
	"aoc/aoc"
	"fmt"
	"strconv"
	"strings"
)

type day07 struct {
	part1 int
}

func Day07() *day07 {
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
	if checkOp(ops, test) {
		aoc.Info("Found valid operation: %s", line)
		solver.part1 += test
	}
}

func checkOp(ops []int, test int) bool {
	if len(ops) < 2 {
		panic(fmt.Sprintf("Too few operands? %s", ops))
	}

	return recurseOp(ops[0]+ops[1], 2, ops, test) || recurseOp(ops[0]*ops[1], 2, ops, test)
}

func recurseOp(accum int, pos int, ops []int, test int) bool {
	// we at end with correct value?
	if pos == len(ops) && accum == test {
		return true
	}
	if pos < len(ops) {
		return recurseOp(accum+ops[pos], pos+1, ops, test) || recurseOp(accum*ops[pos], pos+1, ops, test)
	}
	return false
}

func (solver *day07) Solve() (*string, *string) {
	part1 := strconv.Itoa(solver.part1)
	return &part1, nil
}
