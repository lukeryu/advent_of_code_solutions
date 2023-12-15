using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day13Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
            "",
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#"
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day13.Puzzle1(GetTestData());
        Assert.That(result, Is.EqualTo(405));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day13.txt");
        var result = Day13.Puzzle1(data);
        Assert.That(result, Is.EqualTo(7753)); // 30379 too low
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day13.Puzzle2(GetTestData());
        Assert.That(result, Is.EqualTo(525152));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day13.txt");
        var result = Day13.Puzzle2(data);
        Assert.That(result, Is.EqualTo(280382734828319));
    }
}