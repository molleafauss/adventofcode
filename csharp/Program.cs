using adventofcode.year2025;

namespace adventofcode
{
    // Advent of Code C# runner
    class Program
    {
        static int Main(string[] args)
        {
            if (args.Length == 0)
            {
                Console.Error.WriteLine("Usage: aoc <day|all> [--year <YYYY>]");
                return -1;
            }

            // Find positional arg (first non-option)
            string? dayArg = null;
            int? yearArg = null;
            for (int i = 0; i < args.Length; i++)
            {
                var a = args[i];
                if (a == "--year" || a == "-y")
                {
                    if (i + 1 >= args.Length)
                    {
                        Console.Error.WriteLine("--year requires a value");
                        return -1;
                    }
                    if (!int.TryParse(args[i + 1], out int y))
                    {
                        Console.Error.WriteLine("Invalid year value: " + args[i + 1]);
                        return -1;
                    }
                    yearArg = y;
                    i++; // skip value
                }
                else if (a == "--help" || a == "-h")
                {
                    PrintHelp();
                    return 0;
                }
                else if (!a.StartsWith("-") && dayArg == null)
                {
                    dayArg = a;
                }
                else
                {
                    Console.Error.WriteLine("Unknown arg: " + args[i]);
                    return -1;
                }
            }

            if (string.IsNullOrEmpty(dayArg))
            {
                Console.Error.WriteLine("Missing day argument (e.g. day03 or all)");
                return -1;
            }

            // Compute default year: if today is Dec or later use current year, otherwise previous year
            var now = DateTime.Now;
            int defaultYear = (now.Month >= 12) ? now.Year : now.Year - 1;
            int year = yearArg ?? defaultYear;

            try
            {
                if (dayArg.Equals("all", StringComparison.OrdinalIgnoreCase))
                {
                    SolveAll(year);
                }

                if (!dayArg.StartsWith("day"))
                {
                    Console.Error.WriteLine("Invalid day argument (e.g. day03 or all)");
                    return -1;
                }

                SolveDay(year, dayArg);
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine("Unhandled error: " + ex);
                return -1;
            }
            return 0;
        }

        static void PrintHelp()
        {
            Console.WriteLine("Usage: aoc <dayNN|all> [--year <YYYY>]");
            Console.WriteLine();
            Console.WriteLine("day   : dayNN (e.g. day03) solve specific day");
            Console.WriteLine("all   : solve all available days");
            Console.WriteLine("--year, -y : optional year (defaults to most recent AoC year based on current date)");
        }

        private static void SolveAll(int year)
        {
            foreach (var day in Enumerable.Range(1, 25))
            {
                SolveDay(year, $"day{day:D2}");
            }
        }

        private static ISolver CreateSolver(int year, string dayNum)
        {
            return new Day01();
        }

        private static void SolveDay(int year, string dayNum)
        {
            var solver = CreateSolver(year, dayNum);
            Console.WriteLine($"== Solving {dayNum} for {year} ==");

            // test file
            string testPath = Path.Combine("inputs", year.ToString(), dayNum, "test.txt");
            if (!File.Exists(testPath))
            {
                throw new Exception($"Test file missing: {testPath})");
            }

            Console.WriteLine($"== Solving {year} - {dayNum} ==");
            Solve(testPath, solver);

            // input file
            string inputPath = Path.Combine("inputs", year.ToString(), dayNum, "input.txt");
            if (!File.Exists(inputPath))
            {
                throw new Exception($"Puzzle file missing: {inputPath})");
            }
            Solve(inputPath, solver);
        }

        private static void Solve(string filePath, ISolver solver)
        {
            string? expected1 = null;
            string? expected2 = null;
            foreach (var l in File.ReadAllLines(filePath))
            {
                if (l.StartsWith("result part 1:"))
                {
                    expected1 = l.Substring("result part 1:".Length).Trim();
                }
                else if (l.StartsWith("result part 2:"))
                {
                    expected2 = l.Substring("result part 2:".Length).Trim();
                }
                else
                {
                    solver.Parse(l);
                }
            }
            var (part1, part2) = solver.Solve();
            
            if (part1 == expected1)
                Console.WriteLine($"PART 1 - OK (expected {expected1})");
            else
                Console.WriteLine($"PART 1 - ERROR expected {expected1} actual {part1}");

            if (part2 == expected2)
                Console.WriteLine($"PART 2 - OK (expected {expected2})");
            else
                Console.WriteLine($"PART 2 - ERROR expected {expected2} actual {part2}");
        }
    }
}
