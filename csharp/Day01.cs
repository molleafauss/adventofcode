using System;

namespace adventofcode
{
    public class Day01 : ISolver
    {
        public string DayId => "day01";

        public (string? part1, string? part2) Solve(string input)
        {
            if (string.IsNullOrWhiteSpace(input))
                return (null, null);

            var lines = input.Split(new[] { '\r', '\n' }, System.StringSplitOptions.RemoveEmptyEntries);
            // trivial example: part1 = number of lines, part2 = number of chars
            var part1 = lines.Length.ToString();
            var part2 = input.Length.ToString();
            return (part1, part2);
        }
    }
}
