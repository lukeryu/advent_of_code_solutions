using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day3Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day3.Puzzle1(GetTestData());
        Assert.That(result, Is.EqualTo(4361));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day3.txt");
        var result = Day3.Puzzle1(data);
        Assert.That(result, Is.EqualTo(537832));
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day3.Puzzle2(GetTestData());
        Assert.That(result, Is.EqualTo(467835));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day3.txt");
        var result = Day3.Puzzle2(data);
        Assert.That(result, Is.EqualTo(81939900));
    }
}