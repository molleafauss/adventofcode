package main

import (
	"aoc/aoc"
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
	start aoc.GridPos
	vel   aoc.GridPos
}

func Day14() aoc.Solver {
	return &day14{
		robots: []robot{},
		width:  DEFAULT_WIDTH,
		height: DEFAULT_HEIGHT,
	}
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
	solver.robots = append(solver.robots, robot{start: aoc.RowColToGridPos(px, py), vel: aoc.RowColToGridPos(vx, vy)})
}

func (solver *day14) Solve() (*string, *string) {
	quadrants := []int{0, 0, 0, 0}
	for _, robot := range solver.robots {
		travelX := robot.vel.Col * TIME
		travelY := robot.vel.Row * TIME
		endPos := robot.start.Add(aoc.RowColToGridPos(travelX, travelY))
		// wrap around
		endPos.Row = wrap(endPos.Row, solver.height)
		endPos.Col = wrap(endPos.Col, solver.width)
		q := findQuadrant(endPos, solver.width, solver.height)
		aoc.Info("Robot %v ends at %s / quadrant %d", robot, endPos, q)
		if q != -1 {
			quadrants[q] += 1
		}
	}

	aoc.Info("Final quadrants: %v", quadrants)

	part1 := strconv.Itoa(quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3])
	return &part1, nil
}

func findQuadrant(pos aoc.GridPos, width int, height int) int {
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
