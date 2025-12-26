using adventofcode.utils;

namespace adventofcode.year2025;

public class Day04 : ISolver
{
    private long _part1;
    private long _part2;
    private int _width;
    private readonly List<char[]> _map = new();
        
    public void Parse(string input)
    {
        if (_width != 0 && _width != input.Length)
        {
            throw new Exception($"Invalid input unmatched width: {_width}/{input.Length}: {input}");
        }
        _width = input.Length;
        _map.Add(input.ToCharArray());
    }

    public (string? part1, string? part2) Solve()
    {
        while (true)
        {
            var removable = Enumerable
                .Range(0, _map.Count)
                .SelectMany(row => Enumerable.Range(0, _width)
                    .Where(col => _map[row][col] == '@')
                    .Select(col => (row, col, GridPos.AllSurrounding
                        .Select(dir => new GridPos(col, row).Add(dir))
                        .Count(next =>
                            next.InBounds(_width, _map.Count) &&
                            _map[next.Row][next.Col] == '@'))))
                .Where(tup => tup.Item3 < 4)
                .ToList();
            if (removable.Count == 0)
            {
                break;
            }
            if (_part1 == 0)
            {
                _part1 = removable.Count;
            }
            _part2 += removable.Count;
            // remove all removable
            foreach (var (row, col, _) in removable)
            {
                _map[row][col] = '.';
            }
        }
            
        return (_part1.ToString(), _part2.ToString());
    }
}