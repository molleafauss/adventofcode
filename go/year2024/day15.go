package main

import (
	"adventofcode/utils"
	"aoc/aoc"
	"container/list"
	"fmt"
	"strconv"
)

const WALL = 1
const GOOD = 2
const BOX_LEFT = 3
const BOX_RIGHT = 4

type warehouse struct {
	robot  aoc.GridPos
	goods  map[aoc.GridPos]byte
	width  int
	height int
}

func (w *warehouse) doProgram(program string) int {
	aoc.Info("Robot start: %s instructions: %d", w.robot, len(program))

	for i := range program {
		switch program[i] {
		case '^':
			w.moveRobot(aoc.MOVE_D)
		case '>':
			w.moveRobot(aoc.MOVE_R)
		case 'v':
			w.moveRobot(aoc.MOVE_U)
		case '<':
			w.moveRobot(aoc.MOVE_L)
		}
	}

	aoc.Info("End map")
	w.printMap()
	return calculateGps(w.goods)
}

func (w *warehouse) moveRobot(dir aoc.GridPos) {
	next := w.robot.Add(dir)
	val, exists := w.goods[next]
	if !exists {
		// empty space, just move the robot
		w.robot = next
		return
	}
	if val == WALL {
		// found a wall, robot won't move
		return
	}
	// map all moved good (and what they were)
	movedGoods := make(map[aoc.GridPos]byte)
	queue := list.New()
	queue.PushBack(next)
	// examine the list of goods to push
	for queue.Len() > 0 {
		pos := queue.Remove(queue.Front()).(aoc.GridPos)
		val, exists := w.goods[pos]
		switch {
		case !exists:
			// found an empty space - push everything that was found
			break
		case val == WALL:
			// found a wall - nothing will move
			return
		case val == GOOD:
			movedGoods[pos] = GOOD
			// check next space
			queue.PushBack(pos.Add(dir))
		case val == BOX_LEFT:
			if dir == aoc.MOVE_L {
				// if I want to move left it should always happen that I am pushing a RIGHT box side
				panic("Found move left by pushing a LEFT box side???")
			}
			movedGoods[pos] = BOX_LEFT
			if dir != aoc.MOVE_R {
				// don't add the right position (again) if we're moving in that direction
				queue.PushBack(pos.Add(dir))
			}
			nextRight := pos.Add(aoc.MOVE_R)
			movedGoods[nextRight] = BOX_RIGHT
			queue.PushBack(nextRight.Add(dir))
		case val == BOX_RIGHT:
			if dir == aoc.MOVE_R {
				// if I want to move right it should always happen that I am pushing a LEFT box side
				panic("Found move right by pushing a RIGHT box side???")
			}
			movedGoods[pos] = BOX_RIGHT
			if dir != aoc.MOVE_L {
				// don't add the left slot (again) if we're moving in that direction
				queue.PushBack(pos.Add(dir))
			}
			nextLeft := pos.Add(aoc.MOVE_L)
			movedGoods[nextLeft] = BOX_LEFT
			queue.PushBack(nextLeft.Add(dir))
		}
	}
	// move all goods found - first remove all positions then re-add them shifted
	for pos := range movedGoods {
		delete(w.goods, pos)
	}
	for pos := range movedGoods {
		w.goods[pos.Add(dir)] = movedGoods[pos]
	}
	w.robot = next
}

func (w *warehouse) printMap() {
	for row := range w.height {
		line := ""
		for col := range w.width {
			pos := aoc.RowColToGridPos(col, row)
			if pos == w.robot {
				line += "@"
				continue
			}
			val, exists := w.goods[pos]
			switch {
			case !exists:
				line += "."
			case val == WALL:
				line += "#"
			case val == GOOD:
				line += "O"
			case val == BOX_LEFT:
				line += "["
			case val == BOX_RIGHT:
				line += "]"
			}
		}
		fmt.Println(line)
	}
}

type day15 struct {
	program        string
	parsed         bool
	smallWarehouse warehouse
	bigWarehouse   warehouse
}

func init() {
	utils.RegisterSolver("2022", "day15", func() utils.Solver {
		return &day15{
			smallWarehouse: warehouse{
				goods: make(map[aoc.GridPos]byte),
			},
			bigWarehouse: warehouse{
				goods: make(map[aoc.GridPos]byte),
			},
			parsed: true,
		}
	})
}

func (solver *day15) Parse(line string) {
	if line == "" {
		solver.parsed = false
		return
	}
	if solver.parsed {
		solver.addGoods(line)
	} else {
		solver.program = solver.program + line
	}
}

func (solver *day15) addGoods(line string) {
	if solver.smallWarehouse.width == 0 {
		solver.smallWarehouse.width = len(line)
		solver.bigWarehouse.width = len(line) * 2
	} else if len(line) != solver.smallWarehouse.width {
		panic(fmt.Sprintf("bad line length at %d: %s", solver.smallWarehouse.height, line))
	}
	for i := range line {
		switch line[i] {
		case '#':
			solver.smallWarehouse.goods[aoc.RowColToGridPos(i, solver.smallWarehouse.height)] = WALL
			solver.bigWarehouse.goods[aoc.RowColToGridPos(i*2, solver.bigWarehouse.height)] = WALL
			solver.bigWarehouse.goods[aoc.RowColToGridPos(i*2+1, solver.bigWarehouse.height)] = WALL
		case 'O':
			solver.smallWarehouse.goods[aoc.RowColToGridPos(i, solver.smallWarehouse.height)] = GOOD
			solver.bigWarehouse.goods[aoc.RowColToGridPos(i*2, solver.bigWarehouse.height)] = BOX_LEFT
			solver.bigWarehouse.goods[aoc.RowColToGridPos(i*2+1, solver.bigWarehouse.height)] = BOX_RIGHT
		case '@':
			solver.smallWarehouse.robot = aoc.RowColToGridPos(i, solver.smallWarehouse.height)
			solver.bigWarehouse.robot = aoc.RowColToGridPos(i*2, solver.smallWarehouse.height)
		}
	}
	solver.smallWarehouse.height++
	solver.bigWarehouse.height++
}

func (solver *day15) Solve() (*string, *string) {
	part1 := strconv.Itoa(solver.smallWarehouse.doProgram(solver.program))
	part2 := strconv.Itoa(solver.bigWarehouse.doProgram(solver.program))
	return &part1, &part2
}

func calculateGps(warehouse map[aoc.GridPos]byte) int {
	gps := 0
	for pos := range warehouse {
		if warehouse[pos] == GOOD || warehouse[pos] == BOX_LEFT {
			gps += pos.Row*100 + pos.Col
		}
	}
	return gps
}
