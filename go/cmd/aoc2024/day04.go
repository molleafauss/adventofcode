package main

import (
	"aoc/aoc"
	"fmt"
	"strconv"
)

type day04 struct {
	// horribly assuming that I do not have to deal with UTF8 in the input
	puzzle []string
	width  int
	height int
	part1  int
}

func Day04() *day04 {
	return &day04{
		puzzle: make([]string, 0),
	}
}

func (solver *day04) Parse(line string) {
	// straight line or matrix?
	solver.puzzle = append(solver.puzzle, line)
	solver.height += 1
	if solver.width == 0 {
		solver.width = len(line)
	} else if solver.width != len(line) {
		panic(fmt.Sprintf("Unexpected line lenght: got %d, expected %d", len(line), solver.width))
	}
}

func (solver *day04) Solve() (*string, *string) {
	for row := 0; row < len(solver.puzzle); row++ {
		for col := 0; col < len(solver.puzzle[row]); col++ {
			if solver.puzzle[row][col] == 'X' {
				checkPart1(solver, row, col)
			}
		}
	}
	part1 := strconv.Itoa(solver.part1)
	return &part1, nil
}

func checkPart1(solver *day04, row int, col int) {
	// navigate all directions
	for _, dir := range aoc.ALL_SURROUNDING {
		start := aoc.RowColToGridPos(col, row)
		if checkDir(solver, start, dir) {
			solver.part1 += 1
			aoc.Info("[%d] Found word at %v -> %v", solver.part1, start, dir)
		}
	}
}

const SEARCH = "MAS"

func checkDir(solver *day04, start aoc.GridPos, dir aoc.GridPos) bool {
	// star is already correct
	next := start
	for i := range SEARCH {
		next = next.Add(&dir)
		if !next.InBounds(solver.width, solver.height) {
			return false
		}
		if solver.puzzle[next.Row][next.Col] != SEARCH[i] {
			return false
		}
	}
	return true
}
