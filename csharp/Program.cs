using System.ComponentModel;
using System.Diagnostics;
using Spectre.Console;
using Spectre.Console.Cli;

using adventofcode.utils;

namespace adventofcode
{
    // Advent of Code C# runner
    class Program
    {
        private sealed class Settings : CommandSettings
        {
            [Description("Year of the Advent of Code event - default last available year")]
            [CommandOption("--year <YEAR>")]
            public string Year { get; }

            [Description("Day to solve (specified as 'dayNN' or 'all' to solve all days in sequence)")]
            [CommandArgument(0, "<DAY>")]
            public string Day { get; }
            
            [Description("Directory to read input files from, default current directory")]
            [CommandOption("--inputs <INPUT_DIR>")]
            public DirectoryInfo InputDir { get; }

            [Description("Enables debug logs")]
            [CommandOption("--debug")]
            public bool Debug { get; }

            public Settings(string? year, string day, DirectoryInfo? inputDir, bool? debug)
            {
                Year = year ?? GetDefaultYear();
                Day = day;
                InputDir = inputDir ?? new DirectoryInfo(Directory.GetCurrentDirectory());
                Debug = debug ?? false;
            }
            
            public override ValidationResult Validate()
            {
                if (!InputDir.Exists)
                    return ValidationResult.Error("provided input directory does not exist");
                
                if (Day != "all" && !System.Text.RegularExpressions.Regex.IsMatch(Day, @"^day\d{2}"))
                    return ValidationResult.Error("day argument must be 'all' or in the format 'dayNN' where NN is the day number");
                
                return ValidationResult.Success();
            }

            private static string GetDefaultYear()
            {
                var now = DateTime.Now;
                return ((now.Month >= 12) ? now.Year : now.Year - 1).ToString();
            }
        }

        static int Main(string[] args)
        {
            return new CommandApp<AocCommand>().Run(args);
        }

        private class AocCommand : Command<Settings>
        {
            public override int Execute(CommandContext context, Settings settings, CancellationToken cancellationToken)
            {
                Log.SetLevel(settings.Debug ? LogLevel.Debug : LogLevel.Info);

                if (settings.Day.Equals("all", StringComparison.OrdinalIgnoreCase))
                {
                    SolveAll(settings);
                }
                else
                {
                    SolveDay(settings, settings.Day);
                }
                
                return 0;
            }
        }

        private static void SolveAll(Settings settings)
        {
            foreach (var day in Enumerable.Range(1, 25))
            {
                SolveDay(settings, $"day{day:D2}");
            }
        }

        private static void SolveDay(Settings settings, string dayNum)
        {
            var puzzle = dayNum[..5];
            Log.Info($"== Solving {settings.Year}/{puzzle} ==");

            var solver = CreateSolver(settings.Year, dayNum);
            if (solver == null)
            {
                Log.Warn($"{settings.Year}/{dayNum} | no solution implemented");
                return;
            }

            Solve(settings, puzzle, "test.txt", solver);
            Solve(settings, puzzle, "input.txt", CreateSolver(settings.Year, dayNum));
        }

        private static ISolver? CreateSolver(string year, string dayNum)
        {
            // Capitalize first character: "day03" -> "Day03"
            string fullName = $"adventofcode.year{year}.{char.ToUpper(dayNum[0])}{dayNum[1..]}";

            // Find the type in loaded assemblies
            Type? type = typeof(Program).Assembly.GetType(fullName, false, true);
            if (type == null)
            {
                return null;
            }

            if (!typeof(ISolver).IsAssignableFrom(type))
                return null;
            return (ISolver)Activator.CreateInstance(type)!;
        }

        private static void Solve(Settings settings, string puzzle, string filePath, ISolver solver)
        {
            // test file
            string inputData = Path.Combine(settings.InputDir.FullName, settings.Year, puzzle, filePath);
            if (!File.Exists(inputData))
            {
                Log.Warn($"{settings.Year}/{puzzle} | missing file: {filePath}");
                return;
            }
            string? expected1 = null;
            string? expected2 = null;
            var sw = Stopwatch.StartNew();
            foreach (var l in File.ReadAllLines(inputData))
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
            Log.Info($"{settings.Year}/{puzzle} | {filePath} solved in {PrintElapsed(sw.Elapsed)}");
            
            verify(settings.Year, puzzle, filePath, 1, expected1, part1);
            verify(settings.Year, puzzle, filePath, 2, expected2, part2);
        }

        private static void verify(string year, string puzzle, string filePath, int part, string? expected, string? result)
        {
            if (expected == null)
            {
                return;
            }

            if (expected == result)
            {
                Log.Info($"{year}/{puzzle} | {filePath} | RESULT PART {part} - correct: {expected}");
            }
            else
            {
                Log.Error($"{year}/{puzzle} | {filePath} | RESULT PART {part} - expected {expected}, actual {result}");
            }
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