using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day11Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#....."
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day11.Puzzle1(GetTestData(), 1);
        Assert.That(result, Is.EqualTo(374));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day11.txt");
        var result = Day11.Puzzle1(data, 1);
        Assert.That(result, Is.EqualTo(9370588));
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day11.Puzzle1(GetTestData(), 9);
        Assert.That(result, Is.EqualTo(1030));
    }
    
    [Test]
    public void Puzzle3TestData()
    {
        var result = Day11.Puzzle1(GetTestData(), 99);
        Assert.That(result, Is.EqualTo(8410));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day11.txt");
        var result = Day11.Puzzle1(data, 999_999);
        Assert.That(result, Is.EqualTo(746207878188));
    }
}