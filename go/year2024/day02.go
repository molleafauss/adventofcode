package year2024

import (
	"adventofcode/utils"
	"fmt"
	"strconv"
	"strings"
)

const UP = 1

const FLAT = 0

const DOWN = -1

const MAX_DELTA = 3

type day02 struct {
	part1 int
	part2 int
}

func init() {
	utils.RegisterSolver("2024", "day02", func() utils.Solver {
		return &day02{}
	})
}

func (solver *day02) Parse(line string) {
	vals := strings.Split(line, " ")
	nums := make([]int, len(vals))
	for i, s := range vals {
		nums[i], _ = strconv.Atoi(s)
	}

	if checkOrder(nums, -1) {
		utils.Info("(safe - no ignore) [%d]/[%d] %s", solver.part1, solver.part2, line)
		solver.part1++
		solver.part2++
		return
	}

	for i := range nums {
		if checkOrder(nums, i) {
			utils.Info("(safe - ignoring %d) [%d]/[%d] %s", nums[i], solver.part1, solver.part2, line)
			solver.part2++
			return
		}
	}
}

func checkOrder(nums []int, ignore int) bool {
	direction := FLAT
	prev := nums[0]
	cur := -1
	start := 1
	if ignore == 0 {
		prev = nums[1]
		start = 2
	}
	for i := start; i < len(nums); i++ {
		if i == ignore {
			continue
		}
		cur = nums[i]
		if direction == FLAT && cur > prev {
			direction = UP
		} else if direction == FLAT && cur < prev {
			direction = DOWN
		}

		err := checkLevels(direction, prev, cur)
		if err != nil {
			utils.Warn("%d - %s", nums, err.Error())
			return false
		}
		prev = cur
	}
	return true
}

func checkLevels(direction int, prev int, cur int) error {
	delta := cur - prev
	if delta == 0 {
		return fmt.Errorf("(unsafe) Found flat increase: %d -> %d", prev, cur)
	}
	if delta < -MAX_DELTA || delta > MAX_DELTA {
		return fmt.Errorf("(unsafe) found excessive delta %d: %d -> %d", delta, prev, cur)
	}

	if direction == UP && delta < 0 {
		return fmt.Errorf("(unsafe) found conflicting directions, expected up got %d: %d -> %d",
			delta, prev, cur)
	} else if direction == DOWN && delta > 0 {
		return fmt.Errorf("(unsafe) found conflicting directions, expected down got %d: %d -> %d",
			delta, prev, cur)
	}
	return nil
}

func (solver *day02) Solve() (*string, *string) {
	part1 := strconv.Itoa(solver.part1)
	part2 := strconv.Itoa(solver.part2)
	return &part1, &part2
}
