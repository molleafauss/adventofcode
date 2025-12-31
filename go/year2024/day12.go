package main

import (
	"adventofcode/utils"
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

func init() {
	utils.RegisterSolver("2024", "day12", func() utils.Solver {
		return &day12{
			grid: make([][]plot, 0),
		}
	})
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
	utils.Info("Map is %dx%d", solver.width, solver.height)
	price1 := 0
	price2 := 0
	// starting from 0,0 we keep iterating until we find a non-visited space
	for {
		start := solver.findNextToVisit()
		if start == nil {
			break
		}
		fence := solver.plotFence(start)
		price1 += fence.area * fence.perimeter
		price2 += fence.area * fence.sides
		utils.Info("Found plot '%s': area %d, perimeter %d, sides %d => prices %d/%d (%d/%d total)", fence.plot, fence.area,
			fence.perimeter, fence.sides, fence.area*fence.perimeter, fence.area*fence.sides, price1, price2)
	}
	part1 := strconv.Itoa(price1)
	part2 := strconv.Itoa(price2)
	return &part1, &part2
}

func (solver *day12) findNextToVisit() *utils.GridPos {
	for row := 0; row < solver.height; row++ {
		for col := 0; col < solver.width; col++ {
			if !solver.grid[row][col].visited {
				pos := utils.RowColToGridPos(col, row)
				return &pos
			}
		}
	}
	return nil
}

type fence struct {
	plot      string
	area      int
	perimeter int
	sides     int
}

func (solver *day12) plotFence(start *utils.GridPos) fence {
	queue := list.New()
	queue.PushBack(*start)
	plot := solver.grid[start.Row][start.Col].name
	visited := map[utils.GridPos]bool{}
	area := 0
	perimeter := 0
	for queue.Len() > 0 {
		pos := queue.Remove(queue.Front()).(utils.GridPos)
		if visited[pos] {
			continue
		}
		solver.grid[pos.Row][pos.Col].visited = true
		visited[pos] = true
		area++
		perimeter += 4
		for _, dir := range utils.ALL_ORTHOGONAL {
			next := pos.Add(dir)
			if !next.InBounds(solver.width, solver.height) {
				continue
			}
			if solver.grid[next.Row][next.Col].name == plot {
				// remove 1 from perimeter for every plot of same type surrounding this
				perimeter -= 1
				queue.PushBack(next)
			}
		}
	}
	// sides calculation here, based on what's in visited
	sides := countSides(visited)
	if sides%2 == 1 {
		panic("Even sides found?")
	}
	return fence{plot, area, perimeter, sides}
}

// precalculated "corner" checks
var DIRS = [4][3]utils.GridPos{
	{utils.MOVE_D, utils.MOVE_DR, utils.MOVE_R},
	{utils.MOVE_R, utils.MOVE_UR, utils.MOVE_U},
	{utils.MOVE_U, utils.MOVE_UL, utils.MOVE_L},
	{utils.MOVE_L, utils.MOVE_DL, utils.MOVE_D},
}

func countSides(shape map[utils.GridPos]bool) int {
	// special cases - 1 or 2 is always 4 sides, however arranged
	if len(shape) == 1 || len(shape) == 2 {
		return 4
	}

	// sides = count corners.
	// corner = check every position in the shape and verify if:
	// - all 3 items in DIR are outside (= not present in shape)
	// - the middle is outside and the other two are inside
	corners := 0
	for point := range shape {
		corners += countCorners(point, &shape)
	}

	return corners
}

func countCorners(point utils.GridPos, shape *map[utils.GridPos]bool) int {
	corners := 0
	for _, dirs := range DIRS {
		_, has1 := (*shape)[point.Add(dirs[0])]
		_, has2 := (*shape)[point.Add(dirs[1])]
		_, has3 := (*shape)[point.Add(dirs[2])]
		// 1) x .     2) x x     3) . x
		//    . .        x .        x . (touching corners)
		if (!has1 && !has2 && !has3) || (has1 && !has2 && has3) || (!has1 && has2 && !has3) {
			utils.Info("Corner found: %s - %s [%v %v %v]", point, dirs, has1, has2, has3)
			corners++
		}
	}
	return corners
}
