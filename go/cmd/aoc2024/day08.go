package main

import (
	"aoc/aoc"
	"fmt"
	"strconv"
)

type day08 struct {
	antennas map[uint8][]aoc.GridPos
	width    int
	height   int
}

func Day08() aoc.Solver {
	return &day08{
		antennas: map[uint8][]aoc.GridPos{},
	}
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
	antinodes := map[aoc.GridPos]bool{}
	for ch, antennas := range solver.antennas {
		aoc.Info("Found %d antennas at frequency '%s'", len(antennas), []uint8{ch})
		// check every antenna with the others
		for i := range antennas {
			for j := i + 1; j < len(antennas); j++ {
				pos_a := antennas[i]
				pos_b := antennas[j]
				delta_c, delta_r := pos_a.Distance(&pos_b)
				//
				antinode1 := pos_a.Add(aoc.RowColToGridPos(delta_c, delta_r))
				if antinode1.InBounds(solver.width, solver.height) {
					aoc.Info("Antinode for %s<>%s -> %s", pos_a, pos_b, antinode1)
					antinodes[antinode1] = true
				}
				antinode2 := pos_b.Add(aoc.RowColToGridPos(-delta_c, -delta_r))
				if antinode2.InBounds(solver.width, solver.height) {
					aoc.Info("Antinode for %s<>%s -> %s", pos_a, pos_b, antinode2)
					antinodes[antinode2] = true
				}
			}
		}
	}
	part1 := strconv.Itoa(len(antinodes))
	return &part1, nil
}
