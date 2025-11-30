package main

import (
	"aoc/aoc"
)

func main() {
	aoc.SetLogLevel(aoc.INFO)
	aoc.Run("2022", solverFactory)
}

func solverFactory(day string) aoc.Solver {
	switch day {
	case "day16":
		return Day16()
	}
	return nil
}
