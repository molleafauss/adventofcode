using System;

namespace adventofcode
{
    // Simple Advent of Code C# runner
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
                    return solveAll(year);
                }

                if (!dayArg.StartsWith("day"))
                {
                    Console.Error.WriteLine("Invalid day argument (e.g. day03 or all)");
                    return -1;
                }

                int dayNum = int.Parse(dayArg);
                return solveDay(year, dayNum);
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine("Unhandled error: " + ex);
                return -1;
            }
        }

        private static int solveDay(int year, int dayNum)
        {
            throw new NotImplementedException();
        }

        private static int solveAll(int year)
        {
            throw new NotImplementedException();
        }

        static void PrintHelp()
        {
            Console.WriteLine("Usage: aoc <dayNN|all> [--year <YYYY>]");
            Console.WriteLine();
            Console.WriteLine("day   : dayNN (e.g. day03) solve specific day");
            Console.WriteLine("all   : solve all available days");
            Console.WriteLine("--year, -y : optional year (defaults to most recent AoC year based on current date)");
        }
    }
}
