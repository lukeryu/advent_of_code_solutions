using AdventOfCode2023;

namespace AdventOfCode2023Tests;

public class Day5Tests
{
    private static List<string> GetTestData()
    {
        return new List<string>
        {
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4"
        };
    }

    [Test]
    public void Puzzle1TestData()
    {
        var result = Day5.Puzzle1(GetTestData());
        Assert.That(result, Is.EqualTo(35));
    }

    [Test]
    public void Puzzle1RealData()
    {
        var data = TestUtils.GetDataFromFile("Day5.txt");
        var result = Day5.Puzzle1(data);
        Assert.That(result, Is.EqualTo(324724204));
    }

    [Test]
    public void Puzzle2TestData()
    {
        var result = Day5.Puzzle2(GetTestData());
        Assert.That(result, Is.EqualTo(46));
    }

    [Test]
    public void Puzzle2RealData()
    {
        var data = TestUtils.GetDataFromFile("Day5.txt");
        var result = Day5.Puzzle2(data);
        Assert.That(result, Is.EqualTo(104070862)); // 104070863 too high
    }
    
    [Test]
    public void Puzzle2TestAsdf()
    {
        var result = Day5.Puzzle2RunItAll(GetTestData());
        Assert.That(result, Is.EqualTo(46));
    }
    
    [Test]
    public void Puzzle2RealData2()
    {
        var data = TestUtils.GetDataFromFile("Day5.txt");
        var result = Day5.DoWork(data);
        Assert.That(result, Is.EqualTo(104070862)); // 104070863 too high
    }
    
    [Test]
    public void TestIntersect()
    {
        var mapper = new Day5.MapperValues(10, 15, 10);
        
        var beforeResults = mapper.Intersects(new Tuple<ulong, ulong>(1, 9));
        Assert.That(beforeResults.IntersectingTuple, Is.Null);
        Assert.That(beforeResults.OutsideTuples, Is.Empty);
        
        var afterResults = mapper.Intersects(new Tuple<ulong, ulong>(25, 35));
        Assert.That(afterResults.IntersectingTuple, Is.Null);
        Assert.That(afterResults.OutsideTuples, Is.Empty);
        
        var midResults = mapper.Intersects(new Tuple<ulong, ulong>(11, 18));
        Assert.That(midResults.IntersectingTuple, Is.EqualTo(new Tuple<ulong, ulong>(11, 18)));
        Assert.That(midResults.OutsideTuples, Is.Empty);
        
        var fullSpanResults = mapper.Intersects(new Tuple<ulong, ulong>(0, 35));
        Assert.That(fullSpanResults.IntersectingTuple, Is.EqualTo(new Tuple<ulong, ulong>(10, 20)));
        Assert.That(fullSpanResults.OutsideTuples, Is.EqualTo(new List<Tuple<ulong, ulong>>
        {
            new(0, 9),
            new(21, 35)
        }));
        
        var preResults = mapper.Intersects(new Tuple<ulong, ulong>(0, 15));
        Assert.That(preResults.IntersectingTuple, Is.EqualTo(new Tuple<ulong, ulong>(10, 15)));
        Assert.That(preResults.OutsideTuples, Is.EqualTo(new List<Tuple<ulong, ulong>>
        {
            new(0, 9)
        }));
        
        var postResults = mapper.Intersects(new Tuple<ulong, ulong>(16, 25));
        Assert.That(postResults.IntersectingTuple, Is.EqualTo(new Tuple<ulong, ulong>(16, 20)));
        Assert.That(postResults.OutsideTuples, Is.EqualTo(new List<Tuple<ulong, ulong>>
        {
            new(21, 25)
        }));
        
    }

    [Test]
    public void TestMapReverse()
    {
        var data = TestUtils.GetDataFromFile("Day5.txt");
        data[0] = "seed: 104070862";

        var result = Day5.Puzzle1Reverse(data);
        Assert.That(result, Is.EqualTo(2956313548));
    } 
    
    [Test]
    public void TestMap()
    {
        var data = TestUtils.GetDataFromFile("Day5.txt");
        data[0] = "seed: 2956313548";

        var result = Day5.Puzzle1(data);
        Assert.That(result, Is.EqualTo(104070862));
    }
}