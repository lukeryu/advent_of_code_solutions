namespace AdventOfCode2023;

public static class Day9
{
    public static long Puzzle1(List<string> inputs)
    {
        long sum = 0;
        foreach (var input in inputs)
        {
            sum += Extrapolate(input);
        }

        return sum;
    }

    private static long Extrapolate(string input)
    {
        List<List<long>> rows = new();
        var values = input.Split(' ').Select(long.Parse).ToList();
        rows.Add(values);

        bool allZeros = false;
        while (!allZeros)
        {
            List<long> currentRow = new();

            var previousRow = rows[^1];
            for (int i = 1; i < previousRow.Count; i++)
            {
                currentRow.Add(previousRow[i] - previousRow[i - 1]);
            }

            rows.Add(currentRow);

            if (currentRow.All((value) => value == 0))
            {
                allZeros = true;
            }
        }

        rows[^1].Add( 0);

        for (int i = rows.Count - 2; i >= 0; i--)
        {
            var currentRow = rows[i];
            var previousRow = rows[i + 1];

            var currentValue = currentRow[^1];
            var previousValue = previousRow[^1];
            
            currentRow.Add(currentValue + previousValue);
        }

        return rows[0][^1];
    }

    public static long Puzzle2(List<string> inputs)
    {
        long sum = 0;
        foreach (var input in inputs)
        {
            sum += ExtrapolateBackwards(input);
        }

        return sum;
    }
    
    private static long ExtrapolateBackwards(string input)
    {
        List<LinkedList<long>> rows = new();
        var values = new LinkedList<long>(input.Split(' ').Select(long.Parse).ToList());
        rows.Add(values);

        bool allZeros = false;
        while (!allZeros)
        {
            LinkedList<long> currentRow = new();

            var previousRow = rows[^1];
            for (int i = 1; i < previousRow.Count; i++)
            {
                long value = previousRow.ElementAt(i) - previousRow.ElementAt(i - 1);
                currentRow.AddLast(value);
            }

            rows.Add(currentRow);

            if (currentRow.All((value) => value == 0))
            {
                allZeros = true;
            }
        }

        rows[^1].AddFirst( 0);

        for (int i = rows.Count - 2; i >= 0; i--)
        {
            var currentRow = rows[i];
            var previousRow = rows[i + 1];

            var currentValue = currentRow.First.Value;
            var previousValue = previousRow.First.Value;
            
            currentRow.AddFirst(currentValue - previousValue);
        }

        return rows[0].First.Value;
    }
}