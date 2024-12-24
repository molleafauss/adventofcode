package main

import (
	"aoc/aoc"
	"fmt"
	"strconv"
)

const WALL = 1
const GOOD = 2

type day15 struct {
	goods     map[aoc.GridPos]byte
	robot     aoc.GridPos
	program   string
	warehouse bool
	width     int
	height    int
}

func Day15() aoc.Solver {
	return &day15{
		goods:     make(map[aoc.GridPos]byte),
		warehouse: true,
	}
}

func (solver *day15) Parse(line string) {
	if line == "" {
		solver.warehouse = false
		return
	}
	if solver.warehouse {
		solver.addGoods(line)
	} else {
		solver.program = solver.program + line
	}
}

func (solver *day15) addGoods(line string) {
	if solver.width == 0 {
		solver.width = len(line)
	} else if len(line) != solver.width {
		panic(fmt.Sprintf("bad line length at %d: %s", solver.height, line))
	}
	for i := range line {
		switch line[i] {
		case '#':
			solver.goods[aoc.RowColToGridPos(i, solver.height)] = WALL
		case 'O':
			solver.goods[aoc.RowColToGridPos(i, solver.height)] = GOOD
		case '@':
			solver.robot = aoc.RowColToGridPos(i, solver.height)
		}
	}
	solver.height++
}

func (solver *day15) Solve() (*string, *string) {
	aoc.Info("Robot start: %s instructions: %d", solver.robot, len(solver.program))

	for i := range solver.program {
		switch solver.program[i] {
		case '^':
			solver.moveRobot(aoc.MOVE_D)
		case '>':
			solver.moveRobot(aoc.MOVE_R)
		case 'v':
			solver.moveRobot(aoc.MOVE_U)
		case '<':
			solver.moveRobot(aoc.MOVE_L)
		}
	}

	solver.printMap()
	gps := calculateGps(solver.goods)
	part1 := strconv.Itoa(gps)
	return &part1, nil
}

func calculateGps(warehouse map[aoc.GridPos]byte) int {
	gps := 0
	for pos := range warehouse {
		if warehouse[pos] != GOOD {
			continue
		}
		gps += pos.Row*100 + pos.Col
	}
	return gps
}

func (solver *day15) moveRobot(dir aoc.GridPos) {
	next := solver.robot.Add(dir)
	val, exists := solver.goods[next]
	if !exists {
		// empty space, just move
		solver.robot = next
		return
	}
	if val == WALL {
		// no moves
		return
	}
	// follow goods until an empty space or a wall
	end := next.Add(dir)
	for ; end.InBounds(solver.width, solver.height); end = end.Add(dir) {
		val, exists := solver.goods[end]
		if !exists {
			break
		}
		if val == WALL {
			// no moves
			return
		}
		// val == GOOD, continue
	}
	// move all goods from the warehouse = move next to end and keep the other at the same place
	solver.goods[end] = GOOD
	delete(solver.goods, next)
	solver.robot = next
}

func (solver *day15) printMap() {
	for row := range solver.height {
		line := ""
		for col := range solver.width {
			pos := aoc.RowColToGridPos(col, row)
			if pos == solver.robot {
				line += "@"
				continue
			}
			val, exists := solver.goods[pos]
			if !exists {
				line += "."
			} else if val == GOOD {
				line += "O"
			} else {
				line += "#"
			}
		}
		fmt.Println(line)
	}
}
