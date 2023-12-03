using System.Text;
using System.Text.RegularExpressions;

namespace AdventOfCode2023;

public static partial class Day1
{
    public static ulong Puzzle1(List<string> input)
    {
        ulong sum = 0;

        foreach (var data in input)
        {
            char? firstDigit = null;
            char? lastDigit = null;

            foreach (var character in data.Where(char.IsDigit))
            {
                firstDigit ??= character;
                lastDigit = character;
            }

            var firstValue = firstDigit - '0';
            var lastValue = lastDigit - '0';
            if (firstValue.HasValue && lastValue.HasValue)
            {
                var rowValue = (ulong)(firstValue.Value * 10) + (ulong)lastValue.Value;
                sum += rowValue;
            }
        }

        return sum;
    }

    public static ulong Puzzle2(IEnumerable<string> input)
    {
        var transformedStrings = input.Select(TransformWordsToNumbers).ToList();

        return Puzzle1(transformedStrings);
    }

    private static string TransformWordsToNumbers(string data)
    {
        var replacedString = MyRegex().Replace(data, " ");
        replacedString = ReplaceStringWithDigit(data, replacedString, "one", '1');
        replacedString = ReplaceStringWithDigit(data, replacedString, "two", '2');
        replacedString = ReplaceStringWithDigit(data, replacedString, "three", '3');
        replacedString = ReplaceStringWithDigit(data, replacedString, "four", '4');
        replacedString = ReplaceStringWithDigit(data, replacedString, "five", '5');
        replacedString = ReplaceStringWithDigit(data, replacedString, "six", '6');
        replacedString = ReplaceStringWithDigit(data, replacedString, "seven", '7');
        replacedString = ReplaceStringWithDigit(data, replacedString, "eight", '8');
        replacedString = ReplaceStringWithDigit(data, replacedString, "nine", '9');

        return replacedString;
    }

    private static string ReplaceStringWithDigit(string data, string replacedString, string stringWord,
        char stringDigit)
    {
        var replaceString = new StringBuilder(replacedString);
        var matches = Regex.Matches(data, stringWord);
        foreach (Match match in matches)
        {
            var index = match.Index;
            replaceString[index] = stringDigit;
        }

        return replaceString.ToString();
    }

    [GeneratedRegex("[a-z]")]
    private static partial Regex MyRegex();
}