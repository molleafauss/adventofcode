package utils

import "fmt"

type Solver interface {
	// Parse a line of the input
	Parse(line string)
	// Solve the puzzle
	Solve() (*string, *string)
}
type SolverFactory = func() Solver

var solverRegistry = make(map[string]SolverFactory)

func CreateSolver(year string, day string) Solver {
	key := fmt.Sprintf("%s/%s", year, day)
	factory, exists := solverRegistry[key]
	if !exists {
		return nil
	}
	return factory()
}

func RegisterSolver(year string, day string, factory SolverFactory) {
	key := fmt.Sprintf("%s/%s", year, day)
	solverRegistry[key] = factory
}
