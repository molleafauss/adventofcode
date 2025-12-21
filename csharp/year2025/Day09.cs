using adventofcode.utils;
using Spectre.Console.Cli;

namespace adventofcode.year2025
{
    public class Day09 : ISolver
    {
        private long _part1 = 0;
        private long _part2 = 0;
        private List<GridPos> marks = [];

        public void Parse(string input)
        {
            var parts = input.Split(',', 2);
            // row and col are y and x
            marks.Add(new GridPos(int.Parse(parts[1]), int.Parse(parts[0])));
        }

        public (string? part1, string? part2) Solve()
        {
            for (var i = 0; i < marks.Count; i++)
            {
                for (var j = i + 1; j < marks.Count; j++)
                {
                    // +1 - borders should be counted
                    var area = 1L * Math.Abs(marks[i].Row - marks[j].Row + 1) * Math.Abs(marks[i].Col - marks[j].Col + 1);
                    _part1 = Math.Max(area, _part1);
                }
            }
            return (_part1.ToString(), _part2.ToString());
        }

    }
}
