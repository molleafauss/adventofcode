package main

import (
	"aoc/aoc"
	"fmt"
	"math"
	"strconv"
	"strings"
)

type day17 struct {
	a       int
	b       int
	c       int
	program []int
}

func Day17() aoc.Solver {
	return &day17{
		program: make([]int, 0),
	}
}

func (solver *day17) Parse(line string) {
	if strings.HasPrefix(line, "Register A: ") {
		solver.a, _ = strconv.Atoi(line[12:])
	} else if strings.HasPrefix(line, "Register B: ") {
		solver.b, _ = strconv.Atoi(line[12:])
	} else if strings.HasPrefix(line, "Register C: ") {
		solver.c, _ = strconv.Atoi(line[12:])
	} else if strings.HasPrefix(line, "Program: ") {
		for _, op := range strings.Split(line[9:], ",") {
			val, _ := strconv.Atoi(op)
			// sanity check
			if val < 0 || val >= 8 {
				panic("invalid value for program: " + op)
			}
			solver.program = append(solver.program, val)
		}
	}
}

func (solver *day17) Solve() (*string, *string) {
	out := ""
	ip := 0
	for ip < len(solver.program) {
		switch solver.program[ip] {
		case 0:
			solver.adv(solver.program[ip+1])
			ip += 2
			continue
		case 1:
			solver.bxl(solver.program[ip+1])
			ip += 2
			continue
		case 2:
			solver.bst(solver.program[ip+1])
			ip += 2
			continue
		case 3:
			ip = solver.jnz(ip, solver.program[ip+1])
			continue
		case 4:
			solver.bxc()
			ip += 2
			continue
		case 5:
			out = solver.out(out, solver.program[ip+1])
			ip += 2
			continue
		case 6:
			solver.bdv(solver.program[ip+1])
			ip += 2
			continue
		case 7:
			solver.cdv(solver.program[ip+1])
			ip += 2
			continue
		}
		panic(fmt.Sprintf("Invalid operand?? ip: %d - %d", ip, solver.program[ip]))
	}
	return &out, nil
}

func (solver *day17) combo(op int) int {
	if op >= 0 && op <= 3 {
		return op
	} else if op == 4 {
		return solver.a
	} else if op == 5 {
		return solver.b
	} else if op == 6 {
		return solver.c
	}
	panic(fmt.Sprintf("Invalid combo op: %d", op))
}

func (solver *day17) adv(val int) {
	result := float64(solver.a) / math.Pow(2, float64(solver.combo(val)))
	solver.a = int(math.Trunc(result))
}

func (solver *day17) bxl(val int) {
	solver.b = solver.b ^ val
}

func (solver *day17) bst(val int) {
	solver.b = solver.combo(val) % 8
}

func (solver *day17) jnz(ip int, val int) int {
	if solver.a == 0 {
		return ip + 2
	} else {
		return val
	}
}

func (solver *day17) bxc() {
	solver.b = solver.b ^ solver.c
}

func (solver *day17) out(out string, val int) string {
	txt := strconv.Itoa(solver.combo(val) % 8)
	if len(out) == 0 {
		return txt
	} else {
		return out + "," + txt
	}
}

func (solver *day17) bdv(val int) {
	result := float64(solver.a) / math.Pow(2, float64(solver.combo(val)))
	solver.b = int(math.Trunc(result))
}

func (solver *day17) cdv(val int) {
	result := float64(solver.a) / math.Pow(2, float64(solver.combo(val)))
	solver.c = int(math.Trunc(result))
}
