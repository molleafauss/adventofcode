package aoc;

import java.io.File;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.Objects;

public class Runner {
    public Runner(LogLevel logLevel) {
        Log.setLogLevel(logLevel);
    }

    public void run(String[] args, SolverFactory factory) throws IOException {
        if (args.length != 1) {
            Log.error("Missing all or day");
            System.exit(1);
        }

        if (args[0].equals("all")) {
            solveAll(factory);
        } else {
            solveDay(args[0], factory);
        }
    }

    private void solveDay(String day, SolverFactory factory) throws IOException {
        Log.info("== Solving %s ==", day);

        // assume 'input' is a directory in the current directory
        var testFile = new File("inputs/%s/%s/test.txt".formatted(factory.getYear(), day));
        if (!testFile.exists()) {
            Log.error("ERROR: test file %s does not exist", testFile);
            System.exit(-1);
        }
        solve(testFile, factory.createSolver(day));

        var inputFile = new File("inputs/%s/%s/input.txt".formatted(factory.getYear(), day));
        if (!inputFile.exists()) {
            Log.error("ERROR: input file %s does not exist".formatted(inputFile));
            System.exit(-1);
        }
        solve(inputFile, factory.createSolver(day));

    }

    private void solveAll(SolverFactory factory) throws IOException {
        for (int d = 1; d < 26; d++) {
            var day = String.format("day%02d", d);
            solveDay(day, factory);
        }
    }

    private void solve(File input, Solver solver) throws IOException {
        String expectedPart1 = null;
        String expectedPart2 = null;
        var t0 = System.currentTimeMillis();
        for (var line : Files.readAllLines(input.toPath(), StandardCharsets.UTF_8)) {
            if (line.startsWith("result part 1: ")) {
                expectedPart1 = line.substring(15).trim();
            } else if (line.startsWith("result part 2: ")) {
                expectedPart2 = line.substring(15).trim();
            } else {
                solver.parse(line);
            }
        }
        var result = solver.solve();
        var t1 = System.currentTimeMillis();
        Log.info("File %s: %.3fsec", input, (t1 - t0) / 1000.0);
        if (result == null) {
            Log.warn("==> No result given");
            return;
        }
        if (Objects.equals(result.part1(), expectedPart1)) {
            Log.info("PART 1 - found expected result: %s = %s".formatted(expectedPart1, result.part1()));
        } else {
            Log.error("ERROR - part 1 result is incorrect: expected %s, actual %s".formatted(expectedPart1, result.part1()));
        }
        if (Objects.equals(result.part2(), expectedPart2)) {
            Log.info("PART 2 - found expected result: %s = %s".formatted(expectedPart2, result.part2()));
        } else {
            Log.error("ERROR - part 2 result is incorrect: expected %s, actual %s".formatted(expectedPart2, result.part2()));
        }
    }
}
