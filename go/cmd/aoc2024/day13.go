package main

import (
	"aoc/aoc"
	"regexp"
	"strconv"
)

type claw struct {
	buttonA aoc.GridPos
	buttonB aoc.GridPos
	prize   aoc.GridPos
}

type day13 struct {
	machines []claw
}

func Day13() aoc.Solver {
	return &day13{
		machines: []claw{},
	}
}

var RE_BUTTON = regexp.MustCompile("Button (.): X\\+(\\d+), Y\\+(\\d+)")
var RE_PRIZE = regexp.MustCompile("Prize: X=(\\d+), Y=(\\d+)")

func (solver *day13) Parse(line string) {
	if len(line) == 0 {
		return
	}

	matches := RE_BUTTON.FindStringSubmatch(line)
	if matches != nil && len(matches) == 4 {
		buttonName := matches[1]
		x, _ := strconv.Atoi(matches[2])
		y, _ := strconv.Atoi(matches[3])
		delta := aoc.RowColToGridPos(x, y)
		if buttonName == "A" {
			solver.machines = append(solver.machines, claw{
				buttonA: delta,
			})
		} else if buttonName == "B" {
			solver.machines[len(solver.machines)-1].buttonB = delta
		}
		return
	}

	matches = RE_PRIZE.FindStringSubmatch(line)
	if matches != nil && len(matches) == 3 {
		x, _ := strconv.Atoi(matches[1])
		y, _ := strconv.Atoi(matches[2])
		delta := aoc.RowColToGridPos(x, y)
		solver.machines[len(solver.machines)-1].prize = delta
		return
	}

	panic("Unmatched line?? " + line)
}

func (solver *day13) Solve() (*string, *string) {
	coins1 := 0
	for _, machine := range solver.machines {
		coins := findCoins(machine)
		if coins == -1 {
			continue
		}
		coins1 += coins
	}
	part1 := strconv.Itoa(coins1)
	return &part1, nil
}

func findCoins(machine claw) int {
	// impossible to reach
	if machine.buttonA.Row*100+machine.buttonB.Row*100 < machine.prize.Row ||
		machine.buttonA.Col*100+machine.buttonB.Col*100 < machine.prize.Col {
		return -1
	}

	// see if one button only can surpass
	maxAx := machine.prize.Col / machine.buttonA.Col
	maxAy := machine.prize.Row / machine.buttonA.Row
	pressA := min(maxAx, maxAy, 100)
	maxBx := machine.prize.Col / machine.buttonB.Col
	maxBy := machine.prize.Row / machine.buttonB.Row
	pressB := min(maxBx, maxBy, 100)
	aoc.Info("Would either press A %d or B %d", pressA, pressB)

	count := min(pressA, pressB)
	claw1 := machine.buttonA
	cost1 := 3
	claw2 := machine.buttonB
	cost2 := 1
	if pressB < pressA {
		// invert
		claw2, claw1 = claw1, claw2
		cost2, cost1 = cost1, cost2
	}

	// max is 100*3+100
	minCost := 500
	for ; count >= 0; count-- {
		// every time I find an exact match I record presses and cost
		press2 := willReach(machine.prize.Row, claw1.Row*count, claw2.Row)
		if press2 == -1 {
			continue
		}
		if press2 > 100 {
			// stop here - I am too far
			break
		}
		if claw1.Col*count+claw2.Col*press2 != machine.prize.Col {
			continue
		}
		cost := cost1*count + cost2*press2
		if cost < minCost {
			minCost = cost
			aoc.Info("Found better solution with %dx%d - %dx%d = %d", count, cost1, press2, cost2, cost)
		}
	}

	// no solution found
	if minCost == 500 {
		return -1
	}

	return minCost
}

func willReach(end int, start int, delta int) int {
	if start > end {
		return -1
	}
	remainder := end - start
	count := remainder / delta
	if delta*count == remainder {
		return count
	}
	return -1
}
