package main

import (
	"aoc/aoc"
	"slices"
	"strconv"
	"strings"
)

type day01 struct {
	left  []int
	right []int
}

func Day01() *day01 {
	return &day01{
		left:  make([]int, 0),
		right: make([]int, 0),
	}
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
	if len(solver.left) != len(solver.right) {
		aoc.Error("location sizes are uneven?? %d/%d", len(solver.left), len(solver.right))
		return nil, nil
	}
	slices.Sort(solver.left)
	slices.Sort(solver.right)
	total := 0
	for i := range len(solver.left) {
		delta := solver.left[i] - solver.right[i]
		if delta < 0 {
			delta = -delta
		}
		total += delta
	}
	part1 := strconv.Itoa(total)
	return &part1, nil
}
