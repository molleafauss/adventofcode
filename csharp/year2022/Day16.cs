using System.Diagnostics;
using System.Text.RegularExpressions;
using adventofcode.utils;

namespace adventofcode.year2022;

public class Day16 : ISolver
{
    private static readonly Regex ReValve = new(@"Valve (\S+) has flow rate=(\d+); tunnels? leads? to valves? (.*)", RegexOptions.Compiled);
    private const int Part1Minutes = 30;
    private const int Part2Minutes = 26;
    private const string Start = "AA";

    private readonly List<string> _valvesWithFlow = [];
    private readonly List<Valve> _valves = [];
    private readonly Dictionary<string, Dictionary<string, int>> _distances = new();

    // Parse a single line of the input
    public void Parse(string line)
    {
        if (string.IsNullOrWhiteSpace(line)) return;
        var m = ReValve.Match(line);
        if (m.Success)
        {
            var valve = new Valve(m);
            if (valve.Flow > 0)
            {
                _valvesWithFlow.Add(valve.Name);
                valve.Mask = 1 << _valvesWithFlow.Count;
            }
            _valves.Add(valve);
        }
    }

    // Solve returns part1 and part2 results
    public (string? part1, string? part2) Solve()
    {
        Log.Debug($"Found {_valves.Count} valves to open in {Part1Minutes} minutes");
        CalculateDistances();

        // part 1 - timed
        var sw = Stopwatch.StartNew();
        var onePathSolver = new OnePathSolver(this);
        var bestPath1 = onePathSolver.FindPath(new OnePath(Start));
        sw.Stop();
        Log.Info(string.Format("[1] Found max flow is {0}: {1} ({2} cache hits, {3} calls, {4} cache size) [{5:F3}sec]",
            bestPath1.TotalFlow,
            string.Join("->", bestPath1.Visited),
            onePathSolver.CacheHits,
            onePathSolver.Calls,
            onePathSolver.Cache.Count,
            sw.Elapsed.TotalSeconds));

        // part 2 - timed
        sw.Restart();
        var twoPathSolver = new TwoPathSolver(this);
        var bestPath2 = twoPathSolver.FindPath(new TwoPath(Start));
        sw.Stop();
        Log.Info(string.Format("[2] Found max flow is {0}: {1} / {2} ({3} cache hits, {4} calls, {5} cache size) [{6:F3}sec]",
            bestPath2.TotalFlow,
            string.Join("->", bestPath2.HumanPath),
            string.Join("->", bestPath2.ElePath),
            twoPathSolver.CacheHits,
            twoPathSolver.Calls,
            twoPathSolver.Cache.Count,
            sw.Elapsed.TotalSeconds));

        return (bestPath1.TotalFlow.ToString(), bestPath2.TotalFlow.ToString());
    }

    private void CalculateDistances()
    {
        // add (temporarily) the start into the valves that need to be evaluated
        _valvesWithFlow.Insert(0, Start);
        foreach (var name in _valvesWithFlow)
        {
            var currentDistances = new Dictionary<string, int> { { name, 0 } };
            var visited = new HashSet<string> { name };
            var queue = new Queue<(string, int)>();
            queue.Enqueue((name, 0));
            while (queue.Count > 0)
            {
                var (cave, distance) = queue.Dequeue();
                var valve = GetValve(cave);
                foreach (var next in valve.Connections)
                {
                    if (!visited.Add(next)) continue;
                    if (_valvesWithFlow.Any(v => v == next))
                    {
                        currentDistances[next] = distance + 1;
                    }
                    queue.Enqueue((next, distance + 1));
                }
            }
            currentDistances.Remove(name);
            _distances[name] = currentDistances;
        }
        // remove the temporary start
        _valvesWithFlow.RemoveAt(0);
    }

    private Valve GetValve(string cave) => _valves.First(v => v.Name == cave);

