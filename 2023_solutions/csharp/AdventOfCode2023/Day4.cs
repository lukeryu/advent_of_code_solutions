using System.Text.RegularExpressions;

namespace AdventOfCode2023;

public static class Day4
{
    private record Card(int CardNumber, int MatchCount);

    public static ulong Puzzle1(List<string> inputLines)
    {
        var cards = ParseCards(inputLines);

        ulong sum = 0;
        foreach (var card in cards)
        {
            if (card.MatchCount > 0)
            {
                sum += (ulong)Pow(2, card.MatchCount);
            }
        }

        return sum;
    }

    private static List<Card> ParseCards(List<string> inputLines)
    {
        var cardList = new List<Card>();
        foreach (var card in inputLines)
        {
            var headerAndBody = card.Split(":");
            var header = headerAndBody[0];
            var body = headerAndBody[1];

            var numbersAndWinners = body.Split("|");
            var winners = numbersAndWinners[0];
            var numbers = numbersAndWinners[1];

            var parsedWinners = ParseNumbers(winners);
            var parsedNumbers = ParseNumbers(numbers);

            int matchCount = parsedNumbers.Count(number => parsedWinners.Contains(number));

            var cardNumber = int.Parse(Regex.Matches(header, @"(\d)+")[0].Captures[0].Value);
            cardList.Add(new Card(cardNumber, matchCount));
        }

        return cardList;
    }

    private static int Pow(int baseNum, int power)
    {
        int val = 1;

        for (var num = 1; num < power; num++)
        {
            val *= baseNum;
        }

        return val;
    }


    private static SortedSet<int> ParseNumbers(string numbers)
    {
        var sortedSet = new SortedSet<int>();
        var matches = Regex.Matches(numbers, @"(\d)+");
        foreach (Match match in matches)
        {
            sortedSet.Add(int.Parse(match.Captures[0].Value));
        }

        return sortedSet;
    }

    public static ulong Puzzle2(List<string> inputLines)
    {
        var cards = ParseCards(inputLines);
        var map = new Dictionary<int, int>();

        foreach (var card in cards)
        {
            map[card.CardNumber] = 0;
        }

        foreach (var card in cards)
        {
            map[card.CardNumber]++;
            var value = map[card.CardNumber];
            for (var i = 1; i <= card.MatchCount; i++)
            {
                map[card.CardNumber + i] += value;
            }
        }

        return (ulong) map.Values.Sum();
    }
}