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

    private final List<String> valves_with_flow = new ArrayList<>();
    private final List<Valve> valves = new ArrayList<>();
    private final Map<String, Map<String, Integer>> distances = new HashMap<>();

    @Override
    public void parse(String line) {
        if (line.isEmpty()) {
            return;
        }
        Matcher m = RE_VALVE.matcher(line);
        if (m.matches()) {
            var valve = new Valve(m);
            if (valve.flow > 0) {
                valves_with_flow.add(valve.name);
                valve.mask = 1 << valves_with_flow.size();
            }
            valves.add(valve);
        }
    }

    @Override
    public Results solve() {
        Log.debug("Found %d valves to open in %d minutes", valves.size(), PART1_MINUTES);
        Log.debug("Valves with flow: %d => %d possible paths",
                // TODO: is there a factorial method?
                valves_with_flow.size(), -valves_with_flow.size());
        calculateDistances();

        // part 1 - timed
        var t0 = System.currentTimeMillis();
        var one_path = new OnePathSolver();
        var best_path1 = one_path.find_path(new OnePath(START));
        var t1 = System.currentTimeMillis();
        Log.info("[1] Found max flow is %d: %s (%d cache hits) [%.3fsec]",
                best_path1.total_flow, best_path1.visited, one_path.cache_hits, (t1 - t0) / 1000.0);

        // part 2 - timed
        t0 = System.currentTimeMillis();
        var two_path = new TwoPathSolver();
        var best_path2 = two_path.find_path(new TwoPath(START));
        t1 = System.currentTimeMillis();
        Log.info("[2] Found max flow is %d: %s / %s (%d cache hits) [%.3fsec]",
                best_path2.total_flow, best_path2.human_path, best_path2.ele_path, two_path.cache_hits,
                (t1 - t0) / 1000.0);
        return new Results(String.valueOf(best_path1.total_flow), String.valueOf(best_path2.total_flow));
    }

    private record DistanceWrapper(String cave, int distance) {
    }

    private void calculateDistances() {
        // add (temporarily) the start into the valves that need to be evaluated
        valves_with_flow.addFirst(START);
        for (String name : valves_with_flow) {
            // oh, java, when will you add a simple mutable map/list/set initializer?
            var currentDistances = new HashMap<>(Map.of(name, 0));
            var visited = new HashSet<>(Set.of(name));
            var queue = new LinkedList<>(List.of(new DistanceWrapper(name, 0)));
            while (!queue.isEmpty()) {
                var wrapper = queue.removeFirst();
                var valve = get_valve(wrapper.cave);
                for (String next : valve.connections) {
                    if (visited.contains(next)) {
                        continue;
                    }
                    visited.add(next);
                    if (valves_with_flow.stream().anyMatch(v -> v.equals(next))) {
                        currentDistances.put(next, wrapper.distance + 1);
                    }
                    queue.add(new DistanceWrapper(next, wrapper.distance + 1));
                }
            }
            currentDistances.remove(name);
            distances.put(name, currentDistances);
        }
        valves_with_flow.removeFirst();
    }

    private Valve get_valve(String cave) {
        return valves.stream().filter(v -> Objects.equals(cave, v.name)).findFirst().orElseThrow();
    }

    private static class Valve {
        private final List<String> connections;
        private final String name;
        private final int flow;
        private int mask;

        public Valve(Matcher m) {
            connections = Arrays.stream(m.group(3).split(", ")).toList();
            name = m.group(1);
            flow = Integer.parseInt(m.group(2));
            mask = 0;
        }
    }

    private class OnePathSolver {
        int calls = 0;
        Map<OnePathKey, OnePath> cache = new HashMap<>();
        int cache_hits = 0;

        public OnePath find_path(OnePath path) {
            calls += 1;
            if ((calls % 1000000) == 0) {
                Log.info("%d calls, %d cache hits...", calls, cache_hits);
            }
            var cave = path.visited.getLast();
            var cache_key = path.cache_key();
            if (cache.containsKey(cache_key)) {
                cache_hits += 1;
                var cached = cache.get(cache_key);
                return path.merge(cached);
            }

            var best_path = path;
            for (String name : valves_with_flow) {
                var valve = get_valve(name);
                if ((path.open_valves & valve.mask) != 0) {
                    continue;
                }
                var distance = distances.get(cave).get(name);
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

    private record OnePathKey(String name, int elapsed, int valves) {
    }

    private static class OnePath {
        List<String> visited;
        int open_valves;
        int elapsed;
        int total_flow;

        public OnePath(String start) {
            visited = new ArrayList<>(List.of(start));
        }

        public OnePath(ArrayList<String> visited, int openValves, int elapsed, int total_flow) {
            this.visited = visited;
            this.open_valves = openValves;
            this.elapsed = elapsed;
            this.total_flow = total_flow;
        }

        public OnePathKey cache_key() {
            return new OnePathKey(visited.getLast(), elapsed, open_valves);
        }

        public OnePath merge(OnePath other) {
            var visited = new ArrayList<>(this.visited);
            visited.addAll(other.visited);
            return new OnePath(visited,
                    open_valves,
                    elapsed + other.elapsed,
                    total_flow + other.total_flow);
        }

        public OnePath next(Valve valve, Integer distance) {
            var visited = new ArrayList<>(this.visited);
            visited.add(valve.name);
            var elapsed = this.elapsed + distance + 1;
            var flow = (PART1_MINUTES - elapsed) * valve.flow;
            return new OnePath(
                    visited,
                    open_valves | valve.mask,
                    elapsed,
                    total_flow + flow);
        }

        public OnePath diff(OnePath start) {
            var visited = new ArrayList<>(this.visited.subList(start.visited.size(), this.visited.size()));
            return new OnePath(
                    visited,
                    open_valves,
                    this.elapsed - start.elapsed,
                    this.total_flow - start.total_flow);
        }
    }

    private class TwoPathSolver {
        int calls = 0;
        Map<TwoPathKey, TwoPath> cache = new HashMap<>();
        int cache_hits = 0;

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
            for (String name : valves_with_flow) {
                var valve = get_valve(name);
                // try to move both human and elephant towards the next valve
                if ((path.open_valves & valve.mask) != 0) {
                    continue;
                }
                // move human
                var distance = distances.get(man_pos).get(name);
                var next = path.next_human(valve, distance);
                if (next.elapsed < PART2_MINUTES) {
                    var sub_best = find_path(next);
                    if (sub_best.total_flow > best_path.total_flow) {
                        best_path = sub_best;
                    }
                }

                // move elephant
                distance = distances.get(ele_pos).get(name);
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