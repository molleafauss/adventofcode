package aoc;

public interface SolverFactory {
    String getYear();

    Solver createSolver(String day);
}
