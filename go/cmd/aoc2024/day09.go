package main

import (
	"aoc/aoc"
	"fmt"
	"strconv"
)

type day09 struct {
	checksum int
}

func Day09() aoc.Solver {
	return &day09{}
}

func (solver *day09) Parse(line string) {
	totalLen := 0
	// calculate total len first
	for i := range line {
		val, err := strconv.Atoi(line[i : i+1])
		if err != nil {
			panic(fmt.Sprintf("Line not numbers? %d / %s", i, line[i:i]))
		}
		totalLen += val
	}
	aoc.Info("Found total len: %d", totalLen)

	disk := make([]int, totalLen)
	// fill disk
	pos := 0
	fileId := 0
	for i := range line {
		val, err := strconv.Atoi(line[i : i+1])
		if err != nil {
			panic(fmt.Sprintf("Line not numbers? %d / %s", i, line[i:i]))
		}
		id := fileId
		if (i % 2) == 1 {
			id = -1
		} else {
			fileId++
		}
		for {
			if val == 0 {
				break
			}
			disk[pos] = id
			val--
			pos++
		}
	}
	aoc.Debug("Disk Map before defrag: %v", disk)
	// defrag
	begin := 0
	end := len(disk) - 1
	for begin < end {
		// begin not on free space, move forward
		if disk[begin] != -1 {
			begin++
			continue
		}
		// end not on occupied space, move backward
		if disk[end] == -1 {
			end--
			continue
		}
		// swap
		disk[begin], disk[end] = disk[end], disk[begin]
		begin++
		end--
	}
	aoc.Debug("Disk Map after defrag: %v", disk)
	// calculate checksum
	checksum := 0
	for i, val := range disk {
		if val == -1 {
			break
		}
		checksum += val * i
	}
	aoc.Info("Checksum: %d", checksum)
	solver.checksum = checksum
}

func (solver *day09) Solve() (*string, *string) {
	part1 := strconv.Itoa(solver.checksum)
	return &part1, nil
}
