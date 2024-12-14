package aoc.year2022;

import aoc.api.Results;
import aoc.api.Solver;
import aoc.util.Log;

import java.util.*;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day16 implements Solver {

    private static final Pattern RE_VALVE = Pattern.compile("Valve (\\S+) has flow rate=(\\d+); tunnels? leads? to valves? (.*)");
    private static final int PART1_MINUTES = 30;
    private static final int PART2_MINUTES = 26;
    private static final String START = "AA";

    private final List<Valve> valves = new ArrayList<>();

    @Override
    public void parse(String line) {
        if (line.isEmpty()) {
            return;
        }
        Matcher m = RE_VALVE.matcher(line);
        if (m.matches()) {
            var valve = new Valve((byte) valves.size(), m);
            valves.add(valve);
        }
    }

    @Override
    public Results solve() {
        Log.debug("Found %d valves to open in %d minutes", valves.size(), PART1_MINUTES);

        var valvesWithFlow = precacheValues();

        // TODO: is there a factorial method?
        Log.debug("Valves with flow: %d", valvesWithFlow.length);

        byte start = valves.stream().filter(v -> v.name.equals(START)).map(v -> v.id).findFirst().orElseThrow();
        var distances = calculateDistances(start, valvesWithFlow);

        // part 1 - timed
        var t0 = System.currentTimeMillis();
        var one_path = new OnePathSolver(valvesWithFlow, distances);
        var best_path1 = one_path.find_path(new OnePath(start, valvesWithFlow.length));
        var t1 = System.currentTimeMillis();
        var path = valvePath(best_path1.visited);
        Log.info("[1] Found max flow is %d: %s (%d cache hits, %d calls, %d cache size) [%.3fsec]",
                best_path1.total_flow, path, one_path.cache_hits, one_path.calls,
                one_path.cache.size(), (t1 - t0) / 1000.0);

        // part 2 - timed
//        t0 = System.currentTimeMillis();
//        var two_path = new TwoPathSolver(valvesWithFlow, distances);
//        var best_path2 = two_path.find_path(new TwoPath(START));
//        t1 = System.currentTimeMillis();
//        Log.info("[2] Found max flow is %d: %s / %s (%d cache hits, %d calls, %d cache size) [%.3fsec]",
//                best_path2.total_flow, best_path2.human_path, best_path2.ele_path, two_path.cache_hits,
//                two_path.calls, two_path.cache.size(), (t1 - t0) / 1000.0);
        return new Results(String.valueOf(best_path1.total_flow), String.valueOf(best_path1.total_flow));
    }

    private List<String> valvePath(byte[] visited) {
        var path = new ArrayList<String>();
        for (byte id : visited) {
            path.add(valves.get(id).name);
        }
        return path;
    }

    private byte[] precacheValues() {
        ArrayList<Byte> valves_with_flow = new ArrayList<>();
        HashMap<String, Byte> nameToId = new HashMap<>();
        // add to the valves with flow and set the valve mask
        for (Valve valve : valves) {
            if (valve.flow > 0) {
                valves_with_flow.add(valve.id);
                valve.mask = 1 << valves_with_flow.size();
            }
            nameToId.put(valve.name, valve.id);
        }

        // create the tunnels by reverse-lookup of name to id
        for (Valve valve : valves) {
            byte[] tunnels = new byte[valve.connections.size()];
            for (int i = 0; i < tunnels.length; i++) {
                tunnels[i] = nameToId.get(valve.connections.get(i));
            }
            valve.tunnels = tunnels;
        }

        // create an array of bytes off the ArrayList
        byte[] valves = new byte[valves_with_flow.size()];
        for (int i = 0; i < valves_with_flow.size(); i++) {
            valves[i] = valves_with_flow.get(i);
        }
        return valves;
    }

    private record DistanceWrapper(byte cave, int distance) {
    }

    private Map<Byte, Map<Byte, Byte>> calculateDistances(byte start, byte[] valvesWithFlow) {
        // add (temporarily) the start into the valves that need to be evaluated
        Map<Byte, Map<Byte, Byte>> distances = new HashMap<>();
        var allValves = new ArrayList<Byte>();
        allValves.add(start);
        for (byte valve : valvesWithFlow) {
            allValves.add(valve);
        }
        for (byte id : allValves) {
            // oh, java, when will you add a simple mutable map/list/set initializer?
            var currentDistances = new HashMap<>(Map.of(id, (byte)0));
            var visited = new HashSet<>(Set.of(id));
            var queue = new LinkedList<>(List.of(new DistanceWrapper(id, 0)));
            while (!queue.isEmpty()) {
                var wrapper = queue.removeFirst();
                var valve = valves.get(wrapper.cave);
                for (Byte next : valve.tunnels) {
                    if (visited.contains(next)) {
                        continue;
                    }
                    visited.add(next);
                    if (allValves.stream().anyMatch(v -> v.equals(next))) {
                        currentDistances.put(next, (byte) (wrapper.distance + 1));
                    }
                    queue.add(new DistanceWrapper(next, wrapper.distance + 1));
                }
            }
            currentDistances.remove(id);
            distances.put(id, currentDistances);
        }
        return distances;
    }

    private Valve get_valve(String cave) {
        return valves.stream().filter(v -> Objects.equals(cave, v.name)).findFirst().orElseThrow();
    }

    private static class Valve {
        private final byte id;
        private final List<String> connections;
        private byte[] tunnels;
        private final String name;
        private final int flow;
        private int mask;

        public Valve(byte id, Matcher m) {
            this.id = id;
            connections = Arrays.stream(m.group(3).split(", ")).toList();
            name = m.group(1);
            flow = Integer.parseInt(m.group(2));
            mask = 0;
        }
    }

    private class OnePathSolver {
        private final byte[] valvesWithFlow;
        private final Map<Byte, Map<Byte, Byte>> distances;
        int calls = 0;
        Map<OnePathKey, OnePath> cache = new HashMap<>();
        int cache_hits = 0;

        public OnePathSolver(byte[] valvesWithFlow, Map<Byte, Map<Byte, Byte>> distances) {
            this.valvesWithFlow = valvesWithFlow;
            this.distances = distances;
        }

        public OnePath find_path(OnePath path) {
            calls += 1;
            if ((calls % 1000000) == 0) {
                Log.info("%d calls, %d cache hits...", calls, cache_hits);
            }
            var cave = path.getLast();
            var cache_key = path.cache_key();
            if (cache.containsKey(cache_key)) {
                cache_hits += 1;
                var cached = cache.get(cache_key);
                return path.merge(cached);
            }

            var best_path = path;
            for (byte id : valvesWithFlow) {
                var valve = valves.get(id);
                if ((path.open_valves & valve.mask) != 0) {
                    continue;
                }
                var distance = distances.get(cave).get(id);
                var next = path.next(valve, distance);
                if (next.elapsed >= PART1_MINUTES) {
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

    private record OnePathKey(byte pos, byte elapsed, int valves) {
    }

    private static class OnePath {
        byte[] visited;
        byte visited_pos;
        int open_valves;
        byte elapsed;
        int total_flow;

        public OnePath(byte start, int size) {
            visited = new byte[size + 1];
            visited[0] = start;
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
            var flow = (PART1_MINUTES - elapsed) * valve.flow;
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

    private class TwoPathSolver {
        private final byte[] valvesWithFlow;
        private final Map<Byte, Map<Byte, Byte>> distances;
        int calls = 0;
        Map<TwoPathKey, TwoPath> cache = new HashMap<>();
        int cache_hits = 0;

        public TwoPathSolver(byte[] valvesWithFlow, Map<Byte, Map<Byte, Byte>> distances) {
            this.valvesWithFlow = valvesWithFlow;
            this.distances = distances;
        }

        public TwoPath find_path(TwoPath path) {
            calls += 1;
            if ((calls % 1000000) == 0) {
                Log.info("%d calls, %d cache hits...", calls, cache_hits);
            }

            var man_pos = path.human_path.getLast();
            var ele_pos = path.ele_path.getLast();
            var cache_key = path.cache_key();

            if (cache.containsKey(cache_key)) {
                cache_hits += 1;
                var cached = cache.get(cache_key);
                return path.merge(cached);
            }

            var best_path = path;
            for (byte id : valvesWithFlow) {
                var valve = valves.get(id);
                // try to move both human and elephant towards the next valve
                if ((path.open_valves & valve.mask) != 0) {
                    continue;
                }
                // move human
                var distance = distances.get(man_pos).get(id);
                var next = path.next_human(valve, distance);
                if (next.elapsed < PART2_MINUTES) {
                    var sub_best = find_path(next);
                    if (sub_best.total_flow > best_path.total_flow) {
                        best_path = sub_best;
                    }
                }

                // move elephant
                distance = distances.get(ele_pos).get(id);
                next = path.next_elephant(valve, distance);
                if (next.elapsed < PART2_MINUTES) {
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

    private record TwoPathKey(String human_pos, int human_elapsed, String ele_pos, int ele_elapsed, int valves) {
    }

    private static class TwoPath {
        List<String> human_path;
        int human_elapsed;
        List<String> ele_path;
        int ele_elapsed;
        int open_valves;
        int elapsed;
        int total_flow;

        public TwoPath(List<String> humanPath, int humElapsed, List<String> elePath, int eleElapsed, int openValves, int elapsed, int totalFlow) {
            this.human_path = humanPath;
            this.human_elapsed = humElapsed;
            this.ele_path = elePath;
            this.ele_elapsed = eleElapsed;
            this.open_valves = openValves;
            this.elapsed = elapsed;
            this.total_flow = totalFlow;
        }

        public TwoPath(String start) {
            human_path = new ArrayList<>(List.of(start));
            ele_path = new ArrayList<>(List.of(start));
        }

        public TwoPathKey cache_key() {
            return new TwoPathKey(
                    human_path.getLast(),
                    human_elapsed,
                    ele_path.getLast(),
                    ele_elapsed,
                    open_valves
            );
        }

        public TwoPath merge(TwoPath other) {
            var human_path = new ArrayList<>(this.human_path);
            human_path.addAll(other.human_path);
            var ele_path = new ArrayList<>(this.ele_path);
            ele_path.addAll(other.ele_path);
            return new TwoPath(
                    human_path,
                    this.human_elapsed + other.human_elapsed,
                    ele_path,
                    this.ele_elapsed + other.ele_elapsed,
                    this.open_valves,
                    this.elapsed + other.elapsed,
                    this.total_flow + other.total_flow
            );
        }

        public TwoPath next_human(Valve valve, int distance) {
            var human_path = new ArrayList<>(this.human_path);
            human_path.add(valve.name);
            var ele_path = new ArrayList<>(this.ele_path);
            var elapsed = this.human_elapsed + distance + 1;
            var flow = (PART2_MINUTES - elapsed) * valve.flow;
            return new TwoPath(
                    human_path,
                    elapsed,
                    ele_path,
                    this.ele_elapsed,
                    open_valves | valve.mask,
                    Math.max(elapsed, ele_elapsed),
                    total_flow + flow
            );
        }

        public TwoPath next_elephant(Valve valve, int distance) {
            var human_path = new ArrayList<>(this.human_path);
            var ele_path = new ArrayList<>(this.ele_path);
            ele_path.add(valve.name);
            var elapsed = this.ele_elapsed + distance + 1;
            var flow = (PART2_MINUTES - elapsed) * valve.flow;
            return new TwoPath(
                    human_path,
                    human_elapsed,
                    ele_path,
                    elapsed,
                    open_valves | valve.mask,
                    Math.max(elapsed, human_elapsed),
                    total_flow + flow
            );
        }

        public TwoPath diff(TwoPath start) {
            var human_path = new ArrayList<>(this.human_path.subList(start.human_path.size(), this.human_path.size()));
            var ele_path = new ArrayList<>(this.ele_path.subList(start.ele_path.size(), this.ele_path.size()));
            return new TwoPath(
                    human_path,
                    human_elapsed - start.human_elapsed,
                    ele_path,
                    ele_elapsed - start.ele_elapsed,
                    open_valves,
                    elapsed - start.elapsed,
                    total_flow - start.total_flow
            );
        }
    }
}