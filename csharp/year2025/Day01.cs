using adventofcode.utils;

namespace adventofcode.year2025
{
    public class Day01 : ISolver
    {
        private int _dial = 50;
        private int _part1 = 0;
        private int _part2 = 0;
        
        public void Parse(string input)
        {
            var dir = 0;
            if (input.StartsWith("L"))
            {
                dir = -1;
            }
            else if (input.StartsWith("R"))
            {
                dir = 1;
            }
            else
            {
                throw new Exception($"Invalid input: {input}");
            }
            
            var steps = int.Parse(input.Substring(1));
            if (steps > 100)
            {
                // count the turns
                var (q, r) = Math.DivRem(steps, 100);
                _part2 += q;
                steps = r;
            }
            var next = _dial + (dir * steps);

            if (next > 99)
            {
                next -= 100;
            }
            else if (next < 0)
            {
                next += 100;
            }
            
            // if after the move we are at 0, increment part1
            var key1 = 0;
            var key2 = 0;
            if (next == 0)
            {
                key1 = 1;
            }
            // depending on direction, check if we need to increment part2
            // if we are moving forward, if the end becomes before the start, we crossed the 0
            // if we are moving backward, we cross the 0 if
            // - we didn't start at 0 and the end is after the start
            // - we end at 0
            if ((dir == 1 && next < _dial) || (dir == -1 && next > _dial && _dial != 0) || (dir == -1 && next == 0))
            {
                key2 = 1;
            }

            Log.Debug($"{input} {_dial} => {next} [{key1},{key2}]");
            _dial = next;
            _part1 += key1;
            _part2 += key2;
        }

        public (string? part1, string? part2) Solve()
        {
            return (_part1.ToString(), _part2.ToString());
        }
    }
}
