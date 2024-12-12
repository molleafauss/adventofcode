import aoc.LogLevel;
import aoc.Runner;

import java.io.IOException;

public class AocMain {
    public static void main(String[] args) throws IOException {
        new Runner(LogLevel.INFO).run(args, new aoc.year2022.Solvers());
    }
}