package aoc.api;

public interface Solver {
    void parse(String line);
    Results solve();
}
