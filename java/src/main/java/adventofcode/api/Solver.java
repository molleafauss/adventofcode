package adventofcode.api;

public interface Solver {

    /** Parse a line of the input */
    void parse(String line);

    /** Solve the puzzle and return the results for part 1 and part 2 */
    Results solve();
}
