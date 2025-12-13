using adventofcode.utils;

namespace adventofcode.year2025
{
    public class Day03 : ISolver
    {
        private long _part1 = 0;
        private long _part2 = 0;
        
        public void Parse(string input)
        {
            // scan through the string and find the highest digit first, then find the highest
            // happening after the first position
            int p = 0;
            int d1 = 0;
            int d2 = 0;
            for (var i = 0; i < input.Length - 1; i++)
            {
                var d = input[i] - '0';
                if (d > d1)
                {
                    p = i;
                    d1 = d;
                }
            }
            // second battery position
            for (var i = p + 1; i < input.Length; i++)
            {
                var d = input[i] - '0';
                if (d > d2)
                {
                    d2 = d;
                }
            }
            var jolt = d1 * 10 + d2;
            Log.Info($"{input} => {jolt}");
            _part1 += d1 * 10 + d2;
        }

        public (string? part1, string? part2) Solve()
        {
            return (_part1.ToString(), _part2.ToString());
        }
    }
}
