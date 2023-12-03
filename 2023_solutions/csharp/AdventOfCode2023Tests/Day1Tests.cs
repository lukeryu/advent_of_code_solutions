using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day1Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet"
        };
    }

    private static List<string> GetTestData2()
    {
        return new List<string>
        {
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen"
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day1.Puzzle1(GetTestData());
        Assert.That(result, Is.EqualTo(142));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day1.txt");
        var result = Day1.Puzzle1(data);
        Assert.That(result, Is.EqualTo(53386));
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day1.Puzzle2(GetTestData2());
        Assert.That(result, Is.EqualTo(281));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day1.txt");
        var result = Day1.Puzzle2(data);
        Assert.That(result, Is.EqualTo(53312));
    }
}