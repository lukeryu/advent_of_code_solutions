using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day7Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483"
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day7.Puzzle1(GetTestData());
        Assert.That(result, Is.EqualTo(6440));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day7.txt");
        var result = Day7.Puzzle1(data);
        Assert.That(result, Is.EqualTo(251216224));
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day7.Puzzle2(GetTestData());
        Assert.That(result, Is.EqualTo(5905));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day7.txt");
        var result = Day7.Puzzle2(data);
        Assert.That(result, Is.EqualTo(250825971));
    }
}