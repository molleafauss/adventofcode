package main

import (
	"adventofcode/utils"
	"aoc/aoc"
	"fmt"
	"slices"
	"strconv"
)

type files struct {
	id    int
	start int
	end   int
}

type day09 struct {
	disk     []files
	diskSize int
}

func init() {
	utils.RegisterSolver("2024", "day09", func() utils.Solver {
		return &day09{}
	})
}

func (solver *day09) Parse(line string) {
	disk := make([]files, 0)
	totalLen := 0
	fileId := 0
	// calculate total len first
	for i := range line {
		val, err := strconv.Atoi(line[i : i+1])
		if err != nil {
			panic(fmt.Sprintf("Line not numbers? %d / %s", i, line[i:i]))
		}
		id := fileId
		if (i % 2) == 0 {
			fileId++
		} else {
			id = -1
		}
		disk = append(disk, files{id, totalLen, totalLen + val})
		totalLen += val
	}
	utils.Info("Found total len: %d", totalLen)
	solver.disk = disk
	solver.diskSize = totalLen
}

func (solver *day09) Solve() (*string, *string) {
	// part1
	disk := makeDisk(solver.disk, solver.diskSize)
	// defrag
	defragBasic(disk)
	// calculate checksum
	checksum1 := checksum(disk)

	// part2
	defragFat(solver.disk)
	// calculate checksum
	checksum2 := checksumFat(solver.disk)

	part1 := strconv.Itoa(checksum1)
	part2 := strconv.Itoa(checksum2)
	return &part1, &part2
}

func makeDisk(files []files, size int) []int {
	disk := make([]int, size)
	// fill disk
	for _, f := range files {
		for p := f.start; p < f.end; p++ {
			disk[p] = f.id
		}
	}
	utils.Debug("Disk Map before defrag: %v", disk)
	return disk
}

func defragBasic(disk []int) []int {
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
	utils.Debug("Disk Map after defrag: %v", disk)
	return disk
}

func checksum(disk []int) int {
	checksum := 0
	for i, val := range disk {
		if val == -1 {
			break
		}
		checksum += val * i
	}
	utils.Info("Checksum: %d", checksum)
	return checksum
}

func defragFat(fat []files) {
	// move only FAT entries
	start := 1
	fileId := fat[len(fat)-1].id
	for fileId >= 0 {
		end := -1
		for i := len(fat) - 1; i >= 0; i-- {
			if fat[i].id == fileId {
				end = i
				break
			}
		}
		if end < start {
			// no more stuff to do
			break
		}
		utils.Info("Trying to move file %d [start %d end %d]", fileId, start, end)
		fileEntry := &fat[end]
		fileLength := fileEntry.end - fileEntry.start
		space := start
		for {
			// not found any space - pass on to next
			if space > end {
				fileId--
				utils.Info("Not moving file %d - no space found (start %d, space %d, end %d)", fileEntry.id, start, space, end)
				break
			}
			blankEntry := &fat[space]
			if blankEntry.id != -1 {
				space++
				continue
			}
			blankLength := blankEntry.end - blankEntry.start
			if blankLength < fileLength {
				// doesn't fit, check next space
				space++
				continue
			}
			if fileLength == blankLength {
				utils.Info("Moving file %d [full] -> [%d,%d]  (start %d, space %d, end %d)", fileEntry.id, blankEntry.start, blankEntry.end, start, space, end)
				// if file fits exactly, I should just swap the ids between the two
				fileEntry.id, blankEntry.id = blankEntry.id, fileEntry.id
				// move start only if we found space there
				fileId--
			} else {
				// insert a new entry, reduce the size of the blank space
				movedFile := files{fileEntry.id, blankEntry.start, blankEntry.start + fileLength}
				utils.Info("Moving file %d [part] -> [%d,%d]  (start %d, space %d, end %d)", movedFile.id, movedFile.start, movedFile.end, start, space, end)
				blankEntry.start = blankEntry.start + fileLength
				fileEntry.id = -1
				fat = slices.Insert(fat, space, movedFile)
				fileId--
			}
			// XXX coalesce blank space here
			break
		}
		for fat[start].id != -1 {
			start++
		}
	}
}

func checksumFat(fat []files) int {
	checksum := 0
	for _, file := range fat {
		if file.id == -1 {
			continue
		}
		utils.Info("Adding %v to checksum", file)
		for p := file.start; p < file.end; p++ {
			checksum += file.id * p
		}
	}
	return checksum
}
