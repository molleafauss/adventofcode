using adventofcode.utils;

namespace adventofcode.year2025;

public class Day09 : ISolver
{
    private long _part1;
    private long _part2;
    private readonly List<GridPos> _marks = [];

    public void Parse(string input)
    {
        var parts = input.Split(',', 2);
        // row and col are y and x
        _marks.Add(new GridPos(int.Parse(parts[1]), int.Parse(parts[0])));
    }

    public (string? part1, string? part2) Solve()
    {
        for (var i = 0; i < _marks.Count; i++)
        {
            for (var j = i + 1; j < _marks.Count; j++)
            {
                // +1 - borders should be counted
                var area = 1L * Math.Abs(_marks[i].Row - _marks[j].Row + 1) * Math.Abs(_marks[i].Col - _marks[j].Col + 1);
                _part1 = Math.Max(area, _part1);
            }
        }
        return ($"{_part1}", $"{_part2}");
    }

}