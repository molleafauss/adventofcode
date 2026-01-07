package main

import (
	"adventofcode/utils"
	// register all solvers
	_ "adventofcode/year2022"
	_ "adventofcode/year2024"
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Short: "Solves Advent of Code puzzles",
	Args:  cobra.ExactArgs(1),
	RunE:  adventOfCode,
}

func getLastAocYear() string {
	now := time.Now()
	if now.Month() == time.December {
		return fmt.Sprintf("%d", now.Year())
	}
	return fmt.Sprintf("%d", now.Year()-1)
}

type args struct {
	debug    bool
	year     string
	inputDir string
}

var opts = args{}

func main() {
	rootCmd.Flags().StringVar(&opts.year, "year", getLastAocYear(),
		"Year of the Advent of Code event - default last available year.")
	rootCmd.Flags().StringVar(&opts.inputDir, "inputs", ".",
		"Directory to read input files from, default current directory.")
	rootCmd.Flags().BoolVar(&opts.debug, "debug", false, "Enable debug log.")
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

func adventOfCode(_ *cobra.Command, args []string) error {
	if opts.debug {
		utils.SetLogLevel(utils.DEBUG)
		utils.Debug("enabling debug mode")
	} else {
		utils.SetLogLevel(utils.INFO)
	}

	if err := verifyInputDir(); err != nil {
		return err
	}

	if err := solvePuzzle(args[0]); err != nil {
		return err
	}
	return nil
}

func verifyInputDir() error {
	info, err := os.Stat(opts.inputDir)
	if os.IsNotExist(err) {
		return fmt.Errorf("input directory %s does not exist: %v", opts.inputDir, err)
	}
	if !info.IsDir() {
		return fmt.Errorf("input path %s is not a directory", opts.inputDir)
	}
	absPath, err := filepath.Abs(opts.inputDir)
	if err != nil {
		return fmt.Errorf("failed to get absolute path for %s: %v", opts.inputDir, err)
	}
	opts.inputDir = absPath
	return nil
}

func solvePuzzle(dayArg string) error {
	if dayArg == "all" {
		return solveAll()
	} else if strings.HasPrefix(dayArg, "day") {
		return solveDay(dayArg)
	} else {
		return fmt.Errorf("invalid day argument: %s", dayArg)
	}
}

func solveAll() error {
	for day := range 26 {
		if err := solveDay(fmt.Sprintf("day%02d", day+1)); err != nil {
			return err
		}
	}
	return nil
}

func solveDay(day string) error {
	puzzle := day[:5]
	utils.Info("== Solving %s/%s ==", opts.year, puzzle)

	solver := utils.CreateSolver(opts.year, day)
	if solver == nil {
		utils.Warn("%s/%s | no solution implemented", opts.year, day)
		return nil
	}

	solve(puzzle, "test.txt", solver)
	solve(puzzle, "input.txt", solver)
	return nil
}

func solve(puzzle string, file string, solver utils.Solver) {
	filename := fmt.Sprintf("%s/%s/%s/%s", opts.inputDir, opts.year, puzzle, file)
	f, err := os.Open(filename)
	if err != nil {
		utils.Warn("%s/%s | missing file: %s", opts.year, puzzle, file)
		return
	}
	t0 := time.Now()
	scanner := bufio.NewScanner(f)
	var expectedPart1 string
	var expectedPart2 string
	for scanner.Scan() {
		line := scanner.Text()
		if strings.HasPrefix(line, "result part 1: ") {
			expectedPart1 = line[15:]
		} else if strings.HasPrefix(line, "result part 2: ") {
			expectedPart2 = line[15:]
		} else {
			solver.Parse(line)
		}
	}

	part1, part2 := solver.Solve()
	delta := time.Since(t0)
	utils.Info("%s/%s | %s solved in %s", opts.year, puzzle, file, delta)
	verify(puzzle, file, 1, expectedPart1, part1)
	verify(puzzle, file, 2, expectedPart2, part2)
}

func verify(puzzle string, file string, part int, expected string, result *string) {
	if result == nil {
		return
	}
	if *result == expected {
		utils.Info("%s/%s | %s | RESULT PART %d - correct: %s", opts.year, puzzle, file, part,
			expected)
	} else {
		utils.Error("%s/%s | %s | RESULT PART %d - expected %s, actual %s", opts.year,
			puzzle, file, part, expected, *result)
	}
}
