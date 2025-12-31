package main

import (
	"adventofcode/utils"
	"aoc/aoc"
	"strconv"
	"strings"
)

type day11 struct {
	stones []int
}

func init() {
	utils.RegisterSolver("2022", "day11", func() utils.Solver {
		return &day11{}
	})
}

const ITERATIONS = 25

func (solver *day11) Parse(line string) {
	initial := strings.Split(line, " ")
	solver.stones = make([]int, len(initial))
	for i, val := range initial {
		solver.stones[i], _ = strconv.Atoi(val)
	}
}

type key struct {
	number int
	level  int
}

func (solver *day11) Solve() (*string, *string) {
	var cache = make(map[key]int)
	result := 0
	for _, num := range solver.stones {
		result += blink(num, 25, &cache)
	}
	aoc.Info("Found part1 result: %d, cache size %d", result, len(cache))
	part1 := strconv.Itoa(result)

	result = 0
	for _, num := range solver.stones {
		result += blink(num, 75, &cache)
	}
	aoc.Info("Found part2 result: %d, cache size %d", result, len(cache))
	part2 := strconv.Itoa(result)

	return &part1, &part2
}

func blink(stone int, level int, cache *map[key]int) int {
	if val, ok := (*cache)[key{stone, level}]; ok {
		return val
	}

	if level == 0 {
		return 1
	}

	text := strconv.Itoa(stone)
	digits := len(text)
	var res int
	if stone == 0 {
		res = blink(1, level-1, cache)
	} else if (digits % 2) == 0 {
		a, b := split(text, digits)
		res = blink(a, level-1, cache) + blink(b, level-1, cache)
	} else {
		res = blink(stone*2024, level-1, cache)
	}
	(*cache)[key{stone, level}] = res
	return res
}

func split(text string, digits int) (int, int) {
	digits /= 2
	a, _ := strconv.Atoi(text[:digits])
	b, _ := strconv.Atoi(text[digits:])
	return a, b
}
