using System.Text;

namespace AdventOfCode2023;

public static class Day12
{
    private class InputRow
    {
        public InputRow(string conditionRow, List<int> groupRow)
        {
            ConditionRow = conditionRow;
            GroupRow = groupRow;
        }

        private string ConditionRow { get; }
        private List<int> GroupRow { get; }

        public override bool Equals(object? o)
        {
            if (o is InputRow input)
            {
                return input.ConditionRow.Equals(ConditionRow) && input.GroupRow.SequenceEqual(GroupRow);
            }

            return false;
        }

        public override int GetHashCode()
        {
            return ConditionRow.GetHashCode() * 37 + GroupRow.Sum(intValue => (intValue * 37));
        }
    };

    public static long Puzzle1(IEnumerable<string> input)
    {
        Dictionary<InputRow, long> previouslyCalculations = new();
        long max = 0;
        foreach (var line in input) {
            var parts = line.Split(" ");
            var record = parts[0];
            var groups = parts[1].Split(",").Select(int.Parse).ToList();
            max += CountPermutations(record, groups, previouslyCalculations);
        }
        return max;
    }

    public static long Puzzle2(List<string> input)
    {
        Dictionary<InputRow, long> precalculatedStrings = new();
        long max = 0;
        foreach (var line in input) {
            var parts = line.Split(" ");
            var record = UnfoldCondition(parts[0], 5);
            var groups = UnfoldGroups(parts[1].Split(",").Select(int.Parse).ToList(), 5);
            var permutations = CountPermutations(record, groups, precalculatedStrings);
            max += permutations;
        }

        return max;
    }

    private static String UnfoldCondition(string condition, int times) {
        var sb = new StringBuilder();
        for (var i = 0; i < times - 1; i++) {
            sb.Append(condition);
            sb.Append('?');
        }
        sb.Append(condition);
        return sb.ToString();
    }

    private static List<int> UnfoldGroups(List<int> groups, int times) {
        var unfoldedGroups = new List<int>();
        for (var i = 0; i < times; i++) {
            unfoldedGroups.AddRange(groups);
        }
        return unfoldedGroups;
    }
    
    private static long CountPermutations(string rowCondition, List<int> rowGroupCounts, 
        IDictionary<InputRow, long> previousCalculations) {
        var input = new InputRow(rowCondition, rowGroupCounts);
        if (previousCalculations.TryGetValue(input, out var value)) {
            return value;
        }

        if (rowCondition.Trim().Length == 0) {
            return !rowGroupCounts.Any() ? 1 : 0;
        }

        var firstChar = rowCondition[0];
        long permutations = 0;
        if (firstChar == '.') {
            permutations = CountPermutations(rowCondition.Substring(1), rowGroupCounts, previousCalculations);
        } else if (firstChar == '?') {
            permutations = CountPermutations("." + rowCondition.Substring(1), rowGroupCounts, previousCalculations)
                           + CountPermutations("#" + rowCondition.Substring(1), rowGroupCounts, previousCalculations);
        } else {
            if (rowGroupCounts.Count == 0) {
                permutations = 0;
            } else {
                var nrDamaged = rowGroupCounts[0];
                if (nrDamaged <= rowCondition.Length
                        && rowCondition.ToCharArray().Take(nrDamaged).All(c => c == '#' || c == '?')) {
                    List<int> newGroups = rowGroupCounts.GetRange(1, rowGroupCounts.Count - 1);
                    if (nrDamaged == rowCondition.Length) {
                        permutations = !newGroups.Any() ? 1 : 0;
                    } else if (rowCondition[nrDamaged] == '.') {
                        permutations = CountPermutations(rowCondition.Substring(nrDamaged + 1), newGroups, previousCalculations);
                    } else if (rowCondition[nrDamaged] == '?') {
                        permutations = CountPermutations("." + rowCondition.Substring(nrDamaged + 1), newGroups, previousCalculations);
                    } else {
                        permutations = 0;
                    }
                } else {
                    permutations = 0;
                }
            }
        }
        previousCalculations.Add(input, permutations);
        return permutations;
    }

}