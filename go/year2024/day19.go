package main

import (
	"adventofcode/utils"
	"aoc/aoc"
	"strconv"
	"strings"
)

type day19 struct {
	patterns []string
	part1    int
	part2    int
}

func init() {
	utils.RegisterSolver("2022", "day19", func() utils.Solver {
		return &day19{
			patterns: []string{},
		}
	})
}

func (solver *day19) Parse(line string) {
	if len(solver.patterns) == 0 {
		solver.patterns = strings.Split(line, ", ")
		totalSize := 0
		for _, pattern := range solver.patterns {
			totalSize += len(pattern)
		}
		aoc.Info("Found %d patterns, avg len %d - %s", len(solver.patterns), totalSize/len(solver.patterns), solver.patterns)
	} else if len(line) > 0 {
		cache := make(map[string]int)
		count := designFeasible(cache, solver.patterns, line)
		if count > 0 {
			solver.part1++
		}
		solver.part2 += count
		aoc.Info("%s feasible in %d ways", line, count)
	}
}

func designFeasible(cache map[string]int, patterns []string, design string) int {
	// we matched all
	if 0 == len(design) {
		return 1
	}
	if val, exists := cache[design]; exists {
		return val
	}
	// try to keep matching a towel
	count := 0
	for i := range patterns {
		if strings.HasPrefix(design, patterns[i]) {
			count += designFeasible(cache, patterns, design[len(patterns[i]):])
		}
	}
	cache[design] = count
	return count
}

func (solver *day19) Solve() (*string, *string) {
	part1 := strconv.Itoa(solver.part1)
	part2 := strconv.Itoa(solver.part2)
	return &part1, &part2
}
