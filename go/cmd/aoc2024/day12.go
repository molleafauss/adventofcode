package main

import (
	"aoc/aoc"
	"container/list"
	"fmt"
	"strconv"
)

type plot struct {
	name    string
	visited bool
}

type day12 struct {
	grid   [][]plot
	width  int
	height int
}

func Day12() aoc.Solver {
	return &day12{
		grid: make([][]plot, 0),
	}
}

func (solver *day12) Parse(line string) {
	if solver.width == 0 {
		solver.width = len(line)
	} else if solver.width != len(line) {
		panic(fmt.Sprintf("Map is not square? %d %s", solver.width, line))
	}
	solver.grid = append(solver.grid, make([]plot, solver.width))
	for i, char := range line {
		solver.grid[solver.height][i] = plot{name: string(char), visited: false}
	}
	solver.height++
}

func (solver *day12) Solve() (*string, *string) {
	aoc.Info("Map is %dx%d", solver.width, solver.height)
	price := 0
	// starting from 0,0 we keep iterating until we find a non-visited space
	for {
		start := solver.findNextToVisit()
		if start == nil {
			break
		}
		plot, area, perimeter := solver.plotFence(start)
		price += area * perimeter
		aoc.Info("Found plot '%s': area %d, perimeter %d => price %d (%d total)", plot, area, perimeter, area*perimeter, price)
	}
	part1 := strconv.Itoa(price)
	return &part1, nil
}

func (solver *day12) findNextToVisit() *aoc.GridPos {
	for row := 0; row < solver.height; row++ {
		for col := 0; col < solver.width; col++ {
			if !solver.grid[row][col].visited {
				pos := aoc.RowColToGridPos(col, row)
				return &pos
			}
		}
	}
	return nil
}

func (solver *day12) plotFence(start *aoc.GridPos) (string, int, int) {
	queue := list.New()
	queue.PushBack(*start)
	plot := solver.grid[start.Row][start.Col].name
	visited := map[aoc.GridPos]bool{}
	area := 0
	perimeter := 0
	for queue.Len() > 0 {
		pos := queue.Remove(queue.Front()).(aoc.GridPos)
		if visited[pos] {
			continue
		}
		solver.grid[pos.Row][pos.Col].visited = true
		visited[pos] = true
		area++
		perimeter += 4
		for _, dir := range aoc.ALL_ORTHOGONAL {
			next := pos.Add(dir)
			if !next.InBounds(solver.width, solver.height) {
				continue
			}
			if solver.grid[next.Row][next.Col].name == plot {
				perimeter -= 1
				queue.PushBack(next)
			}
		}
	}
	return plot, area, perimeter
}
