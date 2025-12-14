using adventofcode.utils;

namespace adventofcode.year2025
{
    public class Day04 : ISolver
    {
        private long _part1 = 0;
        private long _part2 = 0;
        private int _width = 0;
        private List<char[]> _map = new();
        
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
            var height = _map.Count;
            for (var row = 0; row < height; row++)
            {
                for (var col = 0; col < _width; col++)
                {
                    if (_map[row][col] != '@')
                    {
                        continue;
                    }
                    var pos = new GridPos(col, row);
                    var surrounding = 0;
                    foreach (var dir in GridPos.AllSurrounding)
                    {
                        var next = pos.Add(dir);
                        if (next.InBounds(_width, height) && _map[next.Row][next.Col] == '@')
                        {
                            surrounding++;
                        }
                    }
                    Log.Info($"({row},{col}) {pos} => {surrounding}");
                    if (surrounding < 4)
                    {
                        _part1++;
                    }
                }
            }
            return (_part1.ToString(), _part2.ToString());
        }
    }
}
