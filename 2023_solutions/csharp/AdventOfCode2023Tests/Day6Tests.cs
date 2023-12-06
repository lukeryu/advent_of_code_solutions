using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day6Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "Time:      7  15   30",
            "Distance:  9  40  200"
        };
    }

    private static List<string> GetRealData()
    {
        return new List<string>
        {
            "Time:        61     67     75     71",
            "Distance:   430   1036   1307   1150"
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day6.Puzzle1(GetTestData());
        Assert.That(result, Is.EqualTo(288));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = GetRealData();
        var result = Day6.Puzzle1(data);
        Assert.That(result, Is.EqualTo(316800));
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day6.Puzzle2(GetTestData());
        Assert.That(result, Is.EqualTo(71503));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = GetRealData();
        var result = Day6.Puzzle2(data);
        Assert.That(result, Is.EqualTo(45647654));
    }
}