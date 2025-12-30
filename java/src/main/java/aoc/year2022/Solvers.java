package aoc.year2022;

import aoc.api.Solver;
import aoc.api.SolverFactory;

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
        else if (Objects.equals(day, "day16opt")) {
            return new Day16opt();
        }
        throw new UnsupportedOperationException("Unknown day: " + day);
    }
}
