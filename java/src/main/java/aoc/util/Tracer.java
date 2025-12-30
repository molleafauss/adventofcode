package aoc.util;

import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.Callable;

public class Tracer {
    private static final Map<String, Stats> timers = new HashMap<>();

    private static class Stats {
        private final boolean isTime;
        String name;
        long count;
        long accum;
        long max;
        long min;

        public Stats(String name, long initial, boolean isTime) {
            this.name = name;
            this.max = this.min = initial;
            this.isTime = isTime;
        }

        public String dump(long totalTime) {
            double avg = accum * 1.0 / count;
            if (isTime) {
                double perc = accum * 100.0 / totalTime;
                return String.format("%s: calls: %d, avg: %.2fns, max: %dns, min: %dns, %.2fX of " +
                                "total",
                        name, count, avg, max, min, perc);
            } else {
                return String.format("%s: count: %d, avg: %.2f, max: %d, min: %d", name, count,
                        avg, max, min);
            }
        }
    }

    public static void reset() {
        timers.clear();
    }

    public static void dumpStats(long totalTime) {
        for (Stats stats : timers.values()) {
            Log.info(stats.dump(totalTime));
        }
    }

    public static void addMeasure(String name, long point, boolean isTime) {
        var stats = timers.computeIfAbsent(name, k -> new Stats(name, point, isTime));
        stats.count++;
        stats.accum += point;
        stats.min = Math.min(stats.min, point);
        stats.max = Math.max(stats.max, point);
    }

    public static <T> T traceCall(String name, Callable<T> callable) throws Exception {
        long t0 = System.nanoTime();
        try {
            return callable.call();
        } finally {
            long delta = System.nanoTime() - t0;
            addMeasure(name, delta, true);
        }
    }
}
