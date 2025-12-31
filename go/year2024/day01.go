package main

import (
	"adventofcode/utils"
	"slices"
	"strconv"
	"strings"
)

type day01 struct {
	left  []int
	right []int
}

func init() {
	utils.RegisterSolver("2024", "day01", func() utils.Solver {
		return &day01{
			left:  make([]int, 0),
			right: make([]int, 0),
		}
	})
}

func (solver *day01) Parse(line string) {
	parts := strings.Split(line, " ")
	l, err := strconv.Atoi(parts[0])
	if err != nil {
		panic(err)
	}
	solver.left = append(solver.left, l)
	r, err := strconv.Atoi(parts[len(parts)-1])
	if err != nil {
		panic(err)
	}
	solver.right = append(solver.right, r)
}

func (solver *day01) Solve() (*string, *string) {
	utils.Info("location sizes %d/%d", len(solver.left), len(solver.right))
	if len(solver.left) != len(solver.right) {
		utils.Error("location sizes are uneven??")
		return nil, nil
	}
	slices.Sort(solver.left)
	slices.Sort(solver.right)
	// part 1
	total := 0
	for i := range len(solver.left) {
		delta := solver.left[i] - solver.right[i]
		if delta < 0 {
			delta = -delta
		}
		total += delta
	}
	part1 := strconv.Itoa(total)

	// part 2 - pop from left until empty and count on right how many times it happens
	// lists are sorted so no need keep going through the list
	similarity := 0
	l := 0
	r := 0
	count := 0
	for l < len(solver.left) {
		if l > 0 && solver.left[l] == solver.left[l-1] {
			similarity += solver.left[l] * count
			l++
			continue
		}
		// is the current left item > than the current right? advance right
		for solver.right[r] < solver.left[l] {
			r++
		}
		// reset count
		count = 0
		// count how many are equal
		for solver.right[r] == solver.left[l] {
			r++
			count++
		}
		// add to similarity
		similarity += solver.left[l] * count
		l++
	}
	part2 := strconv.Itoa(similarity)
	return &part1, &part2
}
