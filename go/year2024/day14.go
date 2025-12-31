package year2024

import (
	"adventofcode/utils"
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

type day14 struct {
	robots []robot
	width  int
	height int
}
type robot struct {
	start utils.GridPos
	vel   utils.GridPos
}

func init() {
	utils.RegisterSolver("2024", "day14", func() utils.Solver {
		return &day14{
			robots: []robot{},
			width:  DEFAULT_WIDTH,
			height: DEFAULT_HEIGHT,
		}
	})
}

var RE_ROBOT = regexp.MustCompile(`p=(\d+),(\d+) v=(-?\d+),(-?\d+)`)

const DEFAULT_WIDTH = 101
const DEFAULT_HEIGHT = 103
const WIDTH_DIRECTIVE = "room width: "
const HEIGHT_DIRECTIVE = "room height: "
const TIME = 100

func (solver *day14) Parse(line string) {
	if strings.HasPrefix(line, WIDTH_DIRECTIVE) {
		solver.width, _ = strconv.Atoi(strings.TrimPrefix(line, WIDTH_DIRECTIVE))
		return
	}
	if strings.HasPrefix(line, HEIGHT_DIRECTIVE) {
		solver.height, _ = strconv.Atoi(strings.TrimPrefix(line, HEIGHT_DIRECTIVE))
		return
	}
	matches := RE_ROBOT.FindStringSubmatch(line)
	if matches == nil {
		panic("Line not matching?? " + line)
	}
	px, _ := strconv.Atoi(matches[1])
	py, _ := strconv.Atoi(matches[2])
	vx, _ := strconv.Atoi(matches[3])
	vy, _ := strconv.Atoi(matches[4])
	solver.robots = append(solver.robots, robot{start: utils.RowColToGridPos(px, py), vel: utils.RowColToGridPos(vx, vy)})
}

func (solver *day14) Solve() (*string, *string) {
	utils.Info("Moving %d robots", len(solver.robots))
	quadrants := []int{0, 0, 0, 0}
	tree := -1
	// arbitrarily only run width*height times.
	for time := 0; time < solver.width*solver.height; time++ {
		robotPositions := map[utils.GridPos]bool{}
		for _, robot := range solver.robots {
			travelX := robot.vel.Col * time
			travelY := robot.vel.Row * time
			endPos := robot.start.Add(utils.RowColToGridPos(travelX, travelY))
			// wrap around
			endPos.Row = wrap(endPos.Row, solver.height)
			endPos.Col = wrap(endPos.Col, solver.width)
			robotPositions[endPos] = true
			if time == TIME {
				q := findQuadrant(endPos, solver.width, solver.height)
				utils.Info("Robot %v ends at %s / quadrant %d", robot, endPos, q)
				if q != -1 {
					quadrants[q] += 1
				}
			}
		}
		if findTree(robotPositions) {
			printTree(robotPositions)
			tree = time
		}
	}

	utils.Info("Final quadrants: %v", quadrants)

	part1 := strconv.Itoa(quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3])
	part2 := strconv.Itoa(tree)
	return &part1, &part2
}

func findQuadrant(pos utils.GridPos, width int, height int) int {
	midX := width / 2
	midY := height / 2
	if pos.Col == midX || pos.Row == midY {
		return -1
	} else if pos.Row >= 0 && pos.Row < midY && pos.Col >= 0 && pos.Col < midX {
		return 0
	} else if pos.Row >= 0 && pos.Row < midY && pos.Col > midX && pos.Col <= width {
		return 1
	} else if pos.Row > midY && pos.Row <= height && pos.Col >= 0 && pos.Col < midX {
		return 2
	} else if pos.Row > midY && pos.Row <= height && pos.Col > midX && pos.Col <= width {
		return 3
	}
	panic("Unidentified position?? " + pos.String())
}

func wrap(pos int, size int) int {
	if pos < 0 {
		return size - ((-pos - 1) % size) - 1
	} else {
		return pos % size
	}
}

// find a subset of the tree - if I find the tip, ensure that it keep growing for a bit in the following rows
func findTree(robots map[utils.GridPos]bool) bool {
	for pos := range robots {
		// just check the first 5 lines
		line := 0
		for line < 5 {
			if line == 0 {
				// ensure no other robot are on the side of the tip
				_, left := robots[pos.Add(utils.MOVE_R)]
				_, right := robots[pos.Add(utils.MOVE_L)]
				if left || right {
					// not a good one
					break
				}
			} else if !robotsAligned(robots, pos, line) {
				break
			}
			line++
		}
		if line == 5 {
			// this is fine
			utils.Info("Found tree at %s", pos)
			return true
		}
	}
	return false
}

// checks if there are a certain number of robots aligned on the side
func robotsAligned(robots map[utils.GridPos]bool, pos utils.GridPos, line int) bool {
	trunk := pos.Add(utils.RowColToGridPos(0, line))
	if _, ok := robots[trunk]; !ok {
		return false
	}
	for i := 1; i <= line; i++ {
		_, left := robots[trunk.Add(utils.RowColToGridPos(-i, 0))]
		_, right := robots[trunk.Add(utils.RowColToGridPos(i, 0))]
		if !left && !right {
			// not a good one
			return false
		}
	}
	return true
}

func printTree(positions map[utils.GridPos]bool) {
	for row := range DEFAULT_HEIGHT {
		line := make([]byte, DEFAULT_WIDTH)
		for col := range DEFAULT_WIDTH {
			_, ok := positions[utils.GridPos{col, row}]
			if ok {
				line[col] = '#'
			} else {
				line[col] = '.'
			}
		}
		fmt.Println(string(line))
	}
}
