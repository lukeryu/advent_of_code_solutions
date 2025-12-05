using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day24Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "19, 13, 30 @ -2,  1, -2",
            "18, 19, 22 @ -1, -1, -2",
            "20, 25, 34 @ -2, -2, -4",
            "12, 31, 28 @ -1, -2, -1",
            "20, 19, 15 @  1, -5, -3"
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day24.Puzzle1(GetTestData(), 7, 27);
        Assert.That(result, Is.EqualTo(2));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day24.txt");
        var result = Day24.Puzzle1(data, 200000000000000, 400000000000000);
        Assert.That(result, Is.EqualTo(7562));
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day24.Puzzle2(GetTestData());
        Assert.That(result, Is.EqualTo(51));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day24.txt");
        var result = Day24.Puzzle2(data);
        Assert.That(result, Is.EqualTo(7793));
    }
}