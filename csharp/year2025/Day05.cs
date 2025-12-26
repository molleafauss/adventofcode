namespace adventofcode.year2025;

public class Day05 : ISolver
{
    private bool _parseRanges = true;
    private readonly List<(long, long)> _ingredientRanges = [];
    private long _part1;
    private long _part2;

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
                if (ingredient < min || ingredient > max)
                {
                    continue;
                }
                _part1 += 1;
                return;
            }
        }
    }

    public (string? part1, string? part2) Solve()
    {
        _ingredientRanges.Sort();
        var i = 0;
        while (i < _ingredientRanges.Count)
        {
            // if the next range is not contiguous or overlapped, count this range and move forward
            if (i + 1 >= _ingredientRanges.Count || _ingredientRanges[i].Item2 + 1 < _ingredientRanges[i + 1].Item1)
            {
                _part2 += _ingredientRanges[i].Item2 - _ingredientRanges[i].Item1 + 1;
                i++;
                continue;
            }
            // otherwise "absorb" the next range into the current one
            var collapsedRange = (_ingredientRanges[i].Item1, Math.Max(_ingredientRanges[i].Item2, _ingredientRanges[i + 1].Item2));
            _ingredientRanges.RemoveRange(i, 2);
            _ingredientRanges.Insert(i, collapsedRange);
        }
            
        return ($"{_part1}", $"{_part2}");
    }
}