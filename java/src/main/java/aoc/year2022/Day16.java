package aoc.year2022;

import aoc.api.Results;
import aoc.api.Solver;
import aoc.util.Log;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.Set;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day16 implements Solver {

    private static final Pattern RE_VALVE = Pattern.compile("Valve (\\S+) has flow rate=(\\d+); tunnels? leads? to valves? (.*)");
    static final int PART1_MINUTES = 30;
    static final int PART2_MINUTES = 26;
    private static final String START = "AA";

    private final Map<String, Valve> valves = new HashMap<>();
    private final Map<String, List<String>> connections = new HashMap<>();

    @Override
    public void parse(String line) {
        if (line.isEmpty()) {
            return;
        }
        Matcher m = RE_VALVE.matcher(line);
        if (m.matches()) {
            var valve = new Valve(m);
            connections.put(valve.name, Arrays.stream(m.group(3).split(", ")).toList());
            valves.put(valve.name, valve);
        }
    }

    @Override
    public Results solve() {
        Log.info("Found %d valves to open in %d minutes", valves.size(), PART1_MINUTES);

        var valvesWithFlow = precacheValues();

        Log.info("Valves with flow: %d", valvesWithFlow.size());

        // part 1 - timed
        var t0 = System.currentTimeMillis();
        var onePathSolver = new OnePathSolver(valvesWithFlow);
        var bestPath1 = onePathSolver.findPath(new OnePath(valves.get(START)));
        var t1 = System.currentTimeMillis();
        var path = bestPath1.visited.stream().map(v -> v.name).toList();
        Log.info("[1] Found max flow is %d: %s (%d cache hits, %d calls, %d cache size) [%.3fsec]",
                bestPath1.totalFlow, path, onePathSolver.cacheHits, onePathSolver.calls,
                onePathSolver.cache.size(), (t1 - t0) / 1000.0);

        // part 2 - timed
        t0 = System.currentTimeMillis();
        var twoPathSolver = new TwoPathSolver(valvesWithFlow);
        var bestPath2 = twoPathSolver.findPath(new TwoPath(valvesWithFlow.size()));
        t1 = System.currentTimeMillis();
        var humanPath = valvePath(valvesWithFlow, bestPath2.humanPath,
                bestPath2.humanPathPos);
        var elePath = valvePath(valvesWithFlow, bestPath2.elePath, bestPath2.elePathPos);
        Log.info("[2] Found max flow is %d: %s / %s (%d cache hits, %d calls, %d cache size) [%.3fsec]",
                bestPath2.totalFlow, humanPath, elePath, twoPathSolver.cacheHits,
                twoPathSolver.calls, twoPathSolver.cache.size(), (t1 - t0) / 1000.0);
        return new Results(String.valueOf(bestPath1.totalFlow), String.valueOf(bestPath2.totalFlow));
    }

    private List<String> valvePath(List<Valve> valvesWithFlow, byte[] visited, byte length) {
        var path = new ArrayList<String>();
        for (int i = 0; i <= length; i++) {
            path.add(valvesWithFlow.get(visited[i]).name);
        }
        return path;
    }

    private List<Valve> precacheValues() {
        List<Valve> valvesWithFlow = new ArrayList<>();
        // add to the valves with flow and set the valve mask
        for (Valve valve : valves.values()) {
            if (valve.flow > 0) {
                valve.mask = 1 << valvesWithFlow.size();
                valvesWithFlow.add(valve);
                // id = 0 will be the START valve
                valve.id = (byte) valvesWithFlow.size();
            }
        }
        // always add the START valve as first
        valvesWithFlow.addFirst(valves.get(START));

        calculateDistances(valvesWithFlow);

        return valvesWithFlow;
    }

    private record DistanceWrapper(Valve cave, int distance) {
    }

    private void calculateDistances(List<Valve> valvesWithFlow) {
        // create a map associating valves and their connections;
        var tunnels = makeTunnelMap();

        for (Valve curr : valvesWithFlow) {
            // oh, java, when will you add a simple mutable map/list/set initializer?
            curr.tunnels = new byte[valvesWithFlow.size()];
            var visited = new HashSet<>(Set.of(curr.name));
            var queue = new ArrayList<DistanceWrapper>();
            queue.add(new DistanceWrapper(curr, 0));
            while (!queue.isEmpty()) {
                var wrapper = queue.removeFirst();
                var valve = wrapper.cave;
                for (Valve next : tunnels.get(valve.name)) {
                    if (visited.contains(next.name)) {
                        continue;
                    }
                    visited.add(next.name);
                    if (valvesWithFlow.contains(next)) {
                        curr.tunnels[next.id] = (byte) (wrapper.distance + 1);
                    }
                    queue.add(new DistanceWrapper(next, wrapper.distance + 1));
                }
            }
        }
    }

    private Map<String, List<Valve>> makeTunnelMap() {
        var tunnels = new HashMap<String, List<Valve>>();
        for (Entry<String, List<String>> conns : connections.entrySet()) {
            tunnels.put(conns.getKey(),
                    conns.getValue().stream().map(valves::get).toList());
        }
        return tunnels;
    }
}

