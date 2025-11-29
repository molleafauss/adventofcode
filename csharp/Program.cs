using System.ComponentModel;
using System.Diagnostics;
using Spectre.Console;
using Spectre.Console.Cli;

namespace adventofcode
{
    // Advent of Code C# runner
    class Program(int year, string day, string inputDir)
    {
        private sealed class Settings : CommandSettings
        {
            [Description("Year of the Advent of Code event - default last available year")]
            [CommandOption("--year <YEAR>")]
            public string Year { get; }

            [Description("Day to solve (specified as 'dayNN' or 'all' to solve all days in sequence")]
            [CommandArgument(0, "<DAY>")]
            public string Day { get; }
            
            [Description("Directory to read input files from, default current directory")]
            [CommandOption("--inputs <INPUT_DIR>")]
            public DirectoryInfo InputDir { get; }

            public Settings(string? year, string day, string inputDir)
            {
                Year = year ?? GetDefaultYear();
                Day = day;
                InputDir = string.IsNullOrEmpty(inputDir) ? new DirectoryInfo(Directory.GetCurrentDirectory()) : new DirectoryInfo(inputDir);
            }
            
            public override ValidationResult Validate()
            {
                if (!InputDir.Exists)
                    return ValidationResult.Error("provided input directory does not exist");
                
                if (Day != "all" && !System.Text.RegularExpressions.Regex.IsMatch(Day, @"^day\d{2}$"))
                    return ValidationResult.Error("day argument must be 'all' or in the format 'dayNN' where NN is the day number");
                
                return ValidationResult.Success();
            }

            private string GetDefaultYear()
            {
                var now = DateTime.Now;
                return ((now.Month >= 12) ? now.Year : now.Year - 1).ToString();
            }
        }

        private class AocCommand : Command<Settings>
        {
            public override int Execute(CommandContext context, Settings settings, CancellationToken cancellationToken)
            {
                if (settings.Day.Equals("all", StringComparison.OrdinalIgnoreCase))
                {
                    SolveAll(settings.Year);
                }
                else
                {
                    SolveDay(settings.Year, settings.Day);
                }
                
                return 0;
            }
        }


        static int Main(string[] args)
        {
            var app = new CommandApp<AocCommand>();

            app.Configure(config =>
            {
                config.AddCommand<AocCommand>("aoc")
                    .WithDescription("Advent of Code solver")
                    .WithExample("aoc", "day03", "--year", "2023")
                    .WithExample("aoc", "all");
            });

            return app.Run(args);
        }

        private static ISolver CreateSolver(string year, string dayNum)
        {
            // Capitalize first character: "day03" -> "Day03"
            string fullName = $"adventofcode.year{year}.{char.ToUpper(dayNum[0])}{dayNum[1..]}";

            // Find the type in loaded assemblies
            Type? type = typeof(Program).Assembly.GetType(fullName, false, true);
            if (type == null)
            {
                throw new ArgumentException($"Solver type not found: {fullName}");
            }

            if (!typeof(ISolver).IsAssignableFrom(type))
                throw new Exception($"{fullName} does not implement ISolver");
            return (ISolver)Activator.CreateInstance(type)!;
        }

        private static void SolveAll(string year)
        {
            foreach (var day in Enumerable.Range(1, 25))
            {
                SolveDay(year, $"day{day:D2}");
            }
        }

        private static void SolveDay(string year, string dayNum)
        {
            Console.WriteLine($"== Solving {year} - {dayNum} ==");

            var solver = CreateSolver(year, dayNum);
            // test file
            string testPath = Path.Combine("inputs", year, dayNum, "test.txt");
            if (!File.Exists(testPath))
            {
                throw new Exception($"Test file missing: {testPath})");
            }

            Solve(testPath, solver);

            // re-create solver
            solver = CreateSolver(year, dayNum);
            // input file
            string inputPath = Path.Combine("inputs", year, dayNum, "input.txt");
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
            var sw = Stopwatch.StartNew();
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
            sw.Stop();
            Console.WriteLine($"{filePath} - solved in {PrintElapsed(sw.Elapsed)}");


            if (part1 == expected1)
                Console.WriteLine($"PART 1 - OK (expected {expected1})");
            else
                Console.WriteLine($"PART 1 - ERROR expected {expected1} actual {part1}");

            if (part2 == expected2)
                Console.WriteLine($"PART 2 - OK (expected {expected2})");
            else
                Console.WriteLine($"PART 2 - ERROR expected {expected2} actual {part2}");
        }

        private static string PrintElapsed(TimeSpan elapsed)
        {
            if (elapsed.TotalMilliseconds > 1000)
            {
                return $"{elapsed.TotalSeconds:F2}s";
            }

            if (elapsed.TotalMicroseconds > 1000)
            {
                return $"{elapsed.TotalMilliseconds:F2}ms";
            }

            return elapsed.TotalNanoseconds > 1000
                ? $"{elapsed.TotalMicroseconds:F2}us"
                : $"{elapsed.TotalNanoseconds:F0}ns";
        }
    }
}