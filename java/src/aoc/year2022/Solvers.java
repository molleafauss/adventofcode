package aoc.year2022;

import aoc.Solver;
import aoc.SolverFactory;

import java.util.Objects;

public class Solvers implements SolverFactory {
    @Override
    public String getYear() {
        return "2022";
    }

    @Override
    public Solver createSolver(String day) {
        if (Objects.equals(day, "day16")) {
            return new Day16();
        }
        throw new UnsupportedOperationException("Unknown day: " + day);
    }
}
