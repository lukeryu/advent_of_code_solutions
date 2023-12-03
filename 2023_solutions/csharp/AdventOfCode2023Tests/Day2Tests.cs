using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day2Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day2.Puzzle1(GetTestData());
        Assert.That(result, Is.EqualTo(8));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day2.txt");
        var result = Day2.Puzzle1(data);
        Assert.That(result, Is.EqualTo(2176));
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day2.Puzzle2(GetTestData());
        Assert.That(result, Is.EqualTo(2286));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day2.txt");
        var result = Day2.Puzzle2(data);
        Assert.That(result, Is.EqualTo(63700));
    }
}