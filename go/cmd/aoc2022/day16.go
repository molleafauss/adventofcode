package main

import (
	"aoc/aoc"
	"container/list"
	"regexp"
	"slices"
	"strconv"
	"strings"
	"time"
)

type day16 struct {
	valves         []Valve
	valvesWithFlow []string
	distances      map[string]map[string]int
}

func Day16() *day16 {
	return &day16{
		valves:         []Valve{},
		valvesWithFlow: []string{},
		distances:      map[string]map[string]int{},
	}
}

var RE_VALVE = regexp.MustCompile("Valve (\\S+) has flow rate=(\\d+); tunnels? leads? to valves? (.*)")

const PART1_MINUTES = 30
const PART2_MINUTES = 26
const START = "AA"

func (solver *day16) Parse(line string) {
	if line == "" {
		return
	}
	matches := RE_VALVE.FindStringSubmatch(line)
	if matches == nil {
		return
	}
	flow, ok := strconv.Atoi(matches[2])
	if ok != nil {
		panic("Error converting flow rate: " + line)
	}
	valve := Valve{
		name:        matches[1],
		flow:        flow,
		mask:        0,
		connections: strings.Split(matches[3], ", "),
	}
	if valve.flow > 0 {
		solver.valvesWithFlow = append(solver.valvesWithFlow, valve.name)
		valve.mask = 1 << len(solver.valvesWithFlow)
	}
	solver.valves = append(solver.valves, valve)
}

func (solver *day16) Solve() (*string, *string) {
	aoc.Info("Valves with flow: %s", solver.valvesWithFlow)
	solver.calculateDistances()

	var t0 = time.Now()
	var one_path = OnePathSolver{
		cache: make(map[OnePathKey]OnePath),
	}
	best_path1 := one_path.find_path(solver, InitOnePath(START))
	var delta = time.Since(t0)
	aoc.Info("[1] Found max flow is %d: %s (%d cache hits / %d calls) [%.3fsec]",
		best_path1.total_flow, best_path1.visited, one_path.cache_hits, one_path.calls, delta.Seconds())

	t0 = time.Now()
	var two_path = TwoPathSolver{
		cache: make(map[TwoPathKey]TwoPath),
	}
	var best_path2 = two_path.find_path(solver, InitTwoPath(START))
	delta = time.Since(t0)
	aoc.Info("[2] Found max flow is %d: %s / %s (%d cache hits / %d calls) [%.3fsec]",
		best_path2.total_flow, best_path2.human_path, best_path2.ele_path, two_path.cache_hits, two_path.calls,
		delta.Seconds())

	part1 := strconv.Itoa(best_path1.total_flow)
	part2 := strconv.Itoa(best_path2.total_flow)
	return &part1, &part2
}

func (solver *day16) calculateDistances() {
	type distanceWrapper struct {
		cave     string
		distance int
	}

	allValves := []string{START}
	allValves = append(allValves, solver.valvesWithFlow...)
	for _, name := range allValves {
		var currentDistances = map[string]int{name: 0}
		var visited = map[string]bool{name: true}
		var queue = list.New()
		queue.PushBack(&distanceWrapper{name, 0})
		for queue.Len() > 0 {
			var wrapper = queue.Remove(queue.Front()).(*distanceWrapper)
			var valve = solver.get_valve(wrapper.cave)
			for _, next := range valve.connections {
				if _, ok := visited[next]; ok {
					continue
				}
				visited[next] = true
				if i := slices.IndexFunc(allValves, func(v string) bool { return v == next }); i != -1 {
					currentDistances[next] = wrapper.distance + 1
				}
				queue.PushBack(&distanceWrapper{next, wrapper.distance + 1})
			}
		}
		delete(currentDistances, name)
		solver.distances[name] = currentDistances
	}
}

func (solver *day16) get_valve(cave string) *Valve {
	i := slices.IndexFunc(solver.valves, func(v Valve) bool { return v.name == cave })
	if i == -1 {
		panic("Error finding valve: " + cave)
	}
	return &solver.valves[i]
}

type Valve struct {
	name        string
	flow        int
	mask        int
	connections []string
}

type OnePathSolver struct {
	cache_hits int
	calls      int
	cache      map[OnePathKey]OnePath
}

func (s *OnePathSolver) find_path(p *day16, path OnePath) OnePath {
	s.calls += 1
	if (s.calls % 1000000) == 0 {
		aoc.Info("%d calls, %d cache hits...", s.calls, s.cache_hits)
	}
	var cave = path.visited[len(path.visited)-1]
	var cache_key = path.cache_key()
	if cached, ok := s.cache[cache_key]; ok {
		s.cache_hits += 1
		return path.merge(cached)
	}

	var best_path = path
	for _, name := range p.valvesWithFlow {
		var valve = p.get_valve(name)
		if (path.open_valves & valve.mask) != 0 {
			continue
		}
		var distance = p.distances[cave][name]
		var next = path.next(valve, distance)
		if next.elapsed >= PART1_MINUTES {
			continue
		}
		var sub_best = s.find_path(p, next)
		if sub_best.total_flow > best_path.total_flow {
			best_path = sub_best
		}
	}

	s.cache[cache_key] = best_path.diff(path)
	return best_path
}

type OnePathKey struct {
	name    string
	elapsed int
	valves  int
}

type OnePath struct {
	visited     []string
	open_valves int
	elapsed     int
	total_flow  int
}

func InitOnePath(start string) OnePath {
	return OnePath{
		visited: []string{start},
	}
}

