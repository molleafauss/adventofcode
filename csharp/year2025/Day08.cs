using adventofcode.utils;

namespace adventofcode.year2025
{
    public class Day08 : ISolver
    {
        private class JunctionBox(long x, long y, long z)
        {
            public readonly (long, long, long) Pos = (x, y, z);
            public int Circuit = -1;
        }

        private long _part1 = 0;
        private long _part2 = 0;
        private int _connections = 1000;
        private List<JunctionBox> _boxes = [];
        private List<(int, int, double)> _distances = [];

        public void Parse(string input)
        {
            if (input.StartsWith("connections: "))
            {
                _connections = int.Parse(input[12..]);
                return;
            }
            var parts =  input.Split(',', 3);
            _boxes.Add(new JunctionBox(int.Parse(parts[0]), int.Parse(parts[1]), int.Parse(parts[2])));
        }

        public (string? part1, string? part2) Solve()
        {
            Log.Info($"Found {_boxes.Count} boxes, {_connections} max connections, {_distances.Count} distances");
            CalculateDistances();
            MakeCircuits();
            return (_part1.ToString(), _part2.ToString());
        }

        private void CalculateDistances()
        {
            for (int i = 0; i < _boxes.Count; i++)
            {
                for (int j = i + 1; j < _boxes.Count; j++)
                {
                    var distance = Math.Sqrt(
                        Math.Pow(_boxes[i].Pos.Item1 - _boxes[j].Pos.Item1, 2) +
                        Math.Pow(_boxes[i].Pos.Item2 - _boxes[j].Pos.Item2, 2) +
                        Math.Pow(_boxes[i].Pos.Item3 - _boxes[j].Pos.Item3, 2)
                    );
                    _distances.Add((i, j, distance));
                }
            }
            _distances.Sort((a, b) => a.Item3.CompareTo(b.Item3));
        }

        private void MakeCircuits()
        {
            List<List<int>> circuits = [];
            for(var conn = 0; conn < _distances.Count; conn++)
            {
                // take the shortest distance and see the two boxes and add them to a circuit
                // if none are part of a circuit, create a new one
                // if one is part of a circuit and the other is not, just add the other
                // if both are part of the same circuit, don't do anything
                // if both are part of different circuits, merge them
                var (i, j, _) = _distances[conn];

                if (_boxes[i].Circuit == -1 && _boxes[j].Circuit == -1)
                {
                    Log.Debug($"[{conn}] Adding {_boxes[i].Pos} to circuit {circuits.Count}");
                    _boxes[i].Circuit = circuits.Count;
                    Log.Debug($"[{conn}] Adding {_boxes[j].Pos} to circuit {circuits.Count}");
                    _boxes[j].Circuit = circuits.Count;
                    circuits.Add([i, j]);
                }
                else if (_boxes[i].Circuit != -1 && _boxes[j].Circuit == -1)
                {
                    Log.Debug($"[{conn}] Adding {_boxes[j].Pos} to circuit {_boxes[i].Circuit}");
                    _boxes[j].Circuit = _boxes[i].Circuit;
                    circuits[_boxes[i].Circuit].Add(j);
                }
                else if (_boxes[i].Circuit == -1 && _boxes[j].Circuit != -1)
                {
                    Log.Debug($"[{conn}] Adding {_boxes[i].Pos} to circuit {_boxes[j].Circuit}");
                    _boxes[i].Circuit = _boxes[j].Circuit;
                    circuits[_boxes[j].Circuit].Add(i);
                }
                else if (_boxes[i].Circuit != -1 && _boxes[i].Circuit == _boxes[j].Circuit)
                {
                    // nothing to do - connection already made
                    Log.Debug($"[{conn}] {_boxes[i].Pos} and {_boxes[j].Pos} already belong to same circuit ({_boxes[i].Circuit})");
                }
                else
                {
                    // TODO Merge circuits here!!
                    Log.Debug($"[{conn}] Merging circuits: {_boxes[i].Pos}/{_boxes[i].Circuit} <> {_boxes[j].Pos}/{_boxes[j].Circuit}");
                    // hardcode merge j into i and make j empty
                    var c = _boxes[j].Circuit;
                    foreach (var k in circuits[c])
                    {
                        _boxes[k].Circuit = _boxes[i].Circuit;
                        circuits[_boxes[i].Circuit].Add(k);
                    }
                    circuits[c] = [];
                }

                if (conn == _connections - 1)
                {
                    CalculatePart1(circuits[..]);
                }

                if (_boxes.Select(box => box.Circuit).Distinct().Count() == 1)
                {
                    Log.Info($"{conn} Found all connected circuits - last two {_boxes[i].Pos} - {_boxes[j].Pos}");
                    _part2 = _boxes[i].Pos.Item1 * _boxes[j].Pos.Item1;
                    return;
                }
            }

            throw new Exception($"Finished distances without connecting all boxes!");
        }

        private void CalculatePart1(List<List<int>> circuits)
        {
            circuits.Sort((a, b) => b.Count.CompareTo(a.Count));
            
            // multiply the length of the first 3 circuits
            var result = 1;
            for (var i = 0; i < 3 && i < circuits.Count && circuits[i].Count > 0; i++)
            {
                result *= circuits[i].Count;
            }
            _part1 = result;
        }
    }
}
