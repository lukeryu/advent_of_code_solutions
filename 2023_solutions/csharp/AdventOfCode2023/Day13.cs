using System.Text;

namespace AdventOfCode2023;

public static class Day13
{
    private static List<List<string>> ParseGraphs(List<string> list)
    {
        List<int> breaks = new List<int>();

        int index = 0;
        foreach (var line in list)
        {
            if (line.Length == 0)
            {
                breaks.Add(index);
            }

            index++;
        }

        breaks.Add(list.Count);


        List<List<string>> graphs = new();

        int indexStart = 0;

        foreach (var br in breaks)
        {
            graphs.Add(list.GetRange(indexStart, br - indexStart));
            indexStart = br + 1;
        }

        return graphs;
    }

    private static int FindRowMatch(List<string> graph)
    {
        int matchIndex = 0;
        for (var rowIndex = 0; rowIndex < graph.Count - 1; rowIndex++)
        {
            var currentRow = graph[rowIndex];
            var previousRow = graph[rowIndex + 1];
            if (previousRow.Equals(currentRow))
            {
                matchIndex = rowIndex;
            }
        }

        int index = 0;
        while (index <= matchIndex && (matchIndex + index + 1) < graph.Count)
        {
            var i = matchIndex - index;
            var lhsRow = graph[i];
            var j = matchIndex + 1 + index;
            var rhsRow = graph[j];

            if (!lhsRow.Equals(rhsRow))
            {
                return 0;
            }

            index++;
        }

        return matchIndex + 1;
    }

    public static ulong Puzzle1(List<string> inputs)
    {
        var graphs = ParseGraphs(inputs);

        ulong leftColumns = 0;
        ulong aboveRows = 0;

        foreach (var graph in graphs)
        {
            aboveRows += (ulong)FindRowMatch(graph);

            var rotateGraphCounterClockwise = RotateGraphCounterClockwise(graph);
            leftColumns += (ulong)FindRowMatch(rotateGraphCounterClockwise);
        }

        return aboveRows * 100 + leftColumns;
    }

    private static List<string> RotateGraphCounterClockwise(List<string> graph)
    {
        var sbList = new List<StringBuilder>();

        for (var i = 0; i < graph[0].Length; i++)
        {
            sbList.Add(new StringBuilder());
        }

        for (var row = 0; row < graph[0].Length; row++)
        {
            for (var graphIndex = graph.Count - 1; graphIndex >= 0; graphIndex--)
            {
                var value = graph[graphIndex][row];
                sbList[row].Append(value);
            }
        }

        return sbList.Select(sb => sb.ToString()).ToList();
    }

    public static ulong Puzzle2(List<string> inputs)
    {
        return 0;
    }
}