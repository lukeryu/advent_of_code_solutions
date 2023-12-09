namespace AdventOfCode2023;

public static class Day9
{
    private delegate T GetExtrapolateValue<T>(LinkedList<T> list);

    private delegate void AddExtrapolateValue<T>(LinkedList<T> list, T valueToAdd);

    private delegate T CalculateExtrapolateValue<T>(T lhs, T rhs);

    public static long Puzzle1(List<string> inputs)
    {
        GetExtrapolateValue<long> getValue = (list) => list.Last!.Value;
        AddExtrapolateValue<long> addValue = (list, valueToAdd) => list.AddLast(valueToAdd);
        CalculateExtrapolateValue<long> calcValue = (lhs, rhs) => lhs + rhs;
        return inputs.Sum(input => Extrapolate(input, getValue, addValue, calcValue));
    }

    public static long Puzzle2(List<string> inputs)
    {
        GetExtrapolateValue<long> getValue = (list) => list.First!.Value;
        AddExtrapolateValue<long> addValue = (list, value) => list.AddFirst(value);
        CalculateExtrapolateValue<long> calcValue = (lhs, rhs) => lhs - rhs;
        return inputs.Sum(input => Extrapolate(input, getValue, addValue, calcValue));
    }

    private static long Extrapolate(string input, GetExtrapolateValue<long> getValue,
        AddExtrapolateValue<long> addValue, CalculateExtrapolateValue<long> calculateExtrapolateValue)
    {
        List<LinkedList<long>> rows = new();
        var values = new LinkedList<long>(input.Split(' ').Select(long.Parse).ToList());
        rows.Add(values);

        while (true)
        {
            LinkedList<long> currentRow = new();

            var previousRow = rows[^1];
            for (var elementIndex = 1; elementIndex < previousRow.Count; elementIndex++)
            {
                currentRow.AddLast(previousRow.ElementAt(elementIndex) - previousRow.ElementAt(elementIndex - 1));
            }

            rows.Add(currentRow);

            if (currentRow.All((value) => value == 0))
            {
                break;
            }
        }

        rows[^1].AddLast(0);

        for (var rowIndex = rows.Count - 2; rowIndex >= 0; rowIndex--)
        {
            var currentRow = rows[rowIndex];
            var previousRow = rows[rowIndex + 1];

            var currentValue = getValue(currentRow);
            var previousValue = getValue(previousRow);

            addValue(currentRow, calculateExtrapolateValue(currentValue, previousValue));
        }

        return getValue(rows[0]);
    }
}