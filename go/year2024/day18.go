package year2024

import (
	"adventofcode/utils"
	"container/list"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

const GRID_SIZE = 70
const CORRUPTION_TIME = 1024

type day18 struct {
	falling []utils.GridPos
	width   int
	height  int
	time    int
}

func init() {
	utils.RegisterSolver("2024", "day18", func() utils.Solver {
		return &day18{
			falling: make([]utils.GridPos, 0),
			width:   GRID_SIZE + 1,
			height:  GRID_SIZE + 1,
			time:    CORRUPTION_TIME,
		}
	})
}

func (solver *day18) Parse(line string) {
	if strings.HasPrefix(line, "grid size: ") {
		w, err := strconv.Atoi(strings.TrimPrefix(line, "grid size: "))
		if err != nil {
			panic("invalid grid width: " + line)
		}
		solver.width = w
		solver.height = w
		return
	}
	if strings.HasPrefix(line, "time: ") {
		w, err := strconv.Atoi(strings.TrimPrefix(line, "time: "))
		if err != nil {
			panic("invalid time: " + line)
		}
		solver.time = w
		return
	}
	parts := strings.Split(line, ",")
	x, _ := strconv.Atoi(parts[0])
	y, _ := strconv.Atoi(parts[1])
	solver.falling = append(solver.falling, utils.GridPos{x, y})
}

type memoryWalk struct {
	pos  utils.GridPos
	cost int
}

func (solver *day18) Solve() (*string, *string) {
	utils.Info("Solving on %dx%d grid, limit corruption to %d/%d", solver.width, solver.height, solver.time, len(solver.falling))
	minSteps := solver.findExit(solver.time)
	maxTime := solver.time
	for ; maxTime < len(solver.falling); maxTime++ {
		exit := solver.findExit(maxTime)
		utils.Info("Found exit for %d at %d", maxTime, exit)
		if exit == -1 {
			break
		}
	}
	part1 := strconv.Itoa(minSteps)
	part2 := fmt.Sprintf("%d,%d", solver.falling[maxTime-1].Col, solver.falling[maxTime-1].Row)
	return &part1, &part2
}

func (solver *day18) findExit(time int) int {
	start := utils.GridPos{0, 0}
	end := utils.GridPos{solver.width - 1, solver.height - 1}
	visited := make(map[utils.GridPos]bool)
	queue := list.New()
	queue.PushBack(memoryWalk{start, 0})
	iterations := 0
	for queue.Len() > 0 {
		iterations++
		mw := queue.Remove(queue.Front()).(memoryWalk)
		if mw.pos == end {
			return mw.cost
		}
		// have I passed through here already?
		if _, beenHere := visited[mw.pos]; beenHere {
			continue
		}
		visited[mw.pos] = true
		for _, dir := range utils.ALL_ORTHOGONAL {
			next := mw.pos.Add(dir)
			if !next.InBounds(solver.width, solver.height) {
				continue
			}
			// would I find a corrupted byte - don't step into it
			corrupted := slices.Index(solver.falling, next)
			if corrupted != -1 && corrupted < time {
				continue
			}
			// follow a path first
			queue.PushBack(memoryWalk{next, mw.cost + 1})
		}
	}
	return -1
}
