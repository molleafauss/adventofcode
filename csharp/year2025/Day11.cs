using adventofcode.utils;

namespace adventofcode.year2025
{
    public class Day11 : ISolver
    {
        private long _part1 = 0;
        private long _part2 = 0;
        // name -> connections
        private Dictionary<string, string[]> _devices = new();

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
            // jolly tree visit
            if (!_devices.ContainsKey("you"))
            {
                throw new Exception($"missing 'you' in devices?");
            }
            // build a path, starting with you:
            List<string> path = [ "you" ];
            var queue =  new Queue<List<string>>();
            queue.Enqueue(path);
            while (queue.Count > 0)
            {
                var current =  queue.Dequeue();
                foreach (var conn in _devices[current.Last()])
                {
                    if (conn == "out")
                    {
                        _part1++;
                        continue;
                    }

                    if (current.Contains(conn))
                    {
                        // don't loop
                        continue;
                    }

                    var next = current[..];
                    next.Add(conn);
                    queue.Enqueue(next);
                }
            }

            return (_part1.ToString(), _part2.ToString());
        }
    }
}
