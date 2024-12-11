package main

import (
	"aoc/aoc"
	"strconv"
	"strings"
)

type day06 struct {
	room     [][]string
	guardPos aoc.GridPos
	guardDir int
	width    int
	height   int
}

func Day06() aoc.Solver {
	return &day06{
		room:     make([][]string, 0),
		guardDir: 0,
	}
}

func (solver *day06) Parse(line string) {
	solver.room = append(solver.room, strings.Split(line, ""))

	pos := strings.Index(line, "^")
	if pos != -1 {
		solver.guardPos = aoc.RowColToGridPos(pos, solver.height)
	}

	solver.height++
	if solver.width == 0 {
		solver.width = len(line)
	} else if solver.width != len(line) {
		panic("Map is not square?")
	}
}

var dirs = [4]aoc.GridPos{aoc.MOVE_D, aoc.MOVE_R, aoc.MOVE_U, aoc.MOVE_L}

func (solver *day06) Solve() (*string, *string) {
	aoc.Info("Guard starting at position %v", solver.guardPos)
	// keep looping until the guard doesn't gtfo
	for {
		nextPos := solver.guardPos.Add(&dirs[solver.guardDir])
		inBounds := nextPos.InBounds(solver.width, solver.height)
		if inBounds && solver.room[nextPos.Row][nextPos.Col] == "#" {
			// rotate and continue
			solver.guardDir += 1
			if solver.guardDir >= len(dirs) {
				solver.guardDir = 0
			}
			continue
		}
		aoc.Info("Guard moving to %v", nextPos)
		// mark current guard pos as X and move to next
		solver.room[solver.guardPos.Row][solver.guardPos.Col] = "X"
		if !inBounds {
			break
		}
		solver.guardPos = nextPos
	}

	// count all X on the map
	walked := 0
	for row := 0; row < solver.height; row++ {
		for col := 0; col < solver.width; col++ {
			if solver.room[row][col] == "X" {
				walked += 1
			}
		}
	}

	part1 := strconv.Itoa(walked)
	return &part1, nil
}