func (p *OnePath) cache_key() OnePathKey {
	return OnePathKey{p.visited[len(p.visited)-1], p.elapsed, p.open_valves}
}

func (p *OnePath) merge(other OnePath) OnePath {
	var visited = slices.Clone(p.visited)
	visited = append(visited, other.visited...)
	return OnePath{
		visited,
		p.open_valves,
		p.elapsed + other.elapsed,
		p.total_flow + other.total_flow,
	}
}

func (p *OnePath) next(valve *Valve, distance int) OnePath {
	var visited = slices.Clone(p.visited)
	visited = append(visited, valve.name)
	var elapsed = p.elapsed + distance + 1
	var flow = (PART1_MINUTES - elapsed) * valve.flow
	return OnePath{
		visited,
		p.open_valves | valve.mask,
		elapsed,
		p.total_flow + flow}
}

func (p *OnePath) diff(path OnePath) OnePath {
	var visited = p.visited[len(path.visited):len(p.visited)]
	return OnePath{
		visited,
		p.open_valves,
		p.elapsed - path.elapsed,
		p.total_flow - path.total_flow,
	}
}

type TwoPathSolver struct {
	cache_hits int
	calls      int
	cache      map[TwoPathKey]TwoPath
}

func (s *TwoPathSolver) find_path(solver *day16, path TwoPath) TwoPath {
	s.calls += 1
	if (s.calls % 1000000) == 0 {
		aoc.Info("%d calls, %d cache hits...", s.calls, s.cache_hits)
	}

	var man_pos = path.human_path[len(path.human_path)-1]
	var ele_pos = path.ele_path[len(path.ele_path)-1]
	var cache_key = path.cache_key()

	if cached, ok := s.cache[cache_key]; ok {
		s.cache_hits += 1
		return path.merge(cached)
	}

	var best_path = path
	for _, name := range solver.valvesWithFlow {
		var valve = solver.get_valve(name)
		// try to move both human and elephant towards the next valve
		if (path.open_valves & valve.mask) != 0 {
			continue
		}
		// move human
		var distance = solver.distances[man_pos][name]
		var next = path.next_human(valve, distance)
		if next.elapsed < PART2_MINUTES {
			var sub_best = s.find_path(solver, next)
			if sub_best.total_flow > best_path.total_flow {
				best_path = sub_best
			}
		}

		// move elephant
		distance = solver.distances[ele_pos][name]
		next = path.next_elephant(valve, distance)
		if next.elapsed < PART2_MINUTES {
			var sub_best = s.find_path(solver, next)
			if sub_best.total_flow > best_path.total_flow {
				best_path = sub_best
			}
		}
	}

	s.cache[cache_key] = best_path.diff(path)
	return best_path
}

type TwoPathKey struct {
	human_pos     string
	human_elapsed int
	ele_pos       string
	ele_elapsed   int
	valves        int
}

type TwoPath struct {
	human_path    []string
	human_elapsed int
	ele_path      []string
	ele_elapsed   int
	open_valves   int
	elapsed       int
	total_flow    int
}

func (p *TwoPath) cache_key() TwoPathKey {
	return TwoPathKey{
		p.human_path[len(p.human_path)-1],
		p.human_elapsed,
		p.ele_path[len(p.ele_path)-1],
		p.ele_elapsed,
		p.open_valves,
	}
}

func (p *TwoPath) merge(other TwoPath) TwoPath {
	var human_path = append(p.human_path, other.human_path...)
	var ele_path = append(p.ele_path, other.ele_path...)
	return TwoPath{
		human_path,
		p.human_elapsed + other.human_elapsed,
		ele_path,
		p.ele_elapsed + other.ele_elapsed,
		p.open_valves,
		p.elapsed + other.elapsed,
		p.total_flow + other.total_flow,
	}
}

func (p *TwoPath) next_human(valve *Valve, distance int) TwoPath {
	var human_path = append(p.human_path, valve.name)
	var ele_path = p.ele_path
	var elapsed = p.human_elapsed + distance + 1
	var flow = (PART2_MINUTES - elapsed) * valve.flow
	return TwoPath{
		human_path,
		elapsed,
		ele_path,
		p.ele_elapsed,
		p.open_valves | valve.mask,
		max(elapsed, p.ele_elapsed),
		p.total_flow + flow,
	}

}

func (p *TwoPath) next_elephant(valve *Valve, distance int) TwoPath {
	var human_path = p.human_path
	var ele_path = append(p.ele_path, valve.name)
	var elapsed = p.ele_elapsed + distance + 1
	var flow = (PART2_MINUTES - elapsed) * valve.flow
	return TwoPath{
		human_path,
		p.human_elapsed,
		ele_path,
		elapsed,
		p.open_valves | valve.mask,
		max(elapsed, p.human_elapsed),
		p.total_flow + flow,
	}
}

func (p *TwoPath) diff(start TwoPath) TwoPath {
	var human_path = p.human_path[len(start.human_path):len(p.human_path)]
	var ele_path = p.ele_path[len(start.ele_path):len(p.ele_path)]
	return TwoPath{
		human_path,
		p.human_elapsed - start.human_elapsed,
		ele_path,
		p.ele_elapsed - start.ele_elapsed,
		p.open_valves,
		p.elapsed - start.elapsed,
		p.total_flow - start.total_flow,
	}
}

func InitTwoPath(start string) TwoPath {
	return TwoPath{
		human_path: []string{start},
		ele_path:   []string{start},
	}
}
