package day01

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

type ElfFood struct {
	num      int
	calories int
}

func Calories() {
	file, err := os.Open("../files/01-calories.txt")
	if err != nil {
		fmt.Printf("Error opening input: %v\n", err)
	}
	scanner := bufio.NewScanner(file)
	var elf = 0
	var elf_calories = 0
	var elves = make([]ElfFood, 10)
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			fmt.Printf("Elf n. %d holding %d calories\n", elf, elf_calories)
			elves = append(elves, ElfFood{elf, elf_calories})
			elf++
			elf_calories = 0
		} else {
			cal, err := strconv.Atoi(line)
			if err != nil {
				fmt.Printf("Invalid line: %s - %v\n", line, err)
			}
			elf_calories += cal
		}
	}
	sort.Slice(elves, func(i, j int) bool {
		return elves[i].calories > elves[j].calories
	})
	fmt.Printf("Highest calories: %d\n", elves[0].calories)
	var top3 = 0
	for i := 0; i < 3; i++ {
		top3 += elves[i].calories
	}
	fmt.Printf("Top 3 calories: %d\n", top3)
}
