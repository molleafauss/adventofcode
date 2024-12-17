package main

import (
	"aoc/aoc"
	"slices"
	"strconv"
	"strings"
)

type day11 struct {
	stones []int
}

func Day11() aoc.Solver {
	return &day11{}
}

const ITERATIONS = 25

func (solver *day11) Parse(line string) {
	initial := strings.Split(line, " ")
	solver.stones = make([]int, len(initial))
	for i, val := range initial {
		solver.stones[i], _ = strconv.Atoi(val)
	}
}

func (solver *day11) Solve() (*string, *string) {
	for i := 0; i < ITERATIONS; i++ {
		for j := 0; j < len(solver.stones); j++ {
			stone := solver.stones[j]
			text := strconv.Itoa(stone)
			digits := len(text)
			if stone == 0 {
				solver.stones[j] = 1
			} else if (digits % 2) == 0 {
				a, b := split(text, digits)
				solver.stones[j] = a
				solver.stones = slices.Insert(solver.stones, j+1, b)
				j++
			} else {
				solver.stones[j] = stone * 2024
			}
		}
	}
	part1 := strconv.Itoa(len(solver.stones))
	return &part1, nil
}

func split(text string, digits int) (int, int) {
	digits /= 2
	a, _ := strconv.Atoi(text[:digits])
	b, _ := strconv.Atoi(text[digits:])
	return a, b
}
