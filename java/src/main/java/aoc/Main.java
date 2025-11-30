package aoc;

import aoc.api.Solver;
import aoc.util.Log;
import aoc.util.LogLevel;
import picocli.CommandLine;
import picocli.CommandLine.Command;
import picocli.CommandLine.IDefaultValueProvider;
import picocli.CommandLine.Model.ArgSpec;
import picocli.CommandLine.Model.CommandSpec;
import picocli.CommandLine.Option;
import picocli.CommandLine.Parameters;
import picocli.CommandLine.Spec;

import java.io.File;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.time.LocalDate;
import java.util.Objects;
import java.util.concurrent.Callable;

@Command(name = "aoc", mixinStandardHelpOptions = true, defaultValueProvider =
        Main.DefaultValues.class,
        description = "Solves Advent of Code puzzles")
public class Main implements Callable<Integer> {

    static class DefaultValues implements IDefaultValueProvider {
        @Override
        public String defaultValue(ArgSpec argSpec) {
            if (argSpec.paramLabel().equals("<year>")) {
                var now = LocalDate.now();
                return String.valueOf(now.getMonthValue() >= 12 ? now.getYear() : now.getYear() - 1);
            }
            return "";
        }
    }

    @Spec
    CommandSpec spec;

    @Option(names = "--inputs", description = "Directory to read input files from, default current directory.")
    private File inputsDir = new File(".");

    @Option(names = "--year", description = "Year of the Advent of Code event - default last available year.")
    private String year;

    private String dayArg;

    @Option(names = "--debug", description = "Enable debug log.")
    private boolean debug = false;

    public static void main(String[] args) {
        System.exit(new CommandLine(new Main()).execute(args));
    }

    @Parameters(description = "Day to solve (specified as 'dayNN' or 'all' to solve all days in sequence).")
    public void setDay(String dayArg) {
        if (!Objects.equals(dayArg, "all") && !dayArg.startsWith("day")) {
            throw new CommandLine.ParameterException(spec.commandLine(),
                    "Invalid day parameter: %s. Must be 'dayNN' or 'all'.".formatted(dayArg));
        }
        this.dayArg = dayArg;
    }

    @Override
    public Integer call() throws Exception {
        validateInputDir();

        Log.setLogLevel(debug ? LogLevel.DEBUG : LogLevel.INFO);

        if (dayArg.equals("all")) {
            solveAll();
        } else {
            solveDay(dayArg);
        }
        return 0;
    }

    private void validateInputDir() {
        if (!inputsDir.exists() || !inputsDir.isDirectory()) {
            throw new CommandLine.ParameterException(spec.commandLine(),
                    "inputs directory %s does not exist or is not a directory".formatted(inputsDir));
        }
        // resolve full path
        inputsDir = inputsDir.getAbsoluteFile();
    }

    private void solveDay(String day) throws IOException {
        String data = day.substring(0, 5);
        Log.info("== Solving %s / %s ==", year, data);

        // assume 'input' is a directory in the current directory
        var testFile = new File("%s/%s/%s/test.txt".formatted(inputsDir, year, data));
        if (!testFile.exists()) {
            Log.error("ERROR: test file %s does not exist", testFile);
            System.exit(-1);
        }
        solve(testFile, createSolver(day));

        var inputFile = new File("%s/%s/%s/input.txt".formatted(inputsDir, year, data));
        if (!inputFile.exists()) {
            Log.error("ERROR: input file %s does not exist".formatted(inputFile));
            System.exit(-1);
        }
        solve(inputFile, createSolver(day));

    }

    private Solver createSolver(String day) {
        String className = "aoc.year%s.%s%s".formatted(year, Character.toUpperCase(day.charAt(0)), day.substring(1));
        try {
            return (Solver) Class.forName(className).getDeclaredConstructor().newInstance();
        } catch (ReflectiveOperationException e) {
            throw new RuntimeException("Cannot create solver for %s".formatted(className), e);
        }
    }

    private void solveAll() throws IOException {
        for (int d = 1; d < 26; d++) {
            var day = String.format("day%02d", d);
            solveDay(day);
        }
    }

    private static void solve(File input, Solver solver) throws IOException {
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