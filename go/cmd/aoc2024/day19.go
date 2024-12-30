package main

import (
	"aoc/aoc"
	"slices"
	"strconv"
	"strings"
)

type day19 struct {
	patterns        []string
	possibleDesigns int
}

func Day19() aoc.Solver {
	return &day19{
		patterns: []string{},
	}
}

func (solver *day19) Parse(line string) {
	if len(solver.patterns) == 0 {
		solver.patterns = strings.Split(line, ", ")
		totalSize := 0
		for _, pattern := range solver.patterns {
			totalSize += len(pattern)
		}
		aoc.Info("Found %d patterns, avg len %d", len(solver.patterns), totalSize/len(solver.patterns))
	} else if len(line) > 0 {
		matching := matchPatterns(line, solver.patterns)
		aoc.Info("Checking patterns in (len %d) %s - len matching %d", len(line), line, len(matching))
		cache := make(map[string]bool)
		if designFeasible(cache, matching, line) {
			solver.possibleDesigns++
			aoc.Info("%s is feasible (%d) - cache size: %d", line, solver.possibleDesigns, len(cache))
		}
	}
}

func matchPatterns(line string, patterns []string) []string {
	// find all patterns that match at least once
	matching := []string{}
	for _, pattern := range patterns {
		if strings.Contains(line, pattern) {
			matching = append(matching, pattern)
		}
	}
	// sort on longer first
	slices.SortFunc(matching, func(a string, b string) int {
		return len(b) - len(a)
	})
	return matching
}

func designFeasible(cache map[string]bool, patterns []string, design string) bool {
	// we matched all
	if 0 == len(design) {
		return true
	}
	if _, exists := cache[design]; exists {
		aoc.Info("Found feasible design for %s", design)
		return true
	}
	// try to keep matching a towel
	for i := range patterns {
		if strings.HasPrefix(design, patterns[i]) && designFeasible(cache, patterns, design[len(patterns[i]):]) {
			aoc.Info("Found feasible design for %s", design)
			cache[design] = true
			return true
		}
	}
	// nothing matches here
	return false
}

func (solver *day19) Solve() (*string, *string) {
	part1 := strconv.Itoa(solver.possibleDesigns)
	return &part1, nil
}
