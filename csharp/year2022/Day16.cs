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

    private readonly Dictionary<string, Valve> _valves = new();
    private readonly Dictionary<string, List<string>> _connections = new();

    // Parse a single line of the input
    public void Parse(string line)
    {
        if (string.IsNullOrWhiteSpace(line)) return;
        var m = ReValve.Match(line);
        if (m.Success)
        {
            var valve = new Valve(m);
            _connections[valve.Name] = m.Groups[3].Value.Split(", ", StringSplitOptions.RemoveEmptyEntries).ToList();
            _valves[valve.Name] = valve;
        }
    }

    // Solve returns part1 and part2 results
    public (string? part1, string? part2) Solve()
    {
        Log.Debug($"Found {_valves.Count} valves to open in {Part1Minutes} minutes");
        
        var valvesWithFlow = PrecacheValues();

        // part 1 - timed
        var sw = Stopwatch.StartNew();
        var onePathSolver = new OnePathSolver(valvesWithFlow);
        var bestPath1 = onePathSolver.FindPath(new OnePath(_valves[Start]));
        sw.Stop();
        Log.Info(
            $"[1] Found max flow is {bestPath1.TotalFlow}: {string.Join("->", bestPath1.Visited)} ({onePathSolver.CacheHits} cache hits, {onePathSolver.Calls} calls, {onePathSolver.Cache.Count} cache size) [{sw.Elapsed.TotalSeconds:F3}sec]");

        // part 2 - timed
        sw.Restart();
        var twoPathSolver = new TwoPathSolver(valvesWithFlow);
        var bestPath2 = twoPathSolver.FindPath(new TwoPath(valvesWithFlow.Count));
        sw.Stop();
        Log.Info(
            $"[2] Found max flow is {bestPath2.TotalFlow}: {string.Join("->", bestPath2.HumanPath)} / {string.Join("->", bestPath2.ElePath)} ({twoPathSolver.CacheHits} cache hits, {twoPathSolver.Calls} calls, {twoPathSolver.Cache.Count} cache size) [{sw.Elapsed.TotalSeconds:F3}sec]");
        return (bestPath1.TotalFlow.ToString(), bestPath2.TotalFlow.ToString());
    }

    private List<Valve> PrecacheValues()
    {
        List<Valve> valvesWithFlow = [];
        foreach (var valve in _valves.Values.Where(valve => valve.Flow > 0))
        {
            valve.Mask = 1 << valvesWithFlow.Count;
            valvesWithFlow.Add(valve);
            // id = 0 will be the START valve
            valve.Id = (byte) valvesWithFlow.Count;
        }
        valvesWithFlow.Insert(0, _valves[Start]);

        foreach (var curr in valvesWithFlow)
        {
            curr.Tunnels = new byte[valvesWithFlow.Count];
            var visited = new HashSet<string> { curr.Name };
            var queue = new Queue<(Valve, int)>();
            queue.Enqueue((curr, 0));
            while (queue.Count > 0)
            {
                var (valve, distance) = queue.Dequeue();
                foreach (var next in _connections[valve.Name])
                {
                    if (!visited.Add(next)) continue;
                    if (valvesWithFlow.Contains(_valves[next]))
                    {
                        curr.Tunnels[_valves[next].Id] = (byte)(distance + 1);
                    }
                    queue.Enqueue((_valves[next], distance + 1));
                }
            }
        }
        
        return valvesWithFlow;
    }

    private sealed class Valve(Match m)
    {
        public byte Id;
        public byte[] Tunnels = [];
        public readonly string Name = m.Groups[1].Value;
        public readonly int Flow = int.Parse(m.Groups[2].Value);
        public int Mask;
    }

    private sealed class OnePathSolver(List<Valve> valvesWithFlow)
    {
        public int Calls;
        public readonly Dictionary<OnePathKey, OnePath> Cache = new();
        public int CacheHits;

        public OnePath FindPath(OnePath path)
        {
            Calls += 1;
            if (Calls % 1000000 == 0) Log.Info($"{Calls} calls, {CacheHits} cache hits...");

            var cacheKey = path.CacheKey();
            if (Cache.TryGetValue(cacheKey, out var cached))
            {
                CacheHits += 1;
                return path.Merge(cached);
            }

            var currValve = path.Visited[^1];
            var bestPath = path;
            foreach (var valve in valvesWithFlow)
            {
                if (valve.Flow == 0) continue;
                if ((path.OpenValves & valve.Mask) != 0) continue;
                var distance = currValve.Tunnels[valve.Id];
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
        public readonly List<Valve> Visited;
        public int OpenValves { get; }
        public int Elapsed { get; }
        public int TotalFlow { get; }

        public OnePath(Valve start)
        {
            Visited = [start];
            OpenValves = 0;
            Elapsed = 0;
            TotalFlow = 0;
        }

        private OnePath(List<Valve> visited, int openValves, int elapsed, int totalFlow)
        {
            Visited = visited;
            OpenValves = openValves;
            Elapsed = elapsed;
            TotalFlow = totalFlow;
        }

        public OnePathKey CacheKey() => new(Visited[^1].Name, Elapsed, OpenValves);

        public OnePath Merge(OnePath other)
        {
            var visited = Visited[..];
            visited.AddRange(other.Visited);
            return new OnePath(visited, OpenValves, Elapsed + other.Elapsed, TotalFlow + other.TotalFlow);
        }

        public OnePath Next(Valve valve, int distance)
        {
            var visited = new List<Valve>(Visited) { valve };
            var elapsed = Elapsed + distance + 1;
            var flow = (Part1Minutes - elapsed) * valve.Flow;
            return new OnePath(visited, OpenValves | valve.Mask, elapsed, TotalFlow + flow);
        }

        public OnePath Diff(OnePath start)
        {
            var visited = Visited[start.Visited.Count..];
            return new OnePath(visited, OpenValves, Elapsed - start.Elapsed, TotalFlow - start.TotalFlow);
        }
    }


    private sealed class TwoPathSolver(List<Valve> valvesWithFlow)
    {
        public int Calls;
        public readonly Dictionary<TwoPathKey, TwoPath> Cache = new();
        public int CacheHits;

        public TwoPath FindPath(TwoPath path)
        {
            Calls += 1;
            if (Calls % 1000000 == 0) Log.Info($"{Calls} calls, {CacheHits} cache hits...");

            var cacheKey = path.CacheKey();
            if (Cache.TryGetValue(cacheKey, out var cached))
            {
                CacheHits += 1;
                return path.Merge(cached);
            }

            var manPos = path.HumanPath[path.HumanPathPos];
            var manValve = valvesWithFlow[manPos];
            var elePos = path.ElePath[path.ElePathPos];
            var eleValve = valvesWithFlow[elePos];
            
            var bestPath = path;
            foreach (var valve in valvesWithFlow)
            {
                if (valve.Flow == 0) continue;
                if ((path.OpenValves & valve.Mask) != 0) continue;

                // move human
                var distance = manValve.Tunnels[valve.Id];
                var next = path.NextHuman(valve, distance);
                if (next.Elapsed < Part2Minutes)
                {
                    var subBest = FindPath(next);
                    if (subBest.TotalFlow > bestPath.TotalFlow) bestPath = subBest;
                }

                // move elephant
                distance = eleValve.Tunnels[valve.Id];
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

    private record TwoPathKey(byte HumanPos, byte HumanElapsed, byte ElePos, byte EleElapsed, int Valves);

    private sealed class TwoPath
    {
        public byte[] HumanPath { get; }
        public byte HumanPathPos { get; }
        private byte HumanElapsed { get; }
        public byte[] ElePath { get; }
        public byte ElePathPos { get; }
        private byte EleElapsed { get; }
        public int OpenValves { get; }
        public byte Elapsed { get; }
        public int TotalFlow { get; }

        public TwoPath(int size) {
            HumanPath = new byte[size + 1];
            HumanPath[0] = 0;
            ElePath = new byte[size + 1];
            ElePath[0] = 0;
        }

        private TwoPath(byte[] humanPath, byte humanPathPos, byte humanElapsed, byte[] elePath, byte elePathPos, byte eleElapsed, int openValves, byte elapsed, int totalFlow)
        {
            HumanPath = humanPath;
            HumanPathPos = humanPathPos;
            HumanElapsed = humanElapsed;
            ElePath = elePath;
            ElePathPos = elePathPos;
            EleElapsed = eleElapsed;
            OpenValves = openValves;
            Elapsed = elapsed;
            TotalFlow = totalFlow;
        }

        public TwoPathKey CacheKey() => new(HumanPath[HumanPathPos], HumanElapsed, ElePath[ElePathPos], EleElapsed, OpenValves);

        public TwoPath Merge(TwoPath other)
        {
            var humanPath = new byte[HumanPath.Length];
            if (HumanPathPos > 0) HumanPath[..HumanPathPos].CopyTo(humanPath, 0);
            if (other.HumanPathPos > 0) other.HumanPath[..other.HumanPathPos].CopyTo(humanPath, HumanPathPos);
            var elePath = new byte[ElePath.Length];
            if (ElePathPos > 0) ElePath[..ElePathPos].CopyTo(elePath, 0);
            if (other.ElePathPos > 0)
            {
                other.ElePath[..other.ElePathPos].CopyTo(elePath, ElePathPos);
            }
            return new TwoPath(
                humanPath, 
                (byte)(HumanPathPos + other.HumanPathPos + 1), 
                (byte)(HumanElapsed + other.HumanElapsed), 
                elePath,
                (byte)(ElePathPos + other.ElePathPos + 1),
                (byte)(EleElapsed + other.EleElapsed), 
                OpenValves, 
                (byte)(Elapsed + other.Elapsed), 
                TotalFlow + other.TotalFlow);
        }

        public TwoPath NextHuman(Valve valve, int distance)
        {
            var humanPath = HumanPath[..];
            humanPath[HumanPathPos + 1] = valve.Id;
            var elePath = ElePath[..];
            var elapsed = HumanElapsed + distance + 1;
            var flow = (Part2Minutes - elapsed) * valve.Flow;
            return new TwoPath(
                humanPath,
                (byte)(HumanPathPos + 1),
                (byte)elapsed, 
                elePath,
                ElePathPos,
                EleElapsed, 
                OpenValves | valve.Mask, 
                (byte)Math.Max(elapsed, EleElapsed), 
                TotalFlow + flow);
        }

        public TwoPath NextElephant(Valve valve, int distance)
        {
            var humanPath = HumanPath[..];
            var elePath = ElePath[..];
            elePath[ElePathPos + 1] = valve.Id;
            var elapsed = EleElapsed + distance + 1;
            var flow = (Part2Minutes - elapsed) * valve.Flow;
            return new TwoPath(
                humanPath,
                HumanPathPos,
                HumanElapsed, 
                elePath,
                (byte)(ElePathPos + 1),
                (byte)elapsed,
                OpenValves | valve.Mask, 
                (byte)Math.Max(elapsed, HumanElapsed), 
                TotalFlow + flow);
        }

        public TwoPath Diff(TwoPath start)
        {
            var humanPath = new byte[HumanPath.Length];
            if (HumanPathPos > start.HumanPathPos)
            {
                HumanPath[(HumanPathPos - start.HumanPathPos)..HumanPathPos].CopyTo(humanPath, 0);
            }
            var elePath = new byte[ElePath.Length];
            if (ElePathPos > start.ElePathPos)
            {
                ElePath[(ElePathPos - start.ElePathPos)..ElePathPos].CopyTo(elePath, 0);
            }
            return new TwoPath(
                humanPath,
                (byte) (HumanPathPos - start.HumanPathPos),
                (byte) (HumanElapsed - start.HumanElapsed),
                elePath,
                (byte) (ElePathPos - start.ElePathPos),
                (byte) (EleElapsed - start.EleElapsed),
                OpenValves,
                (byte) (Elapsed - start.Elapsed),
                TotalFlow - start.TotalFlow
            );
        }
    }
}