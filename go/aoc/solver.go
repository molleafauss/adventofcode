package aoc

import (
	"bufio"
	"fmt"
	"os"
	"time"
)

type Solver interface {
	// Parse a line of the input
	Parse(line string)
	// Solve the puzzle
	Solve() (*string, *string)
}
type SolverFactory = func(string) Solver

func solveAll(year string, factory SolverFactory) {
	for day := range 26 {
		solveDay(year, fmt.Sprintf("day%02d", day+1), factory)
	}
}

func solveDay(year string, day string, factory SolverFactory) {
	Info("== Solving %s ==", day)

	// assume 'input' is a directory in the current directory
	testFile := fmt.Sprintf("inputs/%s/%s/test.txt", year, day)
	// check if solver return anything
	solver := factory(day)
	if solver == nil {
		Error("No solver available for day: %s", day)
		os.Exit(-1)
	}
	solve(testFile, solver)

	inputFile := fmt.Sprintf("inputs/%s/%s/input.txt", year, day)
	solve(inputFile, factory(day))
}

func solve(file string, solver Solver) {
	var expectedPart1 string
	var expectedPart2 string
	f, err := os.Open(file)
	if err != nil {
		Error("file %s does not exist", file)
		os.Exit(-1)
	}
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		if line[:15] == "result part 1: " {
			expectedPart1 = line[15:]
		} else if line[:15] == "result part 2: " {
			expectedPart2 = line[15:]
		} else {
			solver.Parse(line)
		}
	}

	t0 := time.Now()
	part1, part2 := solver.Solve()
	t1 := time.Now()
	Info("File %s: %.3sec", file, t1.Sub(t0))
	if part1 != nil {
		if *part1 == expectedPart1 {
			Info("PART 1 - found expected result: %s = %s", expectedPart1, part1)
		} else {
			Error("ERROR - part 1 result is incorrect: expected %s, actual %s",
				expectedPart1, part1)
		}
	}

	if part2 != nil {
		if *part2 == expectedPart2 {
			Info("PART 2 - found expected result: %s = %s", expectedPart2, part2)
		} else {
			Error("ERROR - part 2 result is incorrect: expected %s, actual %s",
				expectedPart2, part2)
		}
	}
}

func Run(year string, factory SolverFactory) {
	if len(os.Args) < 2 {
		fmt.Println("Please specify a day to resolve like 'day03'")
		os.Exit(-1)
	}
	day := os.Args[1]
	if day == "all" {
		solveAll(year, factory)
	} else {
		solveDay(year, day, factory)
	}
}
