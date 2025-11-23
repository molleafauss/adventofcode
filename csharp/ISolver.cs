// ISolver defines the contract for AoC day solvers
namespace adventofcode
{
    public interface ISolver
    {
        // Day identifier like "day01"
        string DayId { get; }

        // Solve receives the whole input (contents of the file) and returns tuple of part1 and part2.
        // If a part is not applicable, return null for that part.
        (string? part1, string? part2) Solve(string input);
    }
}
