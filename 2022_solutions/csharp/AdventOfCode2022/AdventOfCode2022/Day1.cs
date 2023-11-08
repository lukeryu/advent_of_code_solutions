namespace AdventOfCode2022;

public class Day1
{
    private static Dictionary<ulong, IList<ulong>> ParseElfPacks(IList<string> data)
    {
        ulong elfCount = 1;
        var currentArray = new List<ulong>();
        var elfPacks = new Dictionary<ulong, IList<ulong>>();

        foreach (var datum in data)
        {
            if (datum.Length == 0)
            {
                elfPacks.Add(elfCount, currentArray);
                currentArray = new List<ulong>();
                elfCount += 1;
            }
            else
            {
                currentArray.Add(UInt64.Parse(datum));
            }
        }

        if (currentArray.Count > 0)
        {
            elfPacks.Add(elfCount, currentArray);
        }

        return elfPacks;
    }

    public static ulong Puzzle1(IList<string> input)
    {
        var elfPacks = ParseElfPacks(input);
        var values = new List<ulong>();

        foreach (var keyValuePair in elfPacks)
        {
            ulong sum = 0;
            foreach (var value in keyValuePair.Value)
            {
                sum += value;
            }

            values.Add(sum);
        }

        values.Sort();
        return values.Last();
    }

    public static ulong Puzzle2(IList<string> input)
    {
        var elfPacks = ParseElfPacks(input);
        var values = new List<ulong>();

        foreach (var keyValuePair in elfPacks)
        {
            ulong sum = 0;
            foreach (var value in keyValuePair.Value)
            {
                sum += value;
            }

            values.Add(sum);
        }

        values.Sort();
        values.Reverse();
        var subs = values.GetRange(0, 3);
        ulong finalSum = 0;

        foreach (var value in subs)
        {
            finalSum += value;
        }

        return finalSum;
    }
}