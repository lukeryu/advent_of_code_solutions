using AdventOfCode2022;

namespace AdventOfCode2022Tests;

public class Day3Tests
{
    private static List<string> GetTestData()
    {
        var testData = new List<string>();
        testData.Add("vJrwpWtwJgWrhcsFMMfFFhFp");
        testData.Add("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        testData.Add("PmmdzqPrVvPwwTWBwg");
        testData.Add("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        testData.Add("ttgJtRGJQctTZtZT");
        testData.Add("CrZsJsPPZsGzwwsLwLmpwMDw");

        return testData;
    }

    [Test]
    public void Test1()
    {
        var testData = GetTestData();

        var result = Day3.Puzzle1(testData);
        Assert.AreEqual(157, result);
    }

    [Test]
    public void Test1_Data()
    {
        var testData = TestUtils.GetDataFromFile("Day3.txt");

        var result = Day3.Puzzle1(testData);
        Assert.AreEqual(8185, result);
    }

    [Test]
    public void Test2()
    {
        var testData = GetTestData();

        var result = Day3.Puzzle2(testData);
        Assert.AreEqual(70, result);
    }

    [Test]
    public void Test2_Data()
    {
        var testData = TestUtils.GetDataFromFile("Day3.txt");

        var result = Day3.Puzzle2(testData);
        Assert.AreEqual(2817, result);
    }
}