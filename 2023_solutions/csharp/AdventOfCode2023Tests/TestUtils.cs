namespace AdventOfCode2023Tests;

public class TestUtils
{
    public static List<string> GetDataFromFile(string filename)
    {
        var testData = new List<string>();

        string? line;
        StreamReader sr;
        try
        {
            sr = new StreamReader("../../../../../data/" + filename);
            
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
}