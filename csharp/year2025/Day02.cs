using adventofcode.utils;

namespace adventofcode.year2025
{
    public class Day02 : ISolver
    {
        private long _part1 = 0;
        private long _part2 = 0;
        
        public void Parse(string input)
        {
            foreach (var range in input.Split(","))
            {
                var parts = range.Split('-');
                var min = long.Parse(parts[0]);
                var max = long.Parse(parts[1]);

                foreach (var num in InvalidNumbersInRange(min, max))
                {
                    _part1 += num;
                }
            }
        }

        public static IEnumerable<long> InvalidNumbersInRange(long min, long max)
        {
            // invalid numbers are made up of sequences of digits repeated twice.
            // so if both min and max are made up of an odd number of digits no invalid number exist
            var minDigits = min.ToString().Length;
            var maxDigits = max.ToString().Length;
            if (minDigits % 2 == 1 && maxDigits % 2 == 1)
            {
                return [];
            }
            
            // number from which to start: take the first half of the start of the range or the next
            // power of 10.
            long start;
            if (minDigits % 2 == 0)
            {
                start = long.Parse(min.ToString().Substring(0, minDigits / 2));
            }
            else
            {
                start = (long)Math.Pow(10, minDigits >> 1);
            }

            long end;
            if (maxDigits % 2 == 0)
            {
                end = long.Parse(max.ToString().Substring(0, maxDigits / 2));
            }
            else
            {
                end = (long)Math.Pow(10, maxDigits >> 1) - 1;
            }
            Log.Info($"range {min},{max} => [{start},{end}]");

            var result = new List<long>();
            
            for (var d = start; d <= end; d++)
            {
                //use numbers instead of string concatenation
                var num = long.Parse($"{d}{d}");
                if (num >= min && num <= max)
                {
                    Log.Info($"Invalid num: {num}");
                    result.Add(num);
                }
            }

            return result;
        }

        public (string? part1, string? part2) Solve()
        {
            return (_part1.ToString(), _part2.ToString());
        }
    }
}
