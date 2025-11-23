// ISolver defines the contract for AoC day solvers
namespace adventofcode
{
    public interface ISolver
    {
        // Parse a line of the input
        void Parse(string input);

        // Solve the puzzle and return the results for part 1 and part 2
        (string? part1, string? part2) Solve();
    }
}
