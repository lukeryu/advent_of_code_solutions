namespace AdventOfCode2022Tests;

public class TestUtils
{
    public static List<string> GetDataFromFile(string filename)
    {
        var testData = new List<string>();

        string? line;
        using (StreamReader sr = new StreamReader("../../../../../../data/" + filename))
        {
            line = sr.ReadLine();

            while (line != null)
            {
                testData.Add(line);
                line = sr.ReadLine();
            }
        }

        return testData;
    }
}