package aoc

import (
	"fmt"
)

type Solver interface {
	// Parse a line of the input
	Parse(line string)
	// Solve the puzzle
	Solve() (*string, *string)
}
type SolverFactory = func() Solver

var solverMap = make(map[string]SolverFactory)

func RegisterSolver(year string, day string, factory SolverFactory) {
	solverMap[fmt.Sprintf("%s/%s", year, day)] = factory
}

func CreateSolver(year string, day string) Solver {
	factory, ok := solverMap[fmt.Sprintf("%s/%s", year, day)]
	if !ok {
		return nil
	}
	return factory()
}
