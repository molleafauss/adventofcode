using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace adventofcode
{
    public class Runner
    {
        // Manual registry of solvers. Add new solvers here.
        private List<ISolver> _solvers = new List<ISolver>()
        {
            // Example solver registration; add more as DayNN instances become available
            new Day01(),
        };

        // Run all registered solvers for a given year
        public int RunAll(int year)
        {
            int exitCode = 0; // 0 OK, 1 mismatch, 3 missing file, 4 exception
            foreach (var solver in _solvers.OrderBy(s => s.DayId))
            {
                int code = RunDayInternal(year, solver.DayId, solver);
                if (code != 0)
                {
                    if (code > exitCode) exitCode = code;
                }
            }
            return exitCode;
        }

        // Run a single day by id like "day01"
        public int RunDay(int year, string dayId)
        {
            var solver = _solvers.FirstOrDefault(s => s.DayId.Equals(dayId, StringComparison.OrdinalIgnoreCase));
            if (solver == null)
            {
                Console.Error.WriteLine($"No solver available for day: {dayId}");
                return 3;
            }
            return RunDayInternal(year, dayId, solver);
        }

        private int RunDayInternal(int year, string dayId, ISolver solver)
        {
            try
            {
                Console.WriteLine($"== Solving {dayId} for {year} ==");

                int exitCode = 0;

                // test file
                string testPath = Path.Combine("inputs", year.ToString(), dayId, "test.txt");
                if (File.Exists(testPath))
                {
                    Console.WriteLine($"-- file: {testPath}");
                    var (expected1, expected2, content) = LoadFileAndExpected(testPath);
                    var (got1, got2) = solver.Solve(content);
                    ReportResults(expected1, expected2, got1, got2);
                    if ((expected1 != null && got1 != expected1) || (expected2 != null && got2 != expected2))
                    {
                        exitCode = Math.Max(exitCode, 1);
                    }
                }
                else
                {
                    Console.WriteLine($"(no test file: {testPath})");
                }

                // input file
                string inputPath = Path.Combine("inputs", year.ToString(), dayId, "input.txt");
                if (File.Exists(inputPath))
                {
                    Console.WriteLine($"-- file: {inputPath}");
                    var (expected1, expected2, content) = LoadFileAndExpected(inputPath);
                    var (got1, got2) = solver.Solve(content);
                    ReportResults(expected1, expected2, got1, got2);
                    if ((expected1 != null && got1 != expected1) || (expected2 != null && got2 != expected2))
                    {
                        exitCode = Math.Max(exitCode, 1);
                    }
                }
                else
                {
                    Console.WriteLine($"(no input file: {inputPath})");
                }

                return exitCode;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"Exception while running {dayId} for {year}: {ex}");
                return 4;
            }
        }

        private (string? expected1, string? expected2, string content) LoadFileAndExpected(string path)
        {
            string? expected1 = null;
            string? expected2 = null;
            var lines = File.ReadAllLines(path);
            var contentLines = new List<string>();
            foreach (var l in lines)
            {
                if (l.StartsWith("result part 1:", StringComparison.OrdinalIgnoreCase))
                {
                    expected1 = l.Substring("result part 1:".Length).Trim();
                }
                else if (l.StartsWith("result part 2:", StringComparison.OrdinalIgnoreCase))
                {
                    expected2 = l.Substring("result part 2:".Length).Trim();
                }
                else
                {
                    contentLines.Add(l);
                }
            }
            return (expected1, expected2, string.Join(System.Environment.NewLine, contentLines));
        }

        private void ReportResults(string? expected1, string? expected2, string? got1, string? got2)
        {
            Console.WriteLine($"Part 1: {got1}");
            if (expected1 != null)
            {
                if (got1 == expected1)
                    Console.WriteLine($"PART 1 - OK (expected {expected1})");
                else
                    Console.WriteLine($"PART 1 - MISMATCH expected {expected1} actual {got1}");
            }

            Console.WriteLine($"Part 2: {got2}");
            if (expected2 != null)
            {
                if (got2 == expected2)
                    Console.WriteLine($"PART 2 - OK (expected {expected2})");
                else
                    Console.WriteLine($"PART 2 - MISMATCH expected {expected2} actual {got2}");
            }
        }
    }
}
