package main

import (
	"aoc/aoc"
	"container/list"
	"fmt"
	"regexp"
	"slices"
	"strconv"
	"strings"
	"time"
)

type day16 struct {
	valves     map[string]*Valve
	connection map[string][]string
}

func Day16() aoc.Solver {
	return &day16{
		valves:     make(map[string]*Valve),
		connection: make(map[string][]string),
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
		name: matches[1],
		flow: flow,
		mask: 0,
	}
	solver.valves[valve.name] = &valve
	solver.connection[valve.name] = strings.Split(matches[3], ", ")
}

func (solver *day16) Solve() (*string, *string) {
	aoc.Info("Found %d valves to open in %d minutes", len(solver.valves), PART1_MINUTES)

	var valvesWithFlow = solver.calculateDistances()

	aoc.Info("Valves with flow: %d", len(valvesWithFlow))

	start, ok := solver.valves[START]
	if !ok {
		panic("Start valve not found in the input??")
	}
	var t0 = time.Now()
	var onePath = OnePathSolver{
		cache:          make(map[OnePathKey]OnePath),
		valvesWithFlow: valvesWithFlow,
	}
	bestPath1 := onePath.findPath(OnePath{visited: []*Valve{start}})
	var delta = time.Since(t0)
	var valvePath = solver.makePath(bestPath1.visited)
	aoc.Info("[1] Found max flow is %d: %s (%d cache hits, %d calls, %d cache size) [%.3fsec]",
		bestPath1.totalFlow, valvePath, onePath.cacheHits, onePath.calls, len(onePath.cache),
		delta.Seconds())

	t0 = time.Now()
	var twoPath = TwoPathSolver{
		cache:          make(map[TwoPathKey]TwoPath),
		valvesWithFlow: valvesWithFlow,
	}
	var bestPath2 = twoPath.findPath(InitTwoPath(len(valvesWithFlow)))
	delta = time.Since(t0)
	aoc.Info("[2] Found max flow is %d: %s / %s (%d cache hits, %d calls, %d cache size) [%.3fsec]",
		bestPath2.totalFlow, bestPath2.humanPath, bestPath2.elePath, twoPath.cacheHits, twoPath.calls,
		len(twoPath.cache), delta.Seconds())

	part1 := strconv.Itoa(bestPath1.totalFlow)
	part2 := strconv.Itoa(bestPath2.totalFlow)
	return &part1, &part2
}

func (solver *day16) calculateDistances() []*Valve {
	type distanceWrapper struct {
		cave     string
		distance int
	}
	var valvesWithFlow []*Valve
	start, ok := solver.valves[START]
	if !ok {
		panic("Start valve not found in the list??: ")
	}
	valvesWithFlow = append(valvesWithFlow, start)

	for _, valve := range solver.valves {
		if valve.flow > 0 {
			id := len(valvesWithFlow)
			valve.mask = 1 << (id - 1)
			valve.id = uint8(len(valvesWithFlow))
			valvesWithFlow = append(valvesWithFlow, valve)
		}
	}

	for _, curr := range valvesWithFlow {
		curr.tunnels = make([]uint8, len(valvesWithFlow))
		var visited = map[string]bool{curr.name: true}
		var queue = list.New()
		queue.PushBack(&distanceWrapper{curr.name, 0})
		for queue.Len() > 0 {
			var wrapper = queue.Remove(queue.Front()).(*distanceWrapper)
			for _, nextName := range solver.connection[wrapper.cave] {
				if _, ok := visited[nextName]; ok {
					continue
				}
				visited[nextName] = true
				next, ok := solver.valves[nextName]
				if !ok {
					panic(fmt.Sprintf("Valve connections for %s refer to an invalid value %s",
						wrapper.cave, nextName))
				}
				if next.flow > 0 || next.name == START {
					curr.tunnels[next.id] = uint8(wrapper.distance + 1)
				}
				queue.PushBack(&distanceWrapper{next.name, wrapper.distance + 1})
			}
		}
		aoc.Info("Distances for %s: %v", curr.name, curr.tunnels)
	}

	return valvesWithFlow
}

