using static AdventOfCode2022.Day1;

namespace AdventOfCode2022Tests;

public class Day1Tests
{
    private static List<string> GetTestData()
    {
        var testData = new List<string>();
        testData.Add("1000");
        testData.Add("2000");
        testData.Add("3000");
        testData.Add("");
        testData.Add("4000");
        testData.Add("");
        testData.Add("5000");
        testData.Add("6000");
        testData.Add("");
        testData.Add("7000");
        testData.Add("8000");
        testData.Add("9000");
        testData.Add("");
        testData.Add("10000");

        return testData;
    }

    [Test]
    public void Test1()
    {
        var testData = GetTestData();

        var result = Puzzle1(testData);
        Assert.AreEqual(24000, result);
    }

    [Test]
    public void Test1_Data()
    {
        var testData = TestUtils.GetDataFromFile("Day1.txt");

        var result = Puzzle1(testData);
        Assert.AreEqual(71780, result);
    }

    [Test]
    public void Test2()
    {
        var testData = GetTestData();

        var result = Puzzle2(testData);
        Assert.AreEqual(45000, result);
    }

    [Test]
    public void Test2_Data()
    {
        var testData = TestUtils.GetDataFromFile("Day1.txt");

        var result = Puzzle2(testData);
        Assert.AreEqual(212489, result);
    }
}