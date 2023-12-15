using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day14Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#...."
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day14.Puzzle1(GetTestData());
        Assert.That(result, Is.EqualTo(136));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day14.txt");
        var result = Day14.Puzzle1(data);
        Assert.That(result, Is.EqualTo(110779)); 
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day14.Puzzle2(GetTestData());
        Assert.That(result, Is.EqualTo(64));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day14.txt");
        var result = Day14.Puzzle2(data);
        Assert.That(result, Is.EqualTo(280382734828319));
    }
}