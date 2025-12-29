namespace adventofcode.year2025;

public class Day06 : ISolver
{
    private long _part1;
    private long _part2;
    private readonly List<string> _numbers = [];
    private int _length;

    public void Parse(string input)
    {
        if (_length == 0)
        {
            _length = input.Length;
        }
        else if (input.Length != _length)
        {
            throw new Exception("Not all input has same length - check if it wasn't trimmed");
        }

        if (input[0] != '*' && input[0] != '+')
        {
            _numbers.Add(input);
        }
        else
        {
            CalculatePart1(input);
            CalculatePart2(input);
        }
    }

    private void CalculatePart1(string ops)
    {
        var converted = _numbers
            .Select(n => n.Split(' ', StringSplitOptions.RemoveEmptyEntries).Select(int.Parse).ToArray())
            .ToArray();
        var idx = 0;
        foreach (var op in ops.Split(' ', StringSplitOptions.RemoveEmptyEntries))
        {
            var accum = op == "*" ? 1L : 0L;
            foreach (var val in converted)
            {
                switch (op)
                {
                    case "+":
                        accum += val[idx];
                        break;
                    case "*":
                        accum *= val[idx];
                        break;
                }
            }
            _part1 += accum;
            idx++;
        }
    }

    private void CalculatePart2(string ops)
    {
        // go in reverse from the end looking for space and execute each operation
        var p = ops.Length - 1;
        var q = ops.Length - 1;
        while (true)
        {
            if (q < 0)
            {
                break;
            }
            // find the previous space
            if (ops[q] == ' ')
            {
                q--;
                continue;
            }

            // ok, we have found the start, execute the operation
            _part2 += CephalopodOp(ops, q, p);
            // move back 2 spots
            q = p = q - 2;
        }
    }

    private long CephalopodOp(string ops, int start, int end)
    {
        List<long> factors = [];
        for (var i = end; i >= start; i--)
        {
            var num = _numbers.Where(val => val[i] != ' ')
                .Aggregate(0L, (current, val) => current * 10 + (val[i] - '0'));
            factors.Add(num);
        }

        return ops[start] == '+' ? factors.Sum() : factors.Aggregate(1L, (a, b) => a * b);
    }

    public (string? part1, string? part2) Solve()
    {
        return ($"{_part1}", $"{_part2}");
    }
}