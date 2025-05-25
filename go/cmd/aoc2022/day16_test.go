package main

import (
	"testing"
)

// Helper function to create a test valve
func createTestValve(id uint8, name string, flow int, mask uint) Valve {
	return Valve{
		id:      id,
		tunnels: []uint8{},
		name:    name,
		flow:    flow,
		mask:    mask,
	}
}

func TestTwoPathsCacheKey(t *testing.T) {
	path := InitTwoPath(5)
	path.humanPath[0] = 1
	path.humanElapsed = 10
	path.elePath[0] = 2
	path.eleElapsed = 15
	path.openValves = 0x3

	key := path.cacheKey()
	expected := TwoPathKey{1, 10, 2, 15, 0x3}
	if key != expected {
		t.Errorf("expected %v, got %v", expected, key)
	}
}

func TestTwoPathsNextHuman(t *testing.T) {
	path := InitTwoPath(5)
	valve := createTestValve(1, "BB", 10, 0x2)

	next := path.nextHuman(&valve, 2)

	if next.humanPath[1] != 1 {
		t.Errorf("expected %d, got %d", 1, next.humanPath[1])
	}
	if next.humanPos != 1 {
		t.Errorf("expected %d, got %d", 1, next.humanPos)
	}
	if next.humanElapsed != 3 {
		t.Errorf("expected %d, got %d", 3, next.humanElapsed)
	}
	if next.elePos != 0 {
		t.Errorf("expected %d, got %d", 0, next.elePos)
	}
	if next.eleElapsed != 0 {
		t.Errorf("expected %d, got %d", 0, next.eleElapsed)
	}
	if next.openValves != 0x2 {
		t.Errorf("expected %d, got %d", 0x2, next.openValves)
	}
	expectedFlow := (PART2_MINUTES - 3) * 10
	if next.totalFlow != expectedFlow {
		t.Errorf("expected %d, got %d", expectedFlow, next.totalFlow)
	}
}

func TestTwoPathsNextElephant(t *testing.T) {
	path := InitTwoPath(5)
	valve := createTestValve(1, "BB", 10, 0x2)

	next := path.nextElephant(&valve, 2)

	if next.elePath[1] != 1 {
		t.Errorf("expected %d, got %d", 1, next.elePath[1])
	}
	if next.elePos != 1 {
		t.Errorf("expected %d, got %d", 1, next.elePos)
	}
	if next.eleElapsed != 3 {
		t.Errorf("expected %d, got %d", 3, next.eleElapsed)
	}
	if next.humanPos != 0 {
		t.Errorf("expected %d, got %d", 0, next.humanPos)
	}
	if next.humanElapsed != 0 {
		t.Errorf("expected %d, got %d", 0, next.humanElapsed)
	}
	if next.openValves != 0x2 {
		t.Errorf("expected %d, got %d", 0x2, next.openValves)
	}
	expectedFlow := (PART2_MINUTES - 3) * 10
	if next.totalFlow != expectedFlow {
		t.Errorf("expected %d, got %d", expectedFlow, next.totalFlow)
	}
}

func TestTwoPathsMerge(t *testing.T) {
	path1 := InitTwoPath(5)
	path1.humanPath[0] = 1
	path1.humanPos = 0
	path1.humanElapsed = 5
	path1.elePath[0] = 2
	path1.elePos = 0
	path1.eleElapsed = 6
	path1.totalFlow = 100

	path2 := InitTwoPath(5)
	path2.humanPath[0] = 3
	path2.humanPos = 1
	path2.humanElapsed = 3
	path2.elePath[0] = 4
	path2.elePos = 1
	path2.eleElapsed = 4
	path2.totalFlow = 50

	merged := path1.merge(&path2)

	if merged.humanPath[0] != 1 {
		t.Errorf("expected %d, got %d", 1, merged.humanPath[0])
	}
	if merged.humanPath[1] != 3 {
		t.Errorf("expected %d, got %d", 3, merged.humanPath[1])
	}
	if merged.humanPos != 1 {
		t.Errorf("expected %d, got %d", 1, merged.humanPos)
	}
	if merged.humanElapsed != 8 {
		t.Errorf("expected %d, got %d", 8, merged.humanElapsed)
	}
	if merged.elePath[0] != 2 {
		t.Errorf("expected %d, got %d", 2, merged.elePath[0])
	}
	if merged.elePath[1] != 4 {
		t.Errorf("expected %d, got %d", 4, merged.elePath[1])
	}
	if merged.elePos != 1 {
		t.Errorf("expected %d, got %d", 1, merged.elePos)
	}
	if merged.eleElapsed != 10 {
		t.Errorf("expected %d, got %d", 10, merged.eleElapsed)
	}
	if merged.totalFlow != 150 {
		t.Errorf("expected %d, got %d", 150, merged.totalFlow)
	}
}

