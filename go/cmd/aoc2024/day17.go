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
	aoc.Info("Program: %d", solver.program)
	out := ""
	ip := 0
	for ip < len(solver.program) {
		aoc.Info("A=%d, B=%d, C=%d - IP %d", solver.a, solver.b, solver.c, ip)
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

func (solver *day17) combo(op int) (int, string) {
	if op >= 0 && op <= 3 {
		return op, strconv.Itoa(op)
	} else if op == 4 {
		return solver.a, "(A=" + strconv.Itoa(solver.a) + ")"
	} else if op == 5 {
		return solver.b, "(B=" + strconv.Itoa(solver.b) + ")"
	} else if op == 6 {
		return solver.c, "(C=" + strconv.Itoa(solver.c) + ")"
	}
	panic(fmt.Sprintf("Invalid combo op: %d", op))
}

func (solver *day17) adv(val int) {
	val, text := solver.combo(val)
	divisor := math.Pow(2, float64(val))
	result := int(math.Trunc(float64(solver.a) / divisor))
	aoc.Info("adv %s - A=%d / %d -> A=%d", text, solver.a, divisor, result)
	solver.a = result
}

func (solver *day17) bxl(val int) {
	result := solver.b ^ val
	aoc.Info("bxl B=%d ^ %d -> B=%d", solver.b, val, result)
	solver.b = result
}

func (solver *day17) bst(val int) {
	val, text := solver.combo(val)
	result := val % 8
	aoc.Info("bst %s %% 8 -> B=%d", text, result)
	solver.b = result
}

func (solver *day17) jnz(ip int, val int) int {
	if solver.a == 0 {
		aoc.Info("jnz A=%d = 0 - %d -> %d", solver.a, ip, ip+2)
		return ip + 2
	} else {
		aoc.Info("jnz A=%d <> 0 - %d -> %d", solver.a, ip, val)
		return val
	}
}

func (solver *day17) bxc() {
	result := solver.b ^ solver.c
	aoc.Info("bxc B=%d ^ C=%d -> B=%d", solver.b, solver.c, result)
	solver.b = result
}

func (solver *day17) out(out string, val int) string {
	val, text := solver.combo(val)
	result := val % 8
	aoc.Info("out %s - %d", text, result)
	txt := strconv.Itoa(result)
	if len(out) == 0 {
		return txt
	} else {
		return out + "," + txt
	}
}

func (solver *day17) bdv(val int) {
	val, text := solver.combo(val)
	divisor := math.Pow(2, float64(val))
	result := int(math.Trunc(float64(solver.a) / divisor))
	aoc.Info("bdv %s - A=%d / %d -> B=%d", text, solver.a, divisor, result)
	solver.b = result
}

func (solver *day17) cdv(val int) {
	val, text := solver.combo(val)
	divisor := math.Pow(2, float64(val))
	result := int(math.Trunc(float64(solver.a) / divisor))
	aoc.Info("cdv %s - A=%d / %d -> C=%d", text, solver.a, divisor, result)
	solver.c = result
}