func (solver *day16) makePath(visited []*Valve) string {
	var path = "["
	for _, valve := range visited {
		path += valve.name + ","
	}
	return path[:len(path)-1] + "]"
}

type Valve struct {
	id      uint8
	tunnels []uint8
	name    string
	flow    int
	mask    uint
}

type OnePathSolver struct {
	cacheHits      int
	calls          int
	cache          map[OnePathKey]OnePath
	valvesWithFlow []*Valve
}

func (s *OnePathSolver) findPath(path OnePath) OnePath {
	s.calls += 1
	if (s.calls % 1000000) == 0 {
		aoc.Info("%d calls, %d cache hits...", s.calls, s.cacheHits)
	}
	var cacheKey = path.cacheKey()
	if cached, ok := s.cache[cacheKey]; ok {
		s.cacheHits += 1
		return path.merge(cached)
	}

	var cave = path.visited[len(path.visited)-1]
	var bestPath = path
	for _, valve := range s.valvesWithFlow {
		if valve.flow == 0 {
			continue
		}
		if (path.openValves & valve.mask) != 0 {
			continue
		}
		var distance = valve.tunnels[cave.id]
		var next = path.next(valve, distance)
		if next.elapsed >= PART1_MINUTES {
			continue
		}
		var subBest = s.findPath(next)
		if subBest.totalFlow > bestPath.totalFlow {
			bestPath = subBest
		}
	}

	s.cache[cacheKey] = bestPath.diff(path)
	return bestPath
}

type OnePathKey struct {
	name    string
	elapsed uint8
	valves  uint
}

type OnePath struct {
	visited    []*Valve
	openValves uint
	elapsed    uint8
	totalFlow  int
}

func (p *OnePath) cacheKey() OnePathKey {
	return OnePathKey{p.visited[len(p.visited)-1].name, p.elapsed, p.openValves}
}

func (p *OnePath) merge(other OnePath) OnePath {
	var visited = slices.Clone(p.visited)
	visited = append(visited, other.visited...)
	return OnePath{
		visited,
		p.openValves,
		p.elapsed + other.elapsed,
		p.totalFlow + other.totalFlow,
	}
}

func (p *OnePath) next(valve *Valve, distance uint8) OnePath {
	var visited = slices.Clone(p.visited)
	visited = append(visited, valve)
	var elapsed = p.elapsed + distance + 1
	var flow = (PART1_MINUTES - int(elapsed)) * valve.flow
	return OnePath{
		visited,
		p.openValves | valve.mask,
		elapsed,
		p.totalFlow + flow}
}

func (p *OnePath) diff(path OnePath) OnePath {
	var visited = p.visited[len(path.visited):len(p.visited)]
	return OnePath{
		visited,
		p.openValves,
		p.elapsed - path.elapsed,
		p.totalFlow - path.totalFlow,
	}
}

type TwoPathSolver struct {
	cacheHits      int
	calls          int
	cache          map[TwoPathKey]TwoPath
	valvesWithFlow []*Valve
}

func (s *TwoPathSolver) findPath(path TwoPath) TwoPath {
	s.calls += 1
	if (s.calls % 1000000) == 0 {
		aoc.Info("%d calls, %d cache hits...", s.calls, s.cacheHits)
	}

	var cacheKey = path.cacheKey()
	if cached, ok := s.cache[cacheKey]; ok {
		s.cacheHits += 1
		return path.merge(&cached)
	}

	var manPos = path.humanPath[path.humanPos]
	var manValve = s.valvesWithFlow[manPos]
	var elePos = path.elePath[path.elePos]
	var eleValve = s.valvesWithFlow[elePos]

	var bestPath = path
	for _, valve := range s.valvesWithFlow {
		if valve.flow == 0 {
			continue
		}
		// try to move both human and elephant towards the next valve
		if (path.openValves & valve.mask) != 0 {
			continue
		}
		// move human
		var distance = manValve.tunnels[valve.id]
		var next = path.nextHuman(valve, distance)
		if next.elapsed < PART2_MINUTES {
			var subBest = s.findPath(next)
			if subBest.totalFlow > bestPath.totalFlow {
				bestPath = subBest
			}
		}

		// move elephant
		distance = eleValve.tunnels[valve.id]
		next = path.nextElephant(valve, distance)
		if next.elapsed < PART2_MINUTES {
			var subBest = s.findPath(next)
			if subBest.totalFlow > bestPath.totalFlow {
				bestPath = subBest
			}
		}
	}

	s.cache[cacheKey] = bestPath.diff(&path)
	return bestPath
}

