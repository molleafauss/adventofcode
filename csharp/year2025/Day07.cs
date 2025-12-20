using adventofcode.utils;

namespace adventofcode.year2025
{
    public class Day07 : ISolver
    {
        private long _part1 = 0;
        private long _part2 = 0;
        private int _width = 0;
        private int _height = 0;
        private ISet<GridPos> _splitters = new HashSet<GridPos>();
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

            var is_start = input.IndexOf('S');
            if (is_start != -1 && _start == null)
            {
                _start = new GridPos(is_start, _height);
            }
            else if (is_start != -1 && _start != null)
            {
                throw new ArgumentException($"Found second start on line {_height * 1}");
            }

            var p = 0;
            while (p < input.Length)
            {
                var c = input.IndexOf('^', p);
                if (c == -1)
                {
                    break;
                }
                _splitters.Add(new GridPos(c, _height));
                p = c + 1;
            }

            _height++;
        }

        public (string? part1, string? part2) Solve()
        {
            Log.Info($"Found {_splitters.Count} splitters");
            
            // find splits, starting from start and always going down
            var beams = new Queue<GridPos>();
            beams.Enqueue(_start);
            // avoid duplicate paths
            var paths = new HashSet<GridPos>();
            while (beams.Count > 0)
            {
                var beam = beams.Dequeue();
                if (paths.Contains(beam))
                {
                    // seen this already - ignore
                    continue;
                }
                paths.Add(beam);
                var next = beam.Add(GridPos.MoveU);
                if (!next.InBounds(_width, _height))
                {
                    // ignore if we're out of the grid
                    continue;
                }
                if (!_splitters.Contains(next))
                {
                    beams.Enqueue(next);
                    continue;
                }
                // we split here
                _part1++;
                var splitted = next.Add(GridPos.MoveR);
                if (!paths.Contains(next))
                {
                    beams.Enqueue(splitted);
                    Log.Info($"Splitting -> {beam} -> {splitted} (splitter at {next})");
                }
                splitted = next.Add(GridPos.MoveL);
                if (!paths.Contains(next))
                {
                    beams.Enqueue(splitted);
                    Log.Info($"Splitting -> {beam} -> {splitted} (splitter at {next})");
                }
            }
            
            return (_part1.ToString(), _part2.ToString());
        }
    }
}
