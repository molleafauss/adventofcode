package main

import (
	"aoc/aoc"
)

func main() {
	aoc.SetLogLevel(aoc.DEBUG)
	aoc.Run("2024", solverFactory)
}

func solverFactory(day string) aoc.Solver {
	switch day {
	case "day01":
		return Day01()
	}
	return nil
}
