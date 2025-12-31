package main

import (
	"adventofcode/utils"
	"aoc/aoc"
	"fmt"
	"strconv"
)

type day08 struct {
	antennas map[uint8][]aoc.GridPos
	width    int
	height   int
}

func init() {
	utils.RegisterSolver("2022", "day08", func() utils.Solver {
		return &day08{
			antennas: map[uint8][]aoc.GridPos{},
		}
	})
}

func (solver *day08) Parse(line string) {
	if solver.width == 0 {
		solver.width = len(line)
	} else if solver.width != len(line) {
		panic(fmt.Sprintf("Wrong line length at line %d?", solver.height))
	}
	for i := range line {
		if line[i] == '.' {
			continue
		}
		ch := line[i]
		ant, ok := solver.antennas[ch]
		if !ok {
			ant = []aoc.GridPos{}
		}
		ant = append(ant, aoc.RowColToGridPos(i, solver.height))
		solver.antennas[ch] = ant
	}
	solver.height++
}

func (solver *day08) Solve() (*string, *string) {
	antinodes := map[aoc.GridPos]int{}
	for ch, antennas := range solver.antennas {
		aoc.Info("Found %d antennas at frequency '%s'", len(antennas), []uint8{ch})
		// check every antenna with the others
		for i := range antennas {
			for j := i + 1; j < len(antennas); j++ {
				pos_a := antennas[i]
				pos_b := antennas[j]
				// antennas are antinodes with distance 0, but don't add them if one antinode exists already
				if _, ok := antinodes[pos_a]; !ok {
					antinodes[pos_a] = 0
				}
				if _, ok := antinodes[pos_b]; !ok {
					antinodes[pos_b] = 0
				}
				delta_c, delta_r := pos_a.Distance(&pos_b)
				// iterate and keep the direction until antinode is within bounds
				iter := 1
				antinode := pos_a
				for {
					antinode = antinode.Add(aoc.RowColToGridPos(delta_c, delta_r))
					if !antinode.InBounds(solver.width, solver.height) {
						break
					}
					aoc.Info("Antinode for %s<>%s -> %s  [distance %d]", pos_a, pos_b, antinode, iter)
					antinodes[antinode] = iter
					iter++
				}
				iter = 1
				antinode = pos_b
				for {
					antinode = antinode.Add(aoc.RowColToGridPos(-delta_c, -delta_r))
					if !antinode.InBounds(solver.width, solver.height) {
						break
					}
					aoc.Info("Antinode for %s<>%s -> %s [distance %d]", pos_a, pos_b, antinode, iter)
					antinodes[antinode] = iter
					iter++
				}
			}
		}
	}
	distanceOne := 0
	for _, val := range antinodes {
		if val == 1 {
			distanceOne++
		}
	}
	part1 := strconv.Itoa(distanceOne)
	part2 := strconv.Itoa(len(antinodes))
	return &part1, &part2
}
