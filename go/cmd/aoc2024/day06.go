package main

import (
	"aoc/aoc"
	"strconv"
	"strings"
)

type day06 struct {
	room     [][]string
	startPos aoc.GridPos
	width    int
	height   int
}

func Day06() aoc.Solver {
	return &day06{
		room: make([][]string, 0),
	}
}

func (solver *day06) Parse(line string) {
	solver.room = append(solver.room, strings.Split(line, ""))

	pos := strings.Index(line, "^")
	if pos != -1 {
		solver.startPos = aoc.RowColToGridPos(pos, solver.height)
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
	aoc.Info("Guard starting at position %v - map is (%dx%d)", solver.startPos, solver.width, solver.height)
	// keep looping until the guard doesn't gtfo
	walkMap(solver)

	// count all X on the map - for part 2 save them all as position for possible obstruction
	walked := 0
	obstructions := make([]aoc.GridPos, 0)
	for row := 0; row < solver.height; row++ {
		for col := 0; col < solver.width; col++ {
			if solver.room[row][col] == "X" {
				walked += 1
				obstructions = append(obstructions, aoc.RowColToGridPos(col, row))
			}
		}
	}

	looping := 0
	for _, o := range obstructions {
		solver.room[o.Row][o.Col] = "#"
		if walkMap(solver) {
			aoc.Info("Guard will loop if obstruction placed at %v", o)
			looping += 1
		}
		solver.room[o.Row][o.Col] = "."
	}

	part1 := strconv.Itoa(walked)
	part2 := strconv.Itoa(looping)
	return &part1, &part2
}

type GuardPosition struct {
	position  aoc.GridPos
	direction int
}

// walkMap will move the guard until it either exit the map (returning false) or loops (returning rue)
func walkMap(solver *day06) bool {
	guardDir := 0
	startPos := solver.startPos
	walked := make(map[GuardPosition]struct{})
	for {
		gp := GuardPosition{position: startPos, direction: guardDir}
		if _, ok := walked[gp]; ok {
			aoc.Info("Guard will loop after %d steps", len(walked))
			return true
		}
		walked[gp] = struct{}{}

		nextPos := startPos.Add(dirs[guardDir])
		inBounds := nextPos.InBounds(solver.width, solver.height)
		if inBounds && solver.room[nextPos.Row][nextPos.Col] == "#" {
			// rotate and continue
			guardDir += 1
			if guardDir >= len(dirs) {
				guardDir = 0
			}
			continue
		}
		aoc.Debug("Guard moving to %v", nextPos)
		// mark current guard pos as X and move to next
		solver.room[startPos.Row][startPos.Col] = "X"
		if !inBounds {
			return false
		}
		startPos = nextPos
	}
}
