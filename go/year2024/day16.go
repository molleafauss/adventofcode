package main

import (
	"adventofcode/utils"
	"aoc/aoc"
	"container/heap"
	"math"
	"slices"
	"strconv"
)

type day16 struct {
	start  utils.GridPos
	end    utils.GridPos
	maze   map[utils.GridPos]bool
	width  int
	height int
}

func init() {
	utils.RegisterSolver("2024", "day16", func() utils.Solver {
		return &day16{
			maze: make(map[utils.GridPos]bool),
		}
	})
}

func (solver *day16) Parse(line string) {
	if solver.width == 0 {
		solver.width = len(line)
	} else if len(line) != solver.width {
		panic("wrong line length: " + line)
	}
	for i := range line {
		switch line[i] {
		case '#':
			solver.maze[utils.RowColToGridPos(i, solver.height)] = true
		case 'S':
			solver.start = utils.RowColToGridPos(i, solver.height)
		case 'E':
			solver.end = utils.RowColToGridPos(i, solver.height)
		}
	}
	solver.height++
}

func (solver *day16) Solve() (*string, *string) {
	score, bestSpots := solver.walkMaze()
	part1 := strconv.Itoa(score)
	part2 := strconv.Itoa(bestSpots)
	return &part1, &part2
}

type walk struct {
	path  []utils.GridPos
	dir   utils.GridPos
	score int
}

type position struct {
	pos utils.GridPos
	dir utils.GridPos
}

func (solver *day16) walkMaze() (int, int) {
	empty := solver.width*solver.height - len(solver.maze)
	utils.Info("Maze size: %dx%d - walls %d - empty: %d", solver.width, solver.height, len(solver.maze), empty)
	minScore := math.MaxInt32
	minScores := make(map[position]int)
	bestSpots := make(map[utils.GridPos]bool)
	// max number of empty spaces - this might be needed for optimizations?
	pq := make(PriorityQueue, 1)
	pq[0] = &walk{[]utils.GridPos{solver.start}, utils.MOVE_R, 0}
	heap.Init(&pq)
	iterations := 0
	for pq.Len() > 0 {
		iterations++
		w := heap.Pop(&pq).(*walk)
		pos := w.path[len(w.path)-1]
		if iterations%10000 == 0 {
			utils.Info("Iterations: %d - queue size: %d - score %d - head: %s/%d - cache: %d", iterations, pq.Len(), minScore, pos, w.score, len(minScores))
		}
		if pos == solver.end {
			if w.score < minScore {
				minScore = w.score
				// reset best spots
				clear(bestSpots)
				for _, pos := range w.path {
					bestSpots[pos] = true
				}
			} else if w.score == minScore {
				// add path to all best spots
				for _, pos := range w.path {
					bestSpots[pos] = true
				}
			}
			continue
		}
		if w.score > minScore {
			// won't be getting any better here
			continue
		}
		// follow all avenues, but never walk back. Prioritise walking in a straight line if possible
		// (should the queue be ordered by score instead?)
		for _, d := range utils.ALL_ORTHOGONAL {
			if opposite(d, w.dir) {
				continue
			}
			next := pos.Add(d)
			// don't return to position walked already
			if slices.Index(w.path, next) != -1 {
				continue
			}
			if _, wall := solver.maze[next]; wall {
				continue
			}
			score := w.score
			if d == w.dir {
				score++
			} else {
				// rotate AND move
				score += 1001
			}
			p := position{pos, d}
			prev, exists := minScores[p]
			if exists && prev < score {
				// we've been there with a better score already - don't go there
				continue
			}
			minScores[p] = score
			// make it depth-first
			heap.Push(&pq, &walk{append(slices.Clone(w.path), next), d, score})
		}
	}
	utils.Info("Found end: iterations: %d - score %d", iterations, minScore)
	return minScore, len(bestSpots)
}

func opposite(a utils.GridPos, b utils.GridPos) bool {
	return (a == utils.MOVE_L && b == utils.MOVE_R) ||
		(a == utils.MOVE_R && b == utils.MOVE_L) ||
		(a == utils.MOVE_U && b == utils.MOVE_D) ||
		(a == utils.MOVE_D && b == utils.MOVE_U)
}

type PriorityQueue []*walk

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	// We want Pop to give us the highest, not lowest, priority so we use greater than here.
	return pq[i].score > pq[j].score
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x any) {
	item := x.(*walk)
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil // don't stop the GC from reclaiming the item eventually
	*pq = old[0 : n-1]
	return item
}