    private sealed class Valve(Match m)
    {
        public List<string> Connections { get; } = m.Groups[3].Value.Split(", ", StringSplitOptions.RemoveEmptyEntries).ToList();
        public string Name { get; } = m.Groups[1].Value;
        public int Flow { get; } = int.Parse(m.Groups[2].Value);
        public int Mask { get; set; }
    }

    private sealed class OnePathSolver(Day16 parent)
    {
        public int Calls;
        public readonly Dictionary<OnePathKey, OnePath> Cache = new();
        public int CacheHits;

        public OnePath FindPath(OnePath path)
        {
            Calls += 1;
            if (Calls % 1000000 == 0) Log.Info($"{Calls} calls, {CacheHits} cache hits...");

            var cave = path.Visited[^1];
            var cacheKey = path.CacheKey();
            if (Cache.TryGetValue(cacheKey, out var cached))
            {
                CacheHits += 1;
                return path.Merge(cached);
            }

            var bestPath = path;
            foreach (var name in parent._valvesWithFlow)
            {
                var valve = parent.GetValve(name);
                if ((path.OpenValves & valve.Mask) != 0) continue;
                var distance = parent._distances[cave][name];
                var next = path.Next(valve, distance);
                if (next.Elapsed >= Part1Minutes) continue;
                var subBest = FindPath(next);
                if (subBest.TotalFlow > bestPath.TotalFlow) bestPath = subBest;
            }

            Cache[cacheKey] = bestPath.Diff(path);
            return bestPath;
        }
    }

    private record OnePathKey(string Name, int Elapsed, int Valves);

    private sealed class OnePath
    {
        public List<string> Visited { get; }
        public int OpenValves { get; }
        public int Elapsed { get; }
        public int TotalFlow { get; }

        public OnePath(string start)
        {
            Visited = [start];
            OpenValves = 0;
            Elapsed = 0;
            TotalFlow = 0;
        }

        private OnePath(List<string> visited, int openValves, int elapsed, int totalFlow)
        {
            Visited = visited;
            OpenValves = openValves;
            Elapsed = elapsed;
            TotalFlow = totalFlow;
        }

        public OnePathKey CacheKey() => new(Visited[^1], Elapsed, OpenValves);

        public OnePath Merge(OnePath other)
        {
            var visited = new List<string>(Visited);
            visited.AddRange(other.Visited);
            return new OnePath(visited, OpenValves, Elapsed + other.Elapsed, TotalFlow + other.TotalFlow);
        }

        public OnePath Next(Valve valve, int distance)
        {
            var visited = new List<string>(Visited) { valve.Name };
            var elapsed = Elapsed + distance + 1;
            var flow = (Part1Minutes - elapsed) * valve.Flow;
            return new OnePath(visited, OpenValves | valve.Mask, elapsed, TotalFlow + flow);
        }

        public OnePath Diff(OnePath start)
        {
            var visited = new List<string>(Visited.GetRange(start.Visited.Count, Visited.Count - start.Visited.Count));
            return new OnePath(visited, OpenValves, Elapsed - start.Elapsed, TotalFlow - start.TotalFlow);
        }
    }


    private sealed class TwoPathSolver(Day16 parent)
    {
        public int Calls;
        public readonly Dictionary<TwoPathKey, TwoPath> Cache = new();
        public int CacheHits;

        public TwoPath FindPath(TwoPath path)
        {
            Calls += 1;
            if (Calls % 1000000 == 0) Log.Info($"{Calls} calls, {CacheHits} cache hits...");

            var manPos = path.HumanPath[^1];
            var elePos = path.ElePath[^1];
            var cacheKey = path.CacheKey();

            if (Cache.TryGetValue(cacheKey, out var cached))
            {
                CacheHits += 1;
                return path.Merge(cached);
            }

            var bestPath = path;
            foreach (var name in parent._valvesWithFlow)
            {
                var valve = parent.GetValve(name);
                if ((path.OpenValves & valve.Mask) != 0) continue;

                // move human
                var distance = parent._distances[manPos][name];
                var next = path.NextHuman(valve, distance);
                if (next.Elapsed < Part2Minutes)
                {
                    var subBest = FindPath(next);
                    if (subBest.TotalFlow > bestPath.TotalFlow) bestPath = subBest;
                }

                // move elephant
                distance = parent._distances[elePos][name];
                next = path.NextElephant(valve, distance);
                if (next.Elapsed < Part2Minutes)
                {
                    var subBest = FindPath(next);
                    if (subBest.TotalFlow > bestPath.TotalFlow) bestPath = subBest;
                }
            }

            Cache[cacheKey] = bestPath.Diff(path);
            return bestPath;
        }
    }

