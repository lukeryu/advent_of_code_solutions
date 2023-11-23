namespace AdventOfCode2022Tests;

public class TestUtils
{
    public static List<string> GetDataFromFile(string filename)
    {
        var testData = new List<string>();

        String line;
        StreamReader sr = null;
        try
        {
            sr = new StreamReader("../../../../../../data/" + filename);
            
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