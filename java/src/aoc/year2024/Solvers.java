package aoc.year2024;

import aoc.api.Solver;
import aoc.api.SolverFactory;

import java.util.Objects;

public class Solvers implements SolverFactory {
    @Override
    public String getYear() {
        return "2024";
    }

    @Override
    public Solver createSolver(String day) {
        if (Objects.equals(day, "day19")) {
            return new Day19();
        }
        throw new UnsupportedOperationException("Unknown day: " + day);
    }
}
