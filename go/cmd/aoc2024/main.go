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
	case "day02":
		return Day02()
	case "day03":
		return Day03()
	case "day04":
		return Day04()
	case "day05":
		return Day05()
	}
	return nil
}