type TwoPathKey struct {
	humanPos     uint8
	humanElapsed uint8
	elePos       uint8
	eleElapsed   uint8
	valves       uint
}

type TwoPath struct {
	humanPath    []uint8
	humanPos     uint8
	humanElapsed uint8
	elePath      []uint8
	elePos       uint8
	eleElapsed   uint8
	openValves   uint
	elapsed      uint8
	totalFlow    int
}

func (p *TwoPath) cacheKey() TwoPathKey {
	return TwoPathKey{
		p.humanPath[p.humanPos],
		p.humanElapsed,
		p.elePath[p.elePos],
		p.eleElapsed,
		p.openValves,
	}
}

func (p *TwoPath) merge(other *TwoPath) TwoPath {
	var humanPath = slices.Clone(p.humanPath)
	copy(humanPath[p.humanPos+1:p.humanPos+other.humanPos+1],
		other.humanPath[0:other.humanPos+1])
	var elePath = slices.Clone(p.elePath)
	copy(elePath[p.elePos+1:p.elePos+other.elePos+1], other.elePath[0:other.elePos+1])
	return TwoPath{
		humanPath,
		p.humanPos + other.humanPos,
		p.humanElapsed + other.humanElapsed,
		elePath,
		p.elePos + other.elePos,
		p.eleElapsed + other.eleElapsed,
		p.openValves,
		p.elapsed + other.elapsed,
		p.totalFlow + other.totalFlow,
	}
}

func (p *TwoPath) nextHuman(valve *Valve, distance uint8) TwoPath {
	var humanPath = slices.Clone(p.humanPath)
	humanPath[p.humanPos+1] = valve.id
	var elapsed = p.humanElapsed + distance + 1
	var flow = (PART2_MINUTES - int(elapsed)) * valve.flow
	return TwoPath{
		humanPath,
		p.humanPos + 1,
		elapsed,
		slices.Clone(p.elePath),
		p.elePos,
		p.eleElapsed,
		p.openValves | valve.mask,
		max(elapsed, p.eleElapsed),
		p.totalFlow + flow,
	}

}

func (p *TwoPath) nextElephant(valve *Valve, distance uint8) TwoPath {
	var elePath = slices.Clone(p.elePath)
	elePath[p.elePos+1] = valve.id
	var elapsed = p.eleElapsed + distance + 1
	var flow = (PART2_MINUTES - int(elapsed)) * valve.flow
	return TwoPath{
		slices.Clone(p.humanPath),
		p.humanPos,
		p.humanElapsed,
		elePath,
		p.elePos + 1,
		elapsed,
		p.openValves | valve.mask,
		max(elapsed, p.humanElapsed),
		p.totalFlow + flow,
	}
}

func (p *TwoPath) diff(start *TwoPath) TwoPath {
	var humanPath = make([]uint8, len(p.humanPath))
	copy(humanPath, p.humanPath[start.humanPos+1:p.humanPos+1])
	var elePath = make([]uint8, len(p.elePath))
	copy(elePath, p.elePath[start.elePos+1:p.elePos+1])
	return TwoPath{
		humanPath,
		p.humanPos - start.humanPos,
		p.humanElapsed - start.humanElapsed,
		elePath,
		p.elePos - start.elePos,
		p.eleElapsed - start.eleElapsed,
		p.openValves,
		p.elapsed - start.elapsed,
		p.totalFlow - start.totalFlow,
	}
}

func InitTwoPath(size int) TwoPath {
	return TwoPath{
		humanPath: make([]uint8, size),
		elePath:   make([]uint8, size),
	}
}
