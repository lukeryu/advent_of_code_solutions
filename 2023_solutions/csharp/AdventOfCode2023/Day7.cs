namespace AdventOfCode2023;

public static class Day7
{
    private enum HandType
    {
        FIVE_OF_A_KIND,
        FOUR_OF_A_KIND,
        FULL_HOUSE,
        THREE_OF_A_KIND,
        TWO_PAIR,
        ONE_PAIR,
        HIGH_CARD
        
    }

    private static HandType AddJokers(HandType handType, ulong jokers)
    {
        if (jokers == 0)
        {
            return handType;
        }
        if (jokers == 5 || jokers == 4)
        {
            return HandType.FIVE_OF_A_KIND;
        }

        if (jokers == 1)
        {
            switch (handType)
            {
                case HandType.FIVE_OF_A_KIND:
                    return HandType.FIVE_OF_A_KIND;
                case HandType.FOUR_OF_A_KIND:
                    return HandType.FIVE_OF_A_KIND;
                case HandType.FULL_HOUSE:
                    return HandType.FOUR_OF_A_KIND;
                case HandType.THREE_OF_A_KIND:
                    return HandType.FOUR_OF_A_KIND;
                case HandType.TWO_PAIR:
                    return HandType.FULL_HOUSE;
                case HandType.ONE_PAIR:
                    return HandType.THREE_OF_A_KIND;
                case HandType.HIGH_CARD:
                    return HandType.ONE_PAIR;
            }
        }
        
        if (jokers == 2)
        {
            switch (handType)
            {
                case HandType.THREE_OF_A_KIND:
                    return HandType.FIVE_OF_A_KIND;
                case HandType.ONE_PAIR:
                    return HandType.FOUR_OF_A_KIND;
                case HandType.HIGH_CARD:
                    return HandType.THREE_OF_A_KIND;
            }
        }
        
        if (jokers == 3)
        {
            switch (handType)
            {
                case HandType.ONE_PAIR:
                    return HandType.FIVE_OF_A_KIND;
                case HandType.HIGH_CARD:
                    return HandType.FOUR_OF_A_KIND;
            }
        }

        return handType;
    }

    private record HandAndBid(string Hand, ulong Bid, HandType Type) : IComparable<HandAndBid>
    {
        public int CompareTo(HandAndBid? other)
        {
            if (other == null)
            {
                return 1;
            }
            var val = Type - other.Type;
            if (val != 0)
            {
                return val;
            }

            return other.Hand.CompareTo(Hand);
        }
    }

    public static ulong Puzzle1(List<string> inputs)
    {
        var allHands = ParseAllHands(inputs);

        ulong sum = 0;

        ulong index = (ulong) allHands.Count;
        foreach (var hands in allHands)
        {
            sum += index * hands.Bid;
            index--;
        }

        return sum;
    }

    private static SortedSet<HandAndBid> ParseAllHands(List<string> inputs)
    {
        var handAndBids = new SortedSet<HandAndBid>();

        foreach (var parts in inputs.Select(instruction => instruction.Split(" ")))
        {
            var part = parts[0].Replace("A", "E").Replace("K", "D").Replace("Q", "C").Replace("J", "B")
                .Replace("T", "A");
            handAndBids.Add(new HandAndBid(part, ulong.Parse(parts[1]), DetermineHandType(part)));
        }

        return handAndBids;
    }

    private static HandType DetermineHandType(string part)
    {
        var characterCounts = new Dictionary<char, int>();

        foreach (var character in part)
        {
            if (!characterCounts.TryAdd(character, 1))
            {
                characterCounts[character]++;
            }
        }

        if (characterCounts.Values.Any((value) => value == 5))
        {
            return HandType.FIVE_OF_A_KIND;
        }

        if (characterCounts.Values.Any((value) => value == 4))
        {
            return HandType.FOUR_OF_A_KIND;
        }

        if (characterCounts.Values.Any((value) => value == 3))
        {
            if (characterCounts.Values.Any((value) => value == 2))
            {
                return HandType.FULL_HOUSE;
            }

            return HandType.THREE_OF_A_KIND;
        }

        var pairs = characterCounts.Values.Count((value) => value == 2);

        if (pairs == 2)
        {
            return HandType.TWO_PAIR;
        }

        if (pairs == 1)
        {
            return HandType.ONE_PAIR;
        }

        return HandType.HIGH_CARD;
    }

    public static ulong Puzzle2(List<string> inputs)
    {
        var allHands = ParseAllHands2(inputs);

        ulong sum = 0;

        ulong index = (ulong) allHands.Count;
        foreach (var hands in allHands)
        {
            sum += index * hands.Bid;
            index--;
        }

        return sum;
    }
    
    private static SortedSet<HandAndBid> ParseAllHands2(List<string> inputs)
    {
        var handAndBids = new SortedSet<HandAndBid>();

        foreach (var parts in inputs.Select(instruction => instruction.Split(" ")))
        {
            var part = parts[0].Replace("A", "E").Replace("K", "D").Replace("Q", "C").Replace("J", "0")
                .Replace("T", "A");
            handAndBids.Add(new HandAndBid(part, ulong.Parse(parts[1]), DetermineHandType2(part)));
        }

        return handAndBids;
    }

    private static HandType DetermineHandType2(string part)
    {
        var characterCounts = new Dictionary<char, int>();

        var jokerCount = (ulong) part.Count((c) => c == '0');
        
        foreach (var character in part.Replace("0", ""))
        {
            if (!characterCounts.TryAdd(character, 1))
            {
                characterCounts[character]++;
            }
        }

        if (characterCounts.Values.Any((value) => value == 5))
        {
            return AddJokers(HandType.FIVE_OF_A_KIND, jokerCount);
        }

        if (characterCounts.Values.Any((value) => value == 4))
        {
            return AddJokers(HandType.FOUR_OF_A_KIND, jokerCount);
        }

        if (characterCounts.Values.Any((value) => value == 3))
        {
            if (characterCounts.Values.Any((value) => value == 2))
            {
                return AddJokers(HandType.FULL_HOUSE, jokerCount);
            }

            return AddJokers(HandType.THREE_OF_A_KIND, jokerCount);
        }

        var pairs = characterCounts.Values.Count((value) => value == 2);

        if (pairs == 2)
        {
            return AddJokers(HandType.TWO_PAIR, jokerCount);
        }

        if (pairs == 1)
        {
            return AddJokers(HandType.ONE_PAIR, jokerCount);
        }

        return AddJokers(HandType.HIGH_CARD, jokerCount);
    }
}