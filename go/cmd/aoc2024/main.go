package main

import (
	"aoc/aoc"
)

func main() {
	aoc.SetLogLevel(aoc.INFO)
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
	case "day06":
		return Day06()
	case "day07":
		return Day07()
	case "day08":
		return Day08()
	case "day09":
		return Day09()
	case "day10":
		return Day10()
	case "day11":
		return Day11()
	case "day12":
		return Day12()
	case "day13":
		return Day13()
	case "day14":
		return Day14()
	case "day15":
		return Day15()
	case "day16":
		return Day16()
	case "day17":
		return Day17()
	}
	return nil
}
