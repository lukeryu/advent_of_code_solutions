using AdventOfCode2022;

namespace AdventOfCode2022Tests;

public class Day2Tests
{
    private static List<string> GetTestData()
    {
        var testData = new List<string>();
        testData.Add("A Y");
        testData.Add("B X");
        testData.Add("C Z");

        return testData;
    }

    [Test]
    public void Test1()
    {
        var testData = GetTestData();

        var result = Day2.Puzzle1(testData);
        Assert.AreEqual(15, result);
    }

    [Test]
    public void Test1_Data()
    {
        var testData = TestUtils.GetDataFromFile("Day2.txt");

        var result = Day2.Puzzle1(testData);
        Assert.AreEqual(10941, result);
    }

    [Test]
    public void Test2()
    {
        var testData = GetTestData();

        var result = Day2.Puzzle2(testData);
        Assert.AreEqual(12, result);
    }

    [Test]
    public void Test2_Data()
    {
        var testData = TestUtils.GetDataFromFile("Day2.txt");

        var result = Day2.Puzzle2(testData);
        Assert.AreEqual(13071, result);
    }
}