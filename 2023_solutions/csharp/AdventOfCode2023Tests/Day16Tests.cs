using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day16Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            ".|...\\....",
            "|.-.\\.....",
            ".....|-...",
            "........|.",
            "..........",
            ".........\\",
            "..../.\\\\..",
            ".-.-/..|..",
            ".|....-|.\\",
            "..//.|...."
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day16.Puzzle1(GetTestData());
        Assert.That(result, Is.EqualTo(46));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day16.txt");
        var result = Day16.Puzzle1(data);
        Assert.That(result, Is.EqualTo(7562));
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day16.Puzzle2(GetTestData());
        Assert.That(result, Is.EqualTo(51));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day16.txt");
        var result = Day16.Puzzle2(data);
        Assert.That(result, Is.EqualTo(7793));
    }
}