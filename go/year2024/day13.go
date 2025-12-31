package main

import (
	"adventofcode/utils"
	"aoc/aoc"
	"math/big"
	"regexp"
	"strconv"
)

type claw struct {
	buttonA utils.GridPos
	buttonB utils.GridPos
	prize   utils.GridPos
}

type day13 struct {
	machines []claw
}

func init() {
	utils.RegisterSolver("2024", "day13", func() utils.Solver {
		return &day13{
			machines: []claw{},
		}
	})
}

var RE_BUTTON = regexp.MustCompile("Button (.): X\\+(\\d+), Y\\+(\\d+)")
var RE_PRIZE = regexp.MustCompile("Prize: X=(\\d+), Y=(\\d+)")

const DELTA_PART2 = 10000000000000

func (solver *day13) Parse(line string) {
	if len(line) == 0 {
		return
	}

	matches := RE_BUTTON.FindStringSubmatch(line)
	if matches != nil && len(matches) == 4 {
		buttonName := matches[1]
		x, _ := strconv.Atoi(matches[2])
		y, _ := strconv.Atoi(matches[3])
		delta := utils.RowColToGridPos(x, y)
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
		delta := utils.RowColToGridPos(x, y)
		solver.machines[len(solver.machines)-1].prize = delta
		return
	}

	panic("Unmatched line?? " + line)
}

func (solver *day13) Solve() (*string, *string) {
	coins1 := 0
	coins2 := 0
	for _, machine := range solver.machines {
		coins := findCoins(machine, 0)
		if coins != -1 {
			coins1 += coins
		}
		coins = findCoins(machine, DELTA_PART2)
		if coins != -1 {
			coins2 += coins
		}
	}
	part1 := strconv.Itoa(coins1)
	part2 := strconv.Itoa(coins2)
	return &part1, &part2
}

func findCoins(machine claw, delta int) int {
	// there's probably only one solution for this system
	// { Ax * m + Bx * m = Px
	// { Ay * m + By * m = Py

	Ax := int64(machine.buttonA.Col)
	Ay := int64(machine.buttonA.Row)
	Bx := int64(machine.buttonB.Col)
	By := int64(machine.buttonB.Row)
	Px := int64(machine.prize.Col + delta)
	Py := int64(machine.prize.Row + delta)

	mRes := big.NewRat(Px, Ax)
	mRes.Sub(mRes, big.NewRat(Py, Ay))
	mDen := big.NewRat(Bx, Ax)
	mDen.Sub(mDen, big.NewRat(By, Ay))
	mRes.Quo(mRes, mDen)
	if mRes.Sign() < 0 || !mRes.IsInt() {
		return -1
	}
	m := mRes.Num().Int64()

	// try to find N
	n := big.NewRat(Px-(Bx*m), Ax)
	if n.Sign() < 0 || !n.IsInt() {
		return -1
	}

	coins := n.Num().Int64()*3 + m

	return int(coins)
}
