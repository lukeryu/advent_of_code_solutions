using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day17Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "2413432311323",
            "3215453535623",
            "3255245654254",
            "3446585845452",
            "4546657867536",
            "1438598798454",
            "4457876987766",
            "3637877979653",
            "4654967986887",
            "4564679986453",
            "1224686865563",
            "2546548887735",
            "4322674655533"
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day17.Puzzle1(GetTestData());
        Assert.That(result, Is.EqualTo(102));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day17.txt");
        var result = Day17.Puzzle1(data);
        Assert.That(result, Is.EqualTo(7562));
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day17.Puzzle2(GetTestData());
        Assert.That(result, Is.EqualTo(51));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day17.txt");
        var result = Day17.Puzzle2(data);
        Assert.That(result, Is.EqualTo(7793));
    }
}