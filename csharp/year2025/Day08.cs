using adventofcode.utils;

namespace adventofcode.year2025
{
    public class Day08 : ISolver
    {
        private class JunctionBox(int x, int y, int z)
        {
            public readonly (int, int, int) Pos = (x, y, z);
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
            Log.Info($"Found {_boxes.Count} boxes, {_connections} iterations");
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
            while (_connections > 0 && _distances.Count > 0)
            {
                _connections--;
                // take the shortest distance and see the two boxes and add them to a circuit
                // if none are part of a circuit, create a new one
                // if one is part of a circuit and the other is not, just add the other
                // if both are part of the same circuit, don't do anything
                // if both are part of different circuits, fail
                var (i, j, _) = _distances[0];
                _distances.RemoveAt(0);

                if (_boxes[i].Circuit == -1 && _boxes[j].Circuit == -1)
                {
                    Log.Info($"[{_connections}] Adding {_boxes[i].Pos} to circuit {circuits.Count}");
                    _boxes[i].Circuit = circuits.Count;
                    Log.Info($"[{_connections}] Adding {_boxes[j].Pos} to circuit {circuits.Count}");
                    _boxes[j].Circuit = circuits.Count;
                    circuits.Add([i, j]);
                }
                else if (_boxes[i].Circuit != -1 && _boxes[j].Circuit == -1)
                {
                    Log.Info($"[{_connections}] Adding {_boxes[j].Pos} to circuit {_boxes[i].Circuit}");
                    _boxes[j].Circuit = _boxes[i].Circuit;
                    circuits[_boxes[i].Circuit].Add(j);
                }
                else if (_boxes[i].Circuit == -1 && _boxes[j].Circuit != -1)
                {
                    Log.Info($"[{_connections}] Adding {_boxes[i].Pos} to circuit {_boxes[j].Circuit}");
                    _boxes[i].Circuit = _boxes[j].Circuit;
                    circuits[_boxes[j].Circuit].Add(i);
                }
                else if (_boxes[i].Circuit != -1 && _boxes[i].Circuit == _boxes[j].Circuit)
                {
                    // nothing to do - connection already made
                    Log.Info($"[{_connections}] {_boxes[i].Pos} and {_boxes[j].Pos} already belong to same circuit ({_boxes[i].Circuit})");
                }
                else
                {
                    // TODO Merge circuits here!!
                    Log.Info($"[{_connections}] Merging circuits: {_boxes[i].Pos}/{_boxes[i].Circuit} <> {_boxes[j].Pos}/{_boxes[j].Circuit}");
                    // hardcode merge j into i and make j empty
                    var c = _boxes[j].Circuit;
                    foreach (var k in circuits[c])
                    {
                        _boxes[k].Circuit = _boxes[i].Circuit;
                        circuits[_boxes[i].Circuit].Add(k);
                    }
                    circuits[c] = [];
                }
            }
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
