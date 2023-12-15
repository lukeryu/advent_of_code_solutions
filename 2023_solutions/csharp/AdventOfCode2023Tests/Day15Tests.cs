using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day15Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day15.Puzzle1(GetTestData());
        Assert.That(result, Is.EqualTo(1320));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day15.txt");
        var result = Day15.Puzzle1(data);
        Assert.That(result, Is.EqualTo(511416)); 
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day15.Puzzle2(GetTestData());
        Assert.That(result, Is.EqualTo(145));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day15.txt");
        var result = Day15.Puzzle2(data);
        Assert.That(result, Is.EqualTo(290779));
    }
}