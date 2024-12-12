package aoc.api;

public interface SolverFactory {
    String getYear();

    Solver createSolver(String day);
}
