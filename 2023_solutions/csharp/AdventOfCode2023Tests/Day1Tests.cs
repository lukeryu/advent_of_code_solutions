namespace AdventOfCode2023Tests;

public class Tests
{
    [Test]
    public void Puzzle1TestData()
    {
        var result = Day1.Puzzle1(new List<string>());
        Assert.That(result, Is.EqualTo(0));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day1.txt");
        var result = Day1.Puzzle1(data);
        Assert.That(result, Is.EqualTo(0));
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day1.Puzzle2(new List<string>());
        Assert.That(result, Is.EqualTo(0));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day1.txt");
        var result = Day1.Puzzle2(data);
        Assert.That(result, Is.EqualTo(0));
    }
}