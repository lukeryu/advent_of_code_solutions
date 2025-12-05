using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day18Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "R 6 (#70c710)",
            "D 5 (#0dc571)",
            "L 2 (#5713f0)",
            "D 2 (#d2c081)",
            "R 2 (#59c680)",
            "D 2 (#411b91)",
            "L 5 (#8ceee2)",
            "U 2 (#caa173)",
            "L 1 (#1b58a2)",
            "U 2 (#caa171)",
            "R 2 (#7807d2)",
            "U 3 (#a77fa3)",
            "L 2 (#015232)",
            "U 2 (#7a21e3)"
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day18.Puzzle1(GetTestData());
        Assert.That(result, Is.EqualTo(62));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day18.txt");
        var result = Day18.Puzzle1(data);
        Assert.That(result, Is.EqualTo(7562));
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day18.Puzzle2(GetTestData());
        Assert.That(result, Is.EqualTo(51));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day18.txt");
        var result = Day18.Puzzle2(data);
        Assert.That(result, Is.EqualTo(7793));
    }
}