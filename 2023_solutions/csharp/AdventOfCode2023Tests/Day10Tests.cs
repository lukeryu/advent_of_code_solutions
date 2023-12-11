using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day10Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "..F7.",
            ".FJ|.",
            "SJ.L7",
            "|F--J",
            "LJ..."
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day10.Puzzle1(GetTestData(), Day10.PipeType.F);
        Assert.That(result, Is.EqualTo(8));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day10.txt");
        var result = Day10.Puzzle1(data, Day10.PipeType.Horizontal);
        Assert.That(result, Is.EqualTo(1)); //7206 too high
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day10.Puzzle2(GetTestData());
        Assert.That(result, Is.EqualTo(2));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day10.txt");
        var result = Day10.Puzzle2(data);
        Assert.That(result, Is.EqualTo(1072));
    }
}