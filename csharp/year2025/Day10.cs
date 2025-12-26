using adventofcode.utils;

namespace adventofcode.year2025;

public class Day10 : ISolver
{
    private long _part1;
    private long _part2;

    public void Parse(string input)
    {
        var machine = new Machine(input);
        _part1 += SwitchLights(machine);
    }

    private static long SwitchLights(Machine machine)
    {
        // "randomly" click buttons to see if we can find the right configuration
        var minClicks = int.MaxValue;
        var queue = new Queue<(int, char[])>();
        queue.Enqueue((1, Enumerable.Repeat('.', machine.Lights.Length).ToArray()));
        while (queue.Count != 0)
        {
            var (clicks, status) = queue.Dequeue();
            if (clicks >= minClicks)
            {
                // cull if we found a solution already and we have done the same clicks 
                continue;
            } 
            foreach (var buttons in machine.Buttons)
            {
                // flip .<-># in status at positions indicated by buttons
                var newStatus = status[..];
                foreach (var pos in buttons)
                {
                    newStatus[pos] = status[pos] == '.' ? '#' : '.';
                }

                var lights = new string(newStatus);
                if (machine.Lights == lights)
                {
                    minClicks = Math.Min(minClicks, clicks);
                }
                else
                {
                    queue.Enqueue((clicks + 1, newStatus));
                }
            }
        }
        Log.Debug($"Matched {machine.Lights} in {minClicks} clicks");
        return minClicks;
    }

    public (string? part1, string? part2) Solve()
    {
        return ($"{_part1}", $"{_part2}");
    }

    private class Machine
    {
        public readonly string Lights;
        public readonly string Joltage;
        public readonly List<int[]> Buttons = [];
        public Machine(string input)
        {
            if (!input.StartsWith('[') || !input.Contains(']'))
            {
                throw new Exception($"Invalid machine config - wrong lights configs: {input}");
            }
            var lightsEnd = input.IndexOf(']');
            Lights = input[1..lightsEnd];
            var joltageStart = input.IndexOf('{');
            if (joltageStart == -1 || !input.EndsWith('}'))
            {
                throw new Exception($"Invalid machine config - wrong joltage config: {input}");
            }
            Joltage = input[(joltageStart + 1)..^1];
            foreach (var block in input[(lightsEnd + 2)..(joltageStart - 1)].Split(" "))
            {
                Buttons.Add(block[1..^1].Split(",").Select(int.Parse).ToArray());
            }
        }
    }
}