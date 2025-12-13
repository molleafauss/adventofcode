using adventofcode.utils;

namespace adventofcode.year2025
{
    public class Day03 : ISolver
    {
        private long _part1 = 0;
        private long _part2 = 0;
        
        public void Parse(string input)
        {
            var jolt = FindLargestJolt(input, 2);
            Log.Info($"J2 => {jolt}");
            _part1 += jolt; 
            jolt = FindLargestJolt(input, 12);
            Log.Info($"J12 => {jolt}");
            _part2 += jolt;
        }

        private long FindLargestJolt(string input, int size)
        {
            // start from the last "size" digits and then proceed "backward": take the highest digit
            // and find the highest (if present) in the positions from 0 to Lenght - size, then do
            // the same with the others, until a max is found.
            var jolt = new Jolt(input, size);

            // loop on each digit starting from the "highest"
            for (var d = 0; d < size; d++)
            {
                // loop on the input "backward" from the digit position
                for (var i = jolt.Digits[d].Position - 1; i >= jolt.GetMinPos(d); i--)
                {
                    var v = input[i] - '0';
                    if (v >= jolt.Digits[d].Value)
                    {
                        // found a higher digit, update the jolt
                        jolt.ReplaceDigit(d, v, i);
                    }
                }
            }
            return jolt.Value;
        }

        public (string? part1, string? part2) Solve()
        {
            return (_part1.ToString(), _part2.ToString());
        }
    }

    record Digit(int Value, int Position);

    class Jolt
    {
        internal long Value { get; private set; }
        internal readonly Digit[] Digits;

        internal Jolt(string input, int size)
        {
            Digits = new Digit[size];
            for (var i = 0; i < size; i++)
            {
                var v = input[^(size - i)] - '0';
                Digits[i] = new Digit(v, input.Length - size + i);
                Value = Value * 10 + v;
            }
        }
        
        public override string ToString()
        {
            var rv = $"{Value} [";
            foreach (var d in Digits)
            {
                rv += $" {d.Value}({d.Position}),";
            }
            return rv + "]";
        }

        public void ReplaceDigit(int digit, int val, int pos)
        {
            Value -= Digits[digit].Value * (long)Math.Pow(10, Digits.Length - digit - 1);
            Value += val * (long)Math.Pow(10, Digits.Length - digit - 1);
            Digits[digit] = new Digit(val, pos);
        }

        public int GetMinPos(int digit)
        {
            return digit == 0 ? 0 : Digits[digit - 1].Position + 1;
        }
    }
}
