using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day9Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "0 3 6 9 12 15",
            "1 3 6 10 15 21",
            "10 13 16 21 30 45"
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day9.Puzzle1(GetTestData());
        Assert.That(result, Is.EqualTo(114));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day9.txt");
        var result = Day9.Puzzle1(data);
        Assert.That(result, Is.EqualTo(2075724761));
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day9.Puzzle2(GetTestData());
        Assert.That(result, Is.EqualTo(2));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day9.txt");
        var result = Day9.Puzzle2(data);
        Assert.That(result, Is.EqualTo(1072));
    }
}