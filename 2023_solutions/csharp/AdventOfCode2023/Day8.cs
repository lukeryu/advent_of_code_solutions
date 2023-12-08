using System.Text.RegularExpressions;

namespace AdventOfCode2023;

public static class Day8
{
    private record Node(string Name, string Lhs, string Rhs);

    public delegate bool MatchesEndState(string x);

    public static ulong Puzzle1(List<string> inputs)
    {
        var stepPattern = inputs[0];

        var dictionary = ParseDictionary(inputs.Skip(2));
        var startingNodeName = "AAA";

        return RunIterations(stepPattern, dictionary, startingNodeName, (value) => value.Equals("ZZZ"));
    }

    private static ulong RunIterations(string stepPattern, Dictionary<string, Node> dictionary, string startingNodeName,
        MatchesEndState endState)
    {
        ulong stepCount = 0;
        bool foundZZZ = false;
        int stepPatternIndex = 0;
        string currentNodeName = startingNodeName;
        while (!foundZZZ)
        {
            var stepDirection = stepPattern[stepPatternIndex];
            var currentNode = dictionary[currentNodeName];

            if (stepDirection.Equals('R'))
            {
                currentNodeName = currentNode.Rhs;
            }
            else
            {
                currentNodeName = currentNode.Lhs;
            }

            stepCount++;
            stepPatternIndex++;
            if (stepPatternIndex >= stepPattern.Length)
            {
                stepPatternIndex = 0;
            }

            if (endState(currentNodeName))
            {
                foundZZZ = true;
            }
        }

        return stepCount;
    }

    private static Dictionary<string, Node> ParseDictionary(IEnumerable<string> skip)
    {
        var dictionary = new Dictionary<string, Node>();

        foreach (var str in skip)
        {
            var matches = Regex.Matches(str, @"^(\w{3}) = \((\w{3}), (\w{3})\)$");
            var name = matches[0].Groups[1].Value;
            var lhs = matches[0].Groups[2].Value;
            var rhs = matches[0].Groups[3].Value;

            dictionary.Add(name, new Node(name, lhs, rhs));
        }

        return dictionary;
    }

    public static ulong Puzzle2(List<string> inputs)
    {
        var stepPattern = inputs[0];

        var dictionary = ParseDictionary(inputs.Skip(2));

        var currentNodes = dictionary.Keys.Where((key) => key[^1] == 'A');

        var endingStepValues = currentNodes.Select(node => RunIterations(stepPattern, dictionary, node, (key) => key[^1] == 'Z')).ToList();

        ulong asdf = 1;

        foreach (var value in endingStepValues)
        {
            asdf = lcm(asdf, value);
        }

        return asdf;
    }
    
    static ulong gcf(ulong a, ulong b)
    {
        while (b != 0)
        {
            ulong temp = b;
            b = a % b;
            a = temp;
        }
        return a;
    }

    static ulong lcm(ulong a, ulong b)
    {
        return (a / gcf(a, b)) * b;
    }
}