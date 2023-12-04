using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day4Test
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day4.Puzzle1(GetTestData());
        Assert.That(result, Is.EqualTo(13));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day4.txt");
        var result = Day4.Puzzle1(data);
        Assert.That(result, Is.EqualTo(18519));
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day4.Puzzle2(GetTestData());
        Assert.That(result, Is.EqualTo(30));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day4.txt");
        var result = Day4.Puzzle2(data);
        Assert.That(result, Is.EqualTo(11787590));
    }
}