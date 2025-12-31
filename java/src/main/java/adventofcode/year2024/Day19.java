package adventofcode.year2024;

import adventofcode.api.Results;
import adventofcode.api.Solver;
import adventofcode.util.Log;

import java.util.*;

/* Found this java solution in one of the leaderboards. It's very easy and I could not come up with
 * a solution.
 */
public class Day19 implements Solver {
    private final List<String> patterns = new ArrayList<>();
    private int part1;

    @Override
    public void parse(String line) {
        if (patterns.isEmpty()) {
            Collections.addAll(patterns, line.split(", "));
        } else if (!line.isEmpty()) {
            var count = makeDesign(line, new HashMap<>());
            if (count > 0) {
                part1++;
            }
        }
    }

    @Override
    public Results solve() {
        return new Results(String.valueOf(part1), null);
    }

    private Long makeDesign(String design, Map<String, Long> cache) {
        Log.debug("Checking design: %s", design);
        // found a matching design
        if (design.isEmpty()) {
            return 1L;
        }
        if (!cache.containsKey(design)) {
            var count = patterns.stream()
                    .filter(design::startsWith)
                    .mapToLong(it -> makeDesign(design.substring(it.length()), cache))
                    .sum();
            cache.put(design, count);
            Log.debug("Caching design %s - %d", design, count);
        }
        return cache.get(design);
    }
}
