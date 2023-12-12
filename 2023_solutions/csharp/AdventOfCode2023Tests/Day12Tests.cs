using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day12Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "???.### 1,1,3",
            ".??..??...?##. 1,1,3",
            "?#?#?#?#?#?#?#? 1,3,1,6",
            "????.#...#... 4,1,1",
            "????.######..#####. 1,6,5",
            "?###???????? 3,2,1"
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day12.Puzzle1(GetTestData());
        Assert.That(result, Is.EqualTo(21));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day12.txt");
        var result = Day12.Puzzle1(data);
        Assert.That(result, Is.EqualTo(7753));
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day12.Puzzle2(GetTestData());
        Assert.That(result, Is.EqualTo(525152));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day12.txt");
        var result = Day12.Puzzle2(data);
        Assert.That(result, Is.EqualTo(280382734828319)); 
    }
}