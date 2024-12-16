package main

import (
	"aoc/aoc"
	"container/list"
	"fmt"
	"strconv"
)

type day10 struct {
	width  int
	height int
	grid   [][]int
	zeros  []aoc.GridPos
}

func Day10() aoc.Solver {
	return &day10{
		grid:  [][]int{},
		zeros: []aoc.GridPos{},
	}
}

func (solver *day10) Parse(line string) {
	if solver.width == 0 {
		solver.width = len(line)
	} else if solver.width != len(line) {
		panic(fmt.Sprintf("Wrong line length at line %d?", solver.height))
	}
	row := make([]int, solver.width)
	for i, ch := range line {
		val, err := strconv.Atoi(string(ch))
		if err != nil {
			panic("Cannot parse line: " + line)
		}
		row[i] = val
		if val == 0 {
			solver.zeros = append(solver.zeros, aoc.GridPos{i, solver.height})
		}
	}
	solver.grid = append(solver.grid, row)
	solver.height++
}

func (solver *day10) Solve() (*string, *string) {
	score := 0
	aoc.Info("Map is %dx%d - Found %d zeros", solver.width, solver.height, len(solver.zeros))
	for _, start := range solver.zeros {
		score += solver.findTrails(start)
	}
	part1 := strconv.Itoa(score)
	return &part1, nil
}

func (solver *day10) findTrails(start aoc.GridPos) int {
	var queue = list.New()
	queue.PushBack(start)
	peaks := map[aoc.GridPos]bool{}
	for queue.Len() > 0 {
		spot := queue.Remove(queue.Front()).(aoc.GridPos)
		height := solver.grid[spot.Row][spot.Col]
		for _, dir := range aoc.ALL_ORTHOGONAL {
			pos := spot.Add(dir)
			if !pos.InBounds(solver.width, solver.height) {
				continue
			}
			newHeight := solver.grid[pos.Row][pos.Col]
			if newHeight != height+1 {
				continue
			}
			if newHeight == 9 {
				aoc.Debug("trail %s -> %s", start, pos)
				peaks[pos] = true
				continue
			}
			queue.PushBack(pos)
		}
	}
	aoc.Info("Found %d peaks starting from %s", len(peaks), start)
	return len(peaks)
}
