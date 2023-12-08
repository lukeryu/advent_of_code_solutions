using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day8Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)"
        };
    }

    private static List<string> GetTestData2()
    {
        return new List<string>
        {
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)"
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day8.Puzzle1(GetTestData());
        Assert.That(result, Is.EqualTo(6));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day8.txt");
        var result = Day8.Puzzle1(data);
        Assert.That(result, Is.EqualTo(13771));
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day8.Puzzle2(GetTestData2());
        Assert.That(result, Is.EqualTo(6));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day8.txt");
        var result = Day8.Puzzle2(data);
        Assert.That(result, Is.EqualTo(13129439557681));
    }
}