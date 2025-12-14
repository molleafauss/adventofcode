using adventofcode.utils;

namespace adventofcode.year2025
{
    public class Day05 : ISolver
    {
        private bool _parseRanges = true;
        private List<(long, long)> _ingredientRanges = [];
        private long _part1 = 0;
        private long _part2 = 0;
        
        public void Parse(string input)
        {
            if (input == "")
            {
                _parseRanges = false;
                return;
            }

            if (_parseRanges)
            {
                var p = input.Split("-", 2);
                _ingredientRanges.Add((long.Parse(p[0]), long.Parse(p[1])));
            }
            else
            {
                var ingredient = long.Parse(input);
                foreach (var (min, max) in _ingredientRanges)
                {
                    if (ingredient >= min && ingredient <= max)
                    {
                        _part1 += 1;
                        return;
                    }
                }
            }
        }

        public (string? part1, string? part2) Solve()
        {
            return (_part1.ToString(), _part2.ToString());
        }
    }
}
