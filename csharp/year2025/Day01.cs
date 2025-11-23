namespace adventofcode.year2025
{
    public class Day01 : ISolver
    {
        private readonly List<int> _left = [];
        private readonly List<int> _right = [];
        
        public void Parse(string input)
        {
            var parts = input.Split("   ");
            if (parts.Length != 2)
            {
                throw new ArgumentException($"Wrong line format: {input}");
            }
            _left.Add(int.Parse(parts[0]));
            _right.Add(int.Parse(parts[1]));
        }

        public (string? part1, string? part2) Solve()
        {
            Console.WriteLine($"location sizes {_left.Count}/{_right.Count}");
            if (_left.Count != _right.Count)
            {
                throw new Exception("location sizes are uneven??");
            }
            _left.Sort();
            _right.Sort();
            var part1 = 0;
            for (int i = 0; i < _left.Count; i++)
            {
                part1 += Math.Abs(_left[i] - _right[i]);
            }

            var similarity = 0;
            var l = 0;
            var r = 0;
            var count = 0;
            while (l < _left.Count)
            {
                if (l > 0 && _left[l] == _left[l-1]) {
                    similarity += _left[l] * count;
                    l++;
                    continue;
                }
                // is the current left item > than the current right? advance right
                while (_right[r] < _left[l])
                {
                    r++;
                }
                // reset count
                count = 0;
                // count how many are equal
                while (_right[r] == _left[l])
                {
                    r++;
                    count++;
                }
                // add to similarity
                similarity += _left[l] * count;
                l++;
            }

            
            return (part1.ToString(), similarity.ToString());
        }
    }
}