func TestTwoPathsMergeNoMove(t *testing.T) {
	path1 := InitTwoPath(5)
	path1.humanPath[0] = 1
	path1.humanPos = 0
	path1.humanElapsed = 5
	path1.elePath[0] = 2
	path1.elePos = 0
	path1.eleElapsed = 6
	path1.totalFlow = 100

	path2 := InitTwoPath(5)
	path2.humanPos = 0
	path2.humanElapsed = 0
	path2.elePath[0] = 4
	path2.elePos = 1
	path2.eleElapsed = 4
	path2.totalFlow = 50

	merged := path1.merge(&path2)

	if merged.humanPath[0] != 1 {
		t.Errorf("expected %d, got %d", 1, merged.humanPath[0])
	}
	if merged.humanPos != 0 {
		t.Errorf("expected %d, got %d", 0, merged.humanPos)
	}
	if merged.humanElapsed != 5 {
		t.Errorf("expected %d, got %d", 5, merged.humanElapsed)
	}
	if merged.elePath[0] != 2 {
		t.Errorf("expected %d, got %d", 2, merged.elePath[0])
	}
	if merged.elePath[1] != 4 {
		t.Errorf("expected %d, got %d", 4, merged.elePath[1])
	}
	if merged.elePos != 1 {
		t.Errorf("expected %d, got %d", 1, merged.elePos)
	}
	if merged.eleElapsed != 10 {
		t.Errorf("expected %d, got %d", 10, merged.eleElapsed)
	}
	if merged.totalFlow != 150 {
		t.Errorf("expected %d, got %d", 150, merged.totalFlow)
	}
}

func TestTwoPathsDiff(t *testing.T) {
	path1 := InitTwoPath(5)
	path1.humanPath[0] = 1
	path1.humanPath[1] = 2
	path1.humanPos = 1
	path1.humanElapsed = 5
	path1.elePath[0] = 2
	path1.elePath[1] = 3
	path1.elePos = 1
	path1.eleElapsed = 6
	path1.totalFlow = 100

	path2 := InitTwoPath(5)
	path2.humanPath[0] = 1
	path2.humanPos = 0
	path2.humanElapsed = 1
	path2.elePath[0] = 2
	path2.elePos = 0
	path2.eleElapsed = 1
	path2.totalFlow = 30

	diff := path1.diff(&path2)

	if diff.humanPath[0] != 2 {
		t.Errorf("expected %d, got %d", 2, diff.humanPath[0])
	}
	if diff.humanPos != 1 {
		t.Errorf("expected %d, got %d", 1, diff.humanPos)
	}
	if diff.humanElapsed != 4 {
		t.Errorf("expected %d, got %d", 4, diff.humanElapsed)
	}
	if diff.elePath[0] != 3 {
		t.Errorf("expected %d, got %d", 3, diff.elePath[0])
	}
	if diff.elePos != 1 {
		t.Errorf("expected %d, got %d", 1, diff.elePos)
	}
	if diff.eleElapsed != 5 {
		t.Errorf("expected %d, got %d", 5, diff.eleElapsed)
	}
	if diff.totalFlow != 70 {
		t.Errorf("expected %d, got %d", 70, diff.totalFlow)
	}
}

func TestTwoPathsDiffHumanHasntMoved(t *testing.T) {
	path1 := InitTwoPath(5)
	path1.humanPath[0] = 1
	path1.humanPath[1] = 2
	path1.humanPos = 1
	path1.humanElapsed = 5
	path1.elePath[0] = 2
	path1.elePath[1] = 3
	path1.elePos = 1
	path1.eleElapsed = 6
	path1.totalFlow = 100

	path2 := InitTwoPath(5)
	path2.humanPath[0] = 1
	path2.humanPath[1] = 2
	path2.humanPos = 1
	path2.humanElapsed = 5
	path2.elePath[0] = 2
	path2.elePos = 0
	path2.eleElapsed = 2
	path2.totalFlow = 30

	diff := path1.diff(&path2)

	if diff.humanPos != 0 {
		t.Errorf("expected %d, got %d", 0, diff.humanPos)
	}
	if diff.humanElapsed != 0 {
		t.Errorf("expected %d, got %d", 0, diff.humanElapsed)
	}
	if diff.elePath[0] != 3 {
		t.Errorf("expected %d, got %d", 3, diff.elePath[0])
	}
	if diff.elePos != 1 {
		t.Errorf("expected %d, got %d", 1, diff.elePos)
	}
	if diff.eleElapsed != 4 {
		t.Errorf("expected %d, got %d", 4, diff.eleElapsed)
	}
	if diff.totalFlow != 70 {
		t.Errorf("expected %d, got %d", 70, diff.totalFlow)
	}
}
