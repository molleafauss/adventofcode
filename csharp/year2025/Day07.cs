using adventofcode.utils;

namespace adventofcode.year2025;

public class Day07 : ISolver
{
    private long _part1;
    private long _part2;
    private int _width;
    private int _height;
    // organize splitter in a list, per row, mapping (col, hit) 
    private List<(int, bool)[]> _splitters = [];
    private GridPos? _start;

    public void Parse(string input)
    {
        if (_width == 0)
        {
            _width = input.Length;
        }
        else if (_width != input.Length)
        {
            throw new ArgumentException($"Illegal input length on line {_height * 1}: {input.Length} expected {_width}");
        }

        var startPos = input.IndexOf('S');
        if (startPos != -1 && _start == null)
        {
            _start = new GridPos(startPos, _height);
        }
        else if (startPos != -1 && _start != null)
        {
            throw new ArgumentException($"Found second start on line {_height * 1}");
        }

        _splitters.Add(Enumerable.Range(0, input.Length)
            .Where(i => input[i] == '^')
            .Select(i => (i, false))
            .ToArray());
        _height++;
    }

    public (string? part1, string? part2) Solve()
    {
        Log.Info($"Found {_splitters.Count} splitters");

        RunBeam();
                        
        return (_part1.ToString(), _part2.ToString());
    }

    private void RunBeam()
    {
        var beams = Enumerable.Repeat(0L, _width).ToArray();
        beams[_start.Col] = 1;
        for (var row = _start.Row; row < _height; row++)
        {
            var splitters = _splitters[row];
            if (splitters.Length == 0)
            {
                continue;
            }
            // find splitters in this row, see how many beams get hit, and build the next beams row
            for (var i = 0; i < splitters.Length; i++)
            {
                var splitterPos = splitters[i].Item1;
                if (beams[splitterPos] == 0)
                {
                    // no beam, continue
                    continue;
                }
                // hit the splitter, mark it
                splitters[i].Item2 = true;
                // add the number beam at the splitter position in the before and after positions
                // unless they're out of bounds
                var curbeams =  beams[splitterPos];
                beams[splitterPos] = 0;
                if (splitterPos > 0)
                {
                    beams[splitterPos - 1] += curbeams;
                }
                if (splitterPos + 1 < _width)
                {
                    beams[splitterPos + 1] += curbeams;
                }
            }
        }
            
        // part 1 is the count of splitter hit
        _part1 = _splitters.Sum(spl => spl.Count(s => s.Item2));
            
        // part 2 is just the count of the beams at the end
        _part2 = beams.Sum();
    }
}