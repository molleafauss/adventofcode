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
	data := day[:5]
	utils.Info("== Solving %s / %s ==", opts.year, day)

	solver := utils.CreateSolver(opts.year, day)
	if solver == nil {
		return fmt.Errorf("no solver available for day: %s / %s", opts.year, day)
	}

	testFile := fmt.Sprintf("%s/%s/%s/test.txt", opts.inputDir, opts.year, data)
	if err := solve(testFile, solver); err != nil {
		return err
	}

	inputFile := fmt.Sprintf("%s/%s/%s/input.txt", opts.inputDir, opts.year, data)
	return solve(inputFile, utils.CreateSolver(opts.year, day))
}

func solve(file string, solver utils.Solver) error {
	var expectedPart1 string
	var expectedPart2 string
	f, err := os.Open(file)
	if err != nil {
		return fmt.Errorf("cannot open file %s: %v", file, err)
	}
	t0 := time.Now()
	scanner := bufio.NewScanner(f)
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
	utils.Info("File %s: %s", file, delta)
	if part1 != nil {
		if *part1 == expectedPart1 {
			utils.Info("PART 1 - found expected result: %s = %s", expectedPart1, *part1)
		} else {
			utils.Error("ERROR - part 1 result is incorrect: expected %s, actual %s",
				expectedPart1, *part1)
		}
	}

	if part2 != nil {
		if *part2 == expectedPart2 {
			utils.Info("PART 2 - found expected result: %s = %s", expectedPart2, *part2)
		} else {
			utils.Error("ERROR - part 2 result is incorrect: expected %s, actual %s",
				expectedPart2, *part2)
		}
	}
	return nil
}
