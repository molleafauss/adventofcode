using adventofcode.utils;

namespace adventofcode.year2025
{
    // note: the test input is the one from part2
    public class Day11 : ISolver
    {
        private long _part1 = 0;
        private long _part2 = 0;

        // name -> connections
        private Dictionary<string, string[]> _devices = new();
        private Dictionary<string, (int, bool, bool)> _foundCache = new();

        public void Parse(string input)
        {
            var p = input.IndexOf(": ");
            if (p == -1)
            {
                throw new Exception($"invalid input: no name {input}");
            }
            _devices.Add(input[..p], input[(p + 2)..].Split(" "));
        }

        public (string? part1, string? part2) Solve()
        {
            _part1 = new Finder(_devices, []).CountPaths("you", []);
            Log.Info($"Finished part1: {_part1}");
            _part2 = new Finder(_devices, ["dac", "fft"])
                .CountPaths("svr", Enumerable.Repeat(false, 2).ToArray());
            return (_part1.ToString(), _part2.ToString());
        }

        public class Step(string from, bool[] found)
        {
            private string From { get; } = from;
            private bool[] Found { get; } = found;

            public override bool Equals(object? b)
            {
                // if (other == null) return false;
                return b is Step other && From == other.From && Found.SequenceEqual(other.Found);
            }

            public override int GetHashCode()
            {
                var hash = From.GetHashCode();
                foreach (var b in Found)
                {
                    hash = HashCode.Combine(hash, b);
                }
                return hash;
            }

            public override string ToString()
            {
                return $"{From} / {string.Join(",", Found)}";
            }
        }

        public class Finder(Dictionary<string, string[]> devices, List<string> required)
        {
            private readonly Dictionary<Step, long> _cache = new();
            private long _calls;

            public long CountPaths(string from, bool[] found)
            {
                _calls++;
                if (_calls % 1000000 == 0)
                {
                    Log.Info($"Calls: {_calls}");
                }

                var newFound = found[..];
                var q = required.IndexOf(from);
                if (q != -1)
                {
                    newFound[q] = true;
                }

                var allFound = newFound.All(b => b);
                var newStep = new Step(from, newFound);
                // did we get here before with the same status?
                if (_cache.TryGetValue(newStep, out var value))
                {
                    return value;
                }
                var numPaths = 0L;
                foreach (var conn in devices[from])
                {
                    if (conn == "out")
                    {
                        // if we reach the end, return 1 only if went through all the required targets
                        if (allFound)
                        {
                            numPaths++;
                        }
                        continue;
                    }
                    Log.Info($"Exploring: {from} -> {conn} {string.Join(",", newFound)}");
                    numPaths += CountPaths(conn, newFound);
                }
                Log.Info($"{from}: {numPaths}");
                _cache.Add(newStep, numPaths);
                return numPaths;
            }
        }
    }
}
