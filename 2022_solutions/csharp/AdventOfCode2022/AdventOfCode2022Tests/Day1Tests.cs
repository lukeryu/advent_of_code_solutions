using AdventOfCode2022;
using System.IO;

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

    private static List<string> GetDataFromFile()
    {
        var testData = new List<string>();

        String line;
        StreamReader sr = null;
        try
        {
            sr = new StreamReader("../../../../../../data/Day1.txt");
            
            line = sr.ReadLine();

            while (line != null)
            {
                testData.Add(line);
                line = sr.ReadLine();
            }
        }
        finally
        {
            sr?.Close();
        }


        return testData;
    }

    [Test]
    public void Test1()
    {
        var testData = GetTestData();

        var result = Day1.Puzzle1(testData);
        Assert.AreEqual(24000, result);
    }

    [Test]
    public void Test1_Data()
    {
        var testData = GetDataFromFile();

        var result = Day1.Puzzle1(testData);
        Assert.AreEqual(71780, result);
    }

    [Test]
    public void Test2()
    {
        var testData = GetTestData();

        var result = Day1.Puzzle2(testData);
        Assert.AreEqual(45000, result);
    }

    [Test]
    public void Test2_Data()
    {
        var testData = GetDataFromFile();

        var result = Day1.Puzzle2(testData);
        Assert.AreEqual(212489, result);
    }
}