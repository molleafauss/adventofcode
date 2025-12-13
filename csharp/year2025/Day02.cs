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

                FindInvalidNumbersInRange(min, max);
            }
        }

        public void FindInvalidNumbersInRange(long min, long max)
        {
            // invalid numbers are made up of sequences of digits repeated as many times as the
            // number length.
            Log.Info($"Searching for invalid numbers in range [{min}, {max}]");
            for (var num = min; num <= max; num++)
            {
                var numStr = num.ToString();
                var digits = numStr.Length;
                for (var i = 2; i <= digits; i++)
                {
                    // if the number can't be split in blocks of size i, then we ignore this split
                    if (digits % i != 0)
                    {
                        continue;
                    }
                    var blockSize = digits / i;
                    if (!IsInvalidNumber(numStr, blockSize))
                    {
                        continue;
                    }
                    Log.Debug($"Found invalid number: {num} (part1 {i == 2})");
                    if (i == 2)
                    {
                        _part1 += num;
                    }
                    _part2 += num;
                    // this number is invalid, don't continue checking
                    break;
                }
            }
        }

        private static bool IsInvalidNumber(string strNum, int blockSize)
        {
            var block = strNum[..blockSize];
            var l = blockSize;
            while (l + blockSize <= strNum.Length)
            {
                if (block != strNum[l..blockSize])
                {
                    return false;
                }
                l += blockSize;
            }
            return true;
        }

        public (string? part1, string? part2) Solve()
        {
            return (_part1.ToString(), _part2.ToString());
        }
    }
}