class Valve {
    byte id = 0;
    byte[] tunnels;
    final String name;
    final int flow;
    int mask;

    public Valve(Matcher m) {
        name = m.group(1);
        flow = Integer.parseInt(m.group(2));
        mask = 0;
    }
}

class OnePathSolver {
    private final List<Valve> valvesWithFlow;
    int calls = 0;
    Map<OnePathKey, OnePath> cache = new HashMap<>();
    int cacheHits = 0;

    public OnePathSolver(List<Valve> valvesWithFlow) {
        this.valvesWithFlow = valvesWithFlow;
    }

    public OnePath findPath(OnePath path) {
        calls += 1;
        if ((calls % 1000000) == 0) {
            Log.info("%d calls, %d cache hits...", calls, cacheHits);
        }
        var cacheKey = path.cacheKey();
        if (cache.containsKey(cacheKey)) {
            cacheHits += 1;
            var cached = cache.get(cacheKey);
            return path.merge(cached);
        }

        var currValve = path.getPos();

        var bestPath = path;
        for (var valve : valvesWithFlow) {
            // ignore start - will not increase flow
            if (valve.flow == 0) {
                continue;
            }
            if ((path.openValves & valve.mask) != 0) {
                continue;
            }
            var distance = currValve.tunnels[valve.id];
            var next = path.next(valve, distance);
            if (next.elapsed >= Day16.PART1_MINUTES) {
                continue;
            }
            var subBest = findPath(next);
            if (subBest.totalFlow > bestPath.totalFlow) {
                bestPath = subBest;
            }
        }

        cache.put(cacheKey, bestPath.diff(path));
        return bestPath;
    }
}

record OnePathKey(String name, int elapsed, int valves) {
}

class OnePath {
    List<Valve> visited;
    int openValves;
    int elapsed;
    int totalFlow;

    public OnePath(Valve start) {
        visited = new ArrayList<>(List.of(start));
    }

    public OnePath(List<Valve> visited, int openValves, int elapsed, int totalFlow) {
        this.visited = visited;
        this.openValves = openValves;
        this.elapsed = elapsed;
        this.totalFlow = totalFlow;
    }

    public OnePathKey cacheKey() {
        return new OnePathKey(getPos().name, elapsed, openValves);
    }

    public OnePath merge(OnePath other) {
        var visited = new ArrayList<>(this.visited);
        visited.addAll(other.visited);
        return new OnePath(visited,
                openValves,
                elapsed + other.elapsed,
                totalFlow + other.totalFlow);
    }

    public OnePath next(Valve valve, byte distance) {
        var visited = new ArrayList<>(this.visited);
        visited.add(valve);
        var elapsed = this.elapsed + distance + 1;
        var flow = (Day16.PART1_MINUTES - elapsed) * valve.flow;
        return new OnePath(
                visited,
                openValves | valve.mask,
                elapsed,
                totalFlow + flow);
    }

    public OnePath diff(OnePath start) {
        var visited = new ArrayList<>(this.visited.subList(start.visited.size(), this.visited.size()));
        return new OnePath(
                visited,
                openValves,
                this.elapsed - start.elapsed,
                this.totalFlow - start.totalFlow);
    }

    public Valve getPos() {
        return visited.getLast();
    }
}

class TwoPathSolver {
    private final List<Valve> valvesWithFlow;
    int calls = 0;
    Map<TwoPathKey, TwoPath> cache;
    int cacheHits = 0;

    public TwoPathSolver(List<Valve> valvesWithFlow) {
        this.valvesWithFlow = valvesWithFlow;
        cache = HashMap.newHashMap(35_000_000);
    }

    public TwoPath findPath(TwoPath path) {
        calls += 1;
        if ((calls % 1000000) == 0) {
            Log.info("%d calls, %d cache hits...", calls, cacheHits);
        }

        var cacheKey = path.cacheKey();
        var cached = cache.get(cacheKey);
        if (cached != null) {
            cacheHits += 1;
            return path.merge(cached);
        }

        var manPos = path.getHumanPos();
        var manValve = valvesWithFlow.get(manPos);
        var elePos = path.getElephantPos();
        var eleValve = valvesWithFlow.get(elePos);

        var bestPath = path;
        for (var valve : valvesWithFlow) {
            // ignore start - will not increase flow
            if (valve.flow == 0) {
                continue;
            }
            // ignore valves already open
            if ((path.openValves & valve.mask) != 0) {
                continue;
            }
            // try to move both human and elephant towards the next valve
            // move human
            var distance = manValve.tunnels[valve.id];
            var next = path.nextHuman(valve, distance);
            if (next.elapsed < Day16.PART2_MINUTES) {
                var subBest = findPath(next);
                if (subBest.totalFlow > bestPath.totalFlow) {
                    bestPath = subBest;
                }
            }

            // move elephant
            distance = eleValve.tunnels[valve.id];
            next = path.nextElephant(valve, distance);
            if (next.elapsed < Day16.PART2_MINUTES) {
                var subBest = findPath(next);
                if (subBest.totalFlow > bestPath.totalFlow) {
                    bestPath = subBest;
                }
            }
        }

        cache.put(cacheKey, bestPath.diff(path));
        return bestPath;

    }
}