    private record TwoPathKey(string HumanPos, int HumanElapsed, string ElePos, int EleElapsed, int Valves);

    private sealed class TwoPath
    {
        public List<string> HumanPath { get; }
        private int HumanElapsed { get; }
        public List<string> ElePath { get; }
        private int EleElapsed { get; }
        public int OpenValves { get; }
        public int Elapsed { get; }
        public int TotalFlow { get; }

        private TwoPath(List<string> humanPath, int humanElapsed, List<string> elePath, int eleElapsed, int openValves, int elapsed, int totalFlow)
        {
            HumanPath = humanPath;
            HumanElapsed = humanElapsed;
            ElePath = elePath;
            EleElapsed = eleElapsed;
            OpenValves = openValves;
            Elapsed = elapsed;
            TotalFlow = totalFlow;
        }

        public TwoPath(string start)
        {
            HumanPath = [start];
            ElePath = [start];
            HumanElapsed = 0;
            EleElapsed = 0;
            OpenValves = 0;
            Elapsed = 0;
            TotalFlow = 0;
        }

        public TwoPathKey CacheKey() => new(HumanPath[^1], HumanElapsed, ElePath[^1], EleElapsed, OpenValves);

        public TwoPath Merge(TwoPath other)
        {
            var humanPath = new List<string>(HumanPath);
            humanPath.AddRange(other.HumanPath);
            var elePath = new List<string>(ElePath);
            elePath.AddRange(other.ElePath);
            return new TwoPath(humanPath, HumanElapsed + other.HumanElapsed, elePath, EleElapsed + other.EleElapsed, OpenValves, Elapsed + other.Elapsed, TotalFlow + other.TotalFlow);
        }

        public TwoPath NextHuman(Valve valve, int distance)
        {
            var humanPath = new List<string>(HumanPath) { valve.Name };
            var elePath = new List<string>(ElePath);
            var elapsed = HumanElapsed + distance + 1;
            var flow = (Part2Minutes - elapsed) * valve.Flow;
            return new TwoPath(humanPath, elapsed, elePath, EleElapsed, OpenValves | valve.Mask, Math.Max(elapsed, EleElapsed), TotalFlow + flow);
        }

        public TwoPath NextElephant(Valve valve, int distance)
        {
            var humanPath = new List<string>(HumanPath);
            var elePath = new List<string>(ElePath) { valve.Name };
            var elapsed = EleElapsed + distance + 1;
            var flow = (Part2Minutes - elapsed) * valve.Flow;
            return new TwoPath(humanPath, HumanElapsed, elePath, elapsed, OpenValves | valve.Mask, Math.Max(elapsed, HumanElapsed), TotalFlow + flow);
        }

        public TwoPath Diff(TwoPath start)
        {
            var humanPath = new List<string>(HumanPath.GetRange(start.HumanPath.Count, HumanPath.Count - start.HumanPath.Count));
            var elePath = new List<string>(ElePath.GetRange(start.ElePath.Count, ElePath.Count - start.ElePath.Count));
            return new TwoPath(humanPath, HumanElapsed - start.HumanElapsed, elePath, EleElapsed - start.EleElapsed, OpenValves, Elapsed - start.Elapsed, TotalFlow - start.TotalFlow);
        }
    }
}