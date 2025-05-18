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
        var one_path = new OnePathSolver(valvesWithFlow);
        var best_path1 = one_path.find_path(new OnePath(valvesWithFlow.size()));
        var t1 = System.currentTimeMillis();
        var path = valvePath(valvesWithFlow, best_path1.visited, best_path1.visited_pos);
        Log.info("[1] Found max flow is %d: %s (%d cache hits, %d calls, %d cache size) [%.3fsec]",
                best_path1.total_flow, path, one_path.cache_hits, one_path.calls,
                one_path.cache.size(), (t1 - t0) / 1000.0);

        // part 2 - timed
        t0 = System.currentTimeMillis();
        var two_path = new TwoPathSolver(valvesWithFlow);
        var best_path2 = two_path.find_path(new TwoPath(valvesWithFlow.size()));
        t1 = System.currentTimeMillis();
        var human_path = valvePath(valvesWithFlow, best_path2.human_path,
                best_path2.human_path_pos);
        var ele_path = valvePath(valvesWithFlow, best_path2.ele_path, best_path2.ele_path_pos);
        Log.info("[2] Found max flow is %d: %s / %s (%d cache hits, %d calls, %d cache size) [%.3fsec]",
                best_path2.total_flow, human_path, ele_path, two_path.cache_hits,
                two_path.calls, two_path.cache.size(), (t1 - t0) / 1000.0);
        return new Results(String.valueOf(best_path1.total_flow), String.valueOf(best_path2.total_flow));
    }

    private List<String> valvePath(List<Valve> valvesWithFlow, byte[] visited, byte length) {
        var path = new ArrayList<String>();
        for (int i = 0; i <= length; i++) {
            path.add(valvesWithFlow.get(visited[i]).name);
        }
        return path;
    }

    private List<Valve> precacheValues() {
        List<Valve> valves_with_flow = new ArrayList<>();
        // add to the valves with flow and set the valve mask
        for (Valve valve : valves.values()) {
            if (valve.flow > 0) {
                valve.mask = 1 << valves_with_flow.size();
                valves_with_flow.add(valve);
                // id = 0 will be the START valve
                valve.id = (byte) valves_with_flow.size();
            } else if (valve.name.equals(START)) {
                valve.id = 0;
            }
        }
        // always add the START valve as first
        valves_with_flow.addFirst(valves.get(START));

        calculateDistances(valves_with_flow);

        return valves_with_flow;
    }

    private record DistanceWrapper(Valve cave, int distance) {
    }

    private void calculateDistances(List<Valve> valvesWithFlow) {
        // create a map associating valves and their connections;
        var tunnels = makeTunnelMap();

        for (Valve curr : valvesWithFlow) {
            // oh, java, when will you add a simple mutable map/list/set initializer?
            curr.tunnels = new byte[valvesWithFlow.size() + 1];
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
    byte id = -1;
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
    int cache_hits = 0;

    public OnePathSolver(List<Valve> valvesWithFlow) {
        this.valvesWithFlow = valvesWithFlow;
    }

    public OnePath find_path(OnePath path) {
        calls += 1;
        if ((calls % 1000000) == 0) {
            Log.info("%d calls, %d cache hits...", calls, cache_hits);
        }
        var cache_key = path.cache_key();
        if (cache.containsKey(cache_key)) {
            cache_hits += 1;
            var cached = cache.get(cache_key);
            return path.merge(cached);
        }

        var cave = path.getLast();
        var currValve = valvesWithFlow.get(cave);

        var best_path = path;
        for (var valve : valvesWithFlow) {
            // ignore start - will not increase flow
            if (valve.flow == 0) {
                continue;
            }
            if ((path.open_valves & valve.mask) != 0) {
                continue;
            }
            var distance = currValve.tunnels[valve.id];
            var next = path.next(valve, distance);
            if (next.elapsed >= Day16.PART1_MINUTES) {
                continue;
            }
            var sub_best = find_path(next);
            if (sub_best.total_flow > best_path.total_flow) {
                best_path = sub_best;
            }
        }

        cache.put(cache_key, best_path.diff(path));
        return best_path;
    }
}

record OnePathKey(byte pos, byte elapsed, int valves) {
}

class OnePath {
    byte[] visited;
    byte visited_pos;
    int open_valves;
    byte elapsed;
    int total_flow;

    public OnePath(int size) {
        visited = new byte[size + 1];
        visited[0] = 0;
    }

    private OnePath(byte[] visited, byte visited_pos, int openValves, byte elapsed, int total_flow) {
        this.visited = visited;
        this.visited_pos = visited_pos;
        this.open_valves = openValves;
        this.elapsed = elapsed;
        this.total_flow = total_flow;
    }

    public OnePathKey cache_key() {
        return new OnePathKey(getLast(), elapsed, open_valves);
    }

    public OnePath merge(OnePath other) {
        var visited = new byte[this.visited.length];
        System.arraycopy(this.visited, 0, visited, 0, this.visited_pos + 1);
        System.arraycopy(other.visited, 0, visited, this.visited_pos + 1, other.visited_pos);
        return new OnePath(visited,
                (byte) (this.visited_pos + other.visited_pos),
                open_valves,
                (byte) (elapsed + other.elapsed),
                total_flow + other.total_flow);
    }

    public OnePath next(Valve valve, byte distance) {
        var visited = Arrays.copyOf(this.visited, this.visited.length);
        visited[this.visited_pos + 1] = valve.id;
        var elapsed = this.elapsed + distance + 1;
        var flow = (Day16.PART1_MINUTES - elapsed) * valve.flow;
        return new OnePath(
                visited,
                (byte) (visited_pos + 1),
                open_valves | valve.mask,
                (byte) elapsed,
                total_flow + flow);
    }

    public OnePath diff(OnePath start) {
        var visited = new byte[this.visited.length];
        System.arraycopy(this.visited, start.visited_pos + 1, visited, 0, this.visited_pos - start.visited_pos);
        return new OnePath(
                visited,
                (byte) (this.visited_pos - start.visited_pos),
                open_valves,
                (byte) (this.elapsed - start.elapsed),
                this.total_flow - start.total_flow);
    }

    public byte getLast() {
        return visited[visited_pos];
    }
}

class TwoPathSolver {
    private final List<Valve> valvesWithFlow;
    int calls = 0;
    Map<TwoPathKey, TwoPath> cache;
    int cache_hits = 0;

    public TwoPathSolver(List<Valve> valvesWithFlow) {
        this.valvesWithFlow = valvesWithFlow;
        cache = HashMap.newHashMap(35_000_000);
    }

    public TwoPath find_path(TwoPath path) {
        calls += 1;
        if ((calls % 1000000) == 0) {
            Log.info("%d calls, %d cache hits...", calls, cache_hits);
        }

        var cache_key = path.cache_key();
        var cached = cache.get(cache_key);
        if (cached != null) {
            cache_hits += 1;
            return path.merge(cached);
        }

        var man_pos = path.getHumanPos();
        var manValve = valvesWithFlow.get(man_pos);
        var ele_pos = path.getElephantPos();
        var eleValve = valvesWithFlow.get(ele_pos);
        var best_path = path;
        for (var valve : valvesWithFlow) {
            // ignore start - will not increase flow
            if (valve.flow == 0) {
                continue;
            }
            // try to move both human and elephant towards the next valve
            if ((path.open_valves & valve.mask) != 0) {
                continue;
            }
            // move human
            var distance = manValve.tunnels[valve.id];
            var next = path.next_human(valve, distance);
            if (next.elapsed < Day16.PART2_MINUTES) {
                var sub_best = find_path(next);
                if (sub_best.total_flow > best_path.total_flow) {
                    best_path = sub_best;
                }
            }

            // move elephant
            distance = eleValve.tunnels[valve.id];
            next = path.next_elephant(valve, distance);
            if (next.elapsed < Day16.PART2_MINUTES) {
                var sub_best = find_path(next);
                if (sub_best.total_flow > best_path.total_flow) {
                    best_path = sub_best;
                }
            }
        }

        cache.put(cache_key, best_path.diff(path));
        return best_path;

    }
}

record TwoPathKey(byte human_pos, byte human_elapsed, byte ele_pos, byte ele_elapsed, int valves) {
}

class TwoPath {
    byte[] human_path;
    byte human_path_pos;
    byte human_elapsed;
    byte[] ele_path;
    byte ele_path_pos;
    byte ele_elapsed;
    int open_valves;
    byte elapsed;
    int total_flow;

    public TwoPath(int size) {
        human_path = new byte[size + 1];
        human_path[0] = 0;
        ele_path = new byte[size + 1];
        ele_path[0] = 0;
    }

    private TwoPath(byte[] humanPath, byte humanPathPos, byte humElapsed, byte[] elePath, byte elePathPos, byte eleElapsed, int openValves, byte elapsed, int totalFlow) {
        this.human_path = humanPath;
        this.human_path_pos = humanPathPos;
        this.human_elapsed = humElapsed;
        this.ele_path = elePath;
        this.ele_path_pos = elePathPos;
        this.ele_elapsed = eleElapsed;
        this.open_valves = openValves;
        this.elapsed = elapsed;
        this.total_flow = totalFlow;
    }

    public TwoPathKey cache_key() {
        return new TwoPathKey(
                getHumanPos(),
                human_elapsed,
                getElephantPos(),
                ele_elapsed,
                open_valves
        );
    }

    public TwoPath merge(TwoPath other) {
        var hum_path = new byte[this.human_path.length];
        System.arraycopy(this.human_path, 0, hum_path, 0, this.human_path_pos + 1);
        System.arraycopy(other.human_path, 0, hum_path, this.human_path_pos + 1, other.human_path_pos);
        var ele_path = new byte[this.ele_path.length];
        System.arraycopy(this.ele_path, 0, ele_path, 0, this.ele_path_pos + 1);
        System.arraycopy(other.ele_path, 0, ele_path, this.ele_path_pos + 1, other.ele_path_pos);
        return new TwoPath(
                hum_path,
                (byte) (this.human_path_pos + other.human_path_pos),
                (byte) (this.human_elapsed + other.human_elapsed),
                ele_path,
                (byte) (this.ele_path_pos + other.ele_path_pos),
                (byte) (this.ele_elapsed + other.ele_elapsed),
                this.open_valves,
                (byte) (this.elapsed + other.elapsed),
                this.total_flow + other.total_flow
        );
    }

    public TwoPath next_human(Valve valve, int distance) {
        var hum_path = new byte[this.human_path.length];
        System.arraycopy(this.human_path, 0, hum_path, 0, this.human_path_pos + 1);
        hum_path[this.human_path_pos + 1] = valve.id;
        var ele_path = new byte[this.ele_path.length];
        System.arraycopy(this.ele_path, 0, ele_path, 0, this.ele_path_pos + 1);
        var elapsed = this.human_elapsed + distance + 1;
        var flow = (Day16.PART2_MINUTES - elapsed) * valve.flow;
        return new TwoPath(
                hum_path,
                (byte) (human_path_pos + 1),
                (byte) elapsed,
                ele_path,
                ele_path_pos,
                this.ele_elapsed,
                open_valves | valve.mask,
                (byte) Math.max(elapsed, ele_elapsed),
                total_flow + flow
        );
    }

    public TwoPath next_elephant(Valve valve, int distance) {
        var hum_path = new byte[this.human_path.length];
        System.arraycopy(this.human_path, 0, hum_path, 0, this.human_path_pos + 1);
        var ele_path = new byte[this.ele_path.length];
        System.arraycopy(this.ele_path, 0, ele_path, 0, this.ele_path_pos + 1);
        ele_path[this.ele_path_pos + 1] = valve.id;
        var elapsed = this.ele_elapsed + distance + 1;
        var flow = (Day16.PART2_MINUTES - elapsed) * valve.flow;
        return new TwoPath(
                hum_path,
                human_path_pos,
                human_elapsed,
                ele_path,
                (byte) (ele_path_pos + 1),
                (byte) elapsed,
                open_valves | valve.mask,
                (byte) Math.max(elapsed, human_elapsed),
                total_flow + flow
        );
    }

    public TwoPath diff(TwoPath start) {
        var human_path = new byte[this.human_path.length];
        System.arraycopy(this.human_path, start.human_path_pos + 1, human_path, 0, this.human_path_pos - start.human_path_pos);
        var ele_path = new byte[this.human_path.length];
        System.arraycopy(this.ele_path, start.ele_path_pos + 1, ele_path, 0, this.ele_path_pos - start.ele_path_pos);
        return new TwoPath(
                human_path,
                (byte) (this.human_path_pos - start.human_path_pos),
                (byte) (human_elapsed - start.human_elapsed),
                ele_path,
                (byte) (this.ele_path_pos - start.ele_path_pos),
                (byte) (ele_elapsed - start.ele_elapsed),
                open_valves,
                (byte) (elapsed - start.elapsed),
                total_flow - start.total_flow
        );
    }

    public byte getHumanPos() {
        return human_path[human_path_pos];
    }

    public byte getElephantPos() {
        return ele_path[ele_path_pos];
    }
}