record TwoPathKey(byte humanPos, byte humanElapsed, byte elePos, byte eleElapsed, int valves) {
}

class TwoPath {
    byte[] humanPath;
    byte humanPathPos;
    byte humanElapsed;
    byte[] elePath;
    byte elePathPos;
    byte eleElapsed;
    int openValves;
    byte elapsed;
    int totalFlow;

    public TwoPath(int size) {
        humanPath = new byte[size + 1];
        humanPath[0] = 0;
        elePath = new byte[size + 1];
        elePath[0] = 0;
    }

    private TwoPath(byte[] humanPath, byte humanPathPos, byte humElapsed, byte[] elePath, byte elePathPos, byte eleElapsed, int openValves, byte elapsed, int totalFlow) {
        this.humanPath = humanPath;
        this.humanPathPos = humanPathPos;
        this.humanElapsed = humElapsed;
        this.elePath = elePath;
        this.elePathPos = elePathPos;
        this.eleElapsed = eleElapsed;
        this.openValves = openValves;
        this.elapsed = elapsed;
        this.totalFlow = totalFlow;
    }

    public TwoPathKey cacheKey() {
        return new TwoPathKey(
                getHumanPos(),
                humanElapsed,
                getElephantPos(),
                eleElapsed,
                openValves
        );
    }

    public TwoPath merge(TwoPath other) {
        var humPath = new byte[this.humanPath.length];
        System.arraycopy(this.humanPath, 0, humPath, 0, this.humanPathPos + 1);
        System.arraycopy(other.humanPath, 0, humPath, this.humanPathPos + 1, other.humanPathPos);
        var elePath = new byte[this.elePath.length];
        System.arraycopy(this.elePath, 0, elePath, 0, this.elePathPos + 1);
        System.arraycopy(other.elePath, 0, elePath, this.elePathPos + 1, other.elePathPos);
        return new TwoPath(
                humPath,
                (byte) (this.humanPathPos + other.humanPathPos),
                (byte) (this.humanElapsed + other.humanElapsed),
                elePath,
                (byte) (this.elePathPos + other.elePathPos),
                (byte) (this.eleElapsed + other.eleElapsed),
                this.openValves,
                (byte) (this.elapsed + other.elapsed),
                this.totalFlow + other.totalFlow
        );
    }

    public TwoPath nextHuman(Valve valve, int distance) {
        var humPath = new byte[this.humanPath.length];
        System.arraycopy(this.humanPath, 0, humPath, 0, this.humanPathPos + 1);
        humPath[this.humanPathPos + 1] = valve.id;
        var elePath = new byte[this.elePath.length];
        System.arraycopy(this.elePath, 0, elePath, 0, this.elePathPos + 1);
        var elapsed = this.humanElapsed + distance + 1;
        var flow = (Day16.PART2_MINUTES - elapsed) * valve.flow;
        return new TwoPath(
                humPath,
                (byte) (humanPathPos + 1),
                (byte) elapsed,
                elePath,
                elePathPos,
                this.eleElapsed,
                openValves | valve.mask,
                (byte) Math.max(elapsed, eleElapsed),
                totalFlow + flow
        );
    }

    public TwoPath nextElephant(Valve valve, int distance) {
        var humPath = new byte[this.humanPath.length];
        System.arraycopy(this.humanPath, 0, humPath, 0, this.humanPathPos + 1);
        var elePath = new byte[this.elePath.length];
        System.arraycopy(this.elePath, 0, elePath, 0, this.elePathPos + 1);
        elePath[this.elePathPos + 1] = valve.id;
        var elapsed = this.eleElapsed + distance + 1;
        var flow = (Day16.PART2_MINUTES - elapsed) * valve.flow;
        return new TwoPath(
                humPath,
                humanPathPos,
                humanElapsed,
                elePath,
                (byte) (elePathPos + 1),
                (byte) elapsed,
                openValves | valve.mask,
                (byte) Math.max(elapsed, humanElapsed),
                totalFlow + flow
        );
    }

    public TwoPath diff(TwoPath start) {
        var humanPath = new byte[this.humanPath.length];
        System.arraycopy(this.humanPath, start.humanPathPos + 1, humanPath, 0, this.humanPathPos - start.humanPathPos);
        var elePath = new byte[this.humanPath.length];
        System.arraycopy(this.elePath, start.elePathPos + 1, elePath, 0, this.elePathPos - start.elePathPos);
        return new TwoPath(
                humanPath,
                (byte) (this.humanPathPos - start.humanPathPos),
                (byte) (humanElapsed - start.humanElapsed),
                elePath,
                (byte) (this.elePathPos - start.elePathPos),
                (byte) (eleElapsed - start.eleElapsed),
                openValves,
                (byte) (elapsed - start.elapsed),
                totalFlow - start.totalFlow
        );
    }

    public byte getHumanPos() {
        return humanPath[humanPathPos];
    }

    public byte getElephantPos() {
        return elePath[elePathPos];
    }
}
