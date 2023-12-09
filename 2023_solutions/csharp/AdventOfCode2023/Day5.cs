using System.ComponentModel.DataAnnotations;
using OpenCL.Net;

namespace AdventOfCode2023;

public static class Day5
{
    private record ParsedProblem(List<ulong> Seeds,
        AlmanacMapper SeedToSoilMap,
        AlmanacMapper SoilToFertilizerMap,
        AlmanacMapper FertilizerToWaterMap,
        AlmanacMapper WaterToLightMap,
        AlmanacMapper LightToTemperatureMap,
        AlmanacMapper TemperatureToHumidityMap,
        AlmanacMapper HumidityToLocationMap);

    public record IntersectionResults(Tuple<ulong, ulong>? IntersectingTuple,
        List<Tuple<ulong, ulong>> OutsideTuples);


    public record MapperValues(ulong SourceStart, ulong DestinationStart, ulong Range)
    {
        public ulong TryMap(ulong from)
        {
            if (from < SourceStart)
            {
                return 0;
            }

            var sourceEnd = SourceStart + Range;

            if (from > sourceEnd)
            {
                return 0;
            }

            var difference = from - SourceStart;

            return DestinationStart + difference;
        }
        
        public ulong TryReverseMap(ulong from)
        {
            if (from < DestinationStart)
            {
                return 0;
            }

            var destEnd = DestinationStart + Range;

            if (from > destEnd)
            {
                return 0;
            }

            var difference = from - DestinationStart;

            return SourceStart + difference;
        }

        public IntersectionResults Intersects(Tuple<ulong, ulong> tuple)
        {
            var sourceEnd = (SourceStart + Range);
            if ((tuple.Item2 < SourceStart && tuple.Item1 < SourceStart) || (tuple.Item1 > sourceEnd && tuple.Item2 > sourceEnd))
            {
                return new IntersectionResults(null, new List<Tuple<ulong, ulong>>()); //No Intersection   
            }

            if (tuple.Item1 >= SourceStart && tuple.Item2 <= sourceEnd)
            {
                return new IntersectionResults(tuple,
                    new List<Tuple<ulong, ulong>>()); //Full Intersection, no outside   
            }


            var outerTuples = new List<Tuple<ulong, ulong>>();

            ulong lowerBound;
            if (tuple.Item1 < SourceStart)
            {
                lowerBound = SourceStart;
                outerTuples.Add(new Tuple<ulong, ulong>(tuple.Item1, SourceStart - 1));
            }
            else
            {
                lowerBound = tuple.Item1;
            }

            ulong upperBound;
            if (tuple.Item2 > sourceEnd)
            {
                upperBound = sourceEnd;
                outerTuples.Add(new Tuple<ulong, ulong>(sourceEnd + 1, tuple.Item2));
            }
            else
            {
                upperBound = tuple.Item2;
            }

            var intersectingTuple = new Tuple<ulong, ulong>(lowerBound, upperBound);
            return new IntersectionResults(intersectingTuple, outerTuples);
        }
    }

    private class AlmanacMapper
    {
        private readonly List<MapperValues> _mapperValuesList;

        public AlmanacMapper(List<MapperValues> mapperValuesList)
        {
            _mapperValuesList = mapperValuesList;
        }

        public ulong Map(ulong from)
        {
            foreach (var mapperValue in _mapperValuesList)
            {
                var to = mapperValue.TryMap(from);
                if (to > 0)
                {
                    return to;
                }
            }

            return from;
        }
        
        public ulong ReverseMap(ulong from)
        {
            foreach (var mapperValue in _mapperValuesList)
            {
                var to = mapperValue.TryReverseMap(from);
                if (to > 0)
                {
                    return to;
                }
            }

            return from;
        }

        public List<Tuple<ulong, ulong>> Map(List<Tuple<ulong, ulong>> from)
        {
            var returnValues = new List<Tuple<ulong, ulong>>();
            foreach (var tuple in from)
            {
                var hasBeenMapped = false;
                foreach (var mapperValue in _mapperValuesList)
                {
                    if (!hasBeenMapped)
                    {
                        var intersectionResults = mapperValue.Intersects(tuple);
                        if (intersectionResults.IntersectingTuple == null)
                        {
                            continue;
                        }

                        returnValues.Add(new Tuple<ulong, ulong>(
                            mapperValue.TryMap(intersectionResults.IntersectingTuple.Item1),
                            mapperValue.TryMap(intersectionResults.IntersectingTuple.Item2))
                        );

                        returnValues.AddRange(Map(intersectionResults.OutsideTuples));
                        hasBeenMapped = true;
                    }
                }

                if (!hasBeenMapped)
                {
                    returnValues.Add(tuple);
                }
            }
            
            returnValues.Sort((lhs, rhs) => lhs.Item1.CompareTo(rhs.Item1));

            return returnValues;
        }
    }

    private static List<ulong> ParseSeeds(string line)
    {
        var seeds = new List<ulong>();
        var tokens = line.Split(" ");

        for (var index = 1; index < tokens.Length; index++)
        {
            seeds.Add(ulong.Parse(tokens[index]));
        }

        return seeds;
    }

    private static AlmanacMapper ParseAlmanacMapper(List<string> lines, string header)
    {
        var mappers = new List<MapperValues>();
        var findIndex = lines.FindIndex(EqualPredicate) + 1;

        while (true)
        {
            if (findIndex >= lines.Count)
            {
                break;
            }

            var line = lines[findIndex];

            if ("".Equals(line.Trim()))
            {
                break;
            }

            var values = line.Split(" ");
            var destinationStart = ulong.Parse(values[0]);
            var sourceStart = ulong.Parse(values[1]);
            var rangeLength = ulong.Parse(values[2]);

            mappers.Add(new MapperValues(sourceStart, destinationStart, rangeLength));

            findIndex++;
        }

        return new AlmanacMapper(mappers);

        bool EqualPredicate(string s) => s.Trim().Equals(header.Trim());
    }

    public static ulong Puzzle1(List<string> inputLines)
    {
        var problem = ParseProblem(inputLines);

        var max = ulong.MaxValue;

        foreach (var seed in problem.Seeds)
        {
            var soil = problem.SeedToSoilMap.Map(seed);
            var fertilizer = problem.SoilToFertilizerMap.Map(soil);
            var water = problem.FertilizerToWaterMap.Map(fertilizer);
            var light = problem.WaterToLightMap.Map(water);
            var temp = problem.LightToTemperatureMap.Map(light);
            var humid = problem.TemperatureToHumidityMap.Map(temp);
            var location = problem.HumidityToLocationMap.Map(humid);
            
            if (location < max)
            {
                max = location;
            }
        }

        return max;
    }
    
    public static ulong Puzzle1Reverse(List<string> inputLines)
    {
        var problem = ParseProblem(inputLines);

        var max = ulong.MaxValue;

        foreach (var location in problem.Seeds)
        {
            var humid = problem.HumidityToLocationMap.ReverseMap(location);
            var temp = problem.TemperatureToHumidityMap.ReverseMap(humid);
            var light = problem.LightToTemperatureMap.ReverseMap(temp);
            var water = problem.WaterToLightMap.ReverseMap(light);
            var fertilizer = problem.FertilizerToWaterMap.ReverseMap(water);
            var soil = problem.SoilToFertilizerMap.ReverseMap(fertilizer);
            var seed = problem.SeedToSoilMap.ReverseMap(soil);
            
            if (seed < max)
            {
                max = seed;
            }
        }

        return max;
    }

    private static ParsedProblem ParseProblem(List<string> inputLines)
    {
        var seeds = ParseSeeds(inputLines[0]);
        var seedToSoilMap = ParseAlmanacMapper(inputLines, "seed-to-soil map:");
        var soilToFertilizerMap = ParseAlmanacMapper(inputLines, "soil-to-fertilizer map:");
        var fertilizerToWaterMap = ParseAlmanacMapper(inputLines, "fertilizer-to-water map:");
        var waterToLightMap = ParseAlmanacMapper(inputLines, "water-to-light map:");
        var lightToTemperatureMap = ParseAlmanacMapper(inputLines, "light-to-temperature map:");
        var temperatureToHumidityMap = ParseAlmanacMapper(inputLines, "temperature-to-humidity map:");
        var humidityToLocationMap = ParseAlmanacMapper(inputLines, "humidity-to-location map:");

        return new ParsedProblem(seeds,
            seedToSoilMap,
            soilToFertilizerMap,
            fertilizerToWaterMap,
            waterToLightMap,
            lightToTemperatureMap,
            temperatureToHumidityMap,
            humidityToLocationMap);
    }

    public static ulong Puzzle2(List<string> inputLines)
    {
        var problem = ParseProblem(inputLines);

        var seedList = problem.Seeds;
        var seedRanges = new List<Tuple<ulong, ulong>>();
        for (var i = 0; i < seedList.Count; i += 2)
        {
            seedRanges.Add(new(seedList[i], seedList[i] + seedList[i + 1] - 1));
        }

        foreach (var range in seedRanges)
        {
            if (range.Item1 <= 2956313548 && range.Item2 >= 2956313548)
            {
                Console.Out.WriteLine(range.Item1);
            }
        }

        var soilRanges = problem.SeedToSoilMap.Map(seedRanges);
        var fertilizerRanges = problem.SoilToFertilizerMap.Map(soilRanges);
        var waterRanges = problem.FertilizerToWaterMap.Map(fertilizerRanges);
        var lightRanges = problem.WaterToLightMap.Map(waterRanges);
        var tempRanges = problem.LightToTemperatureMap.Map(lightRanges);
        var humidRanges = problem.TemperatureToHumidityMap.Map(tempRanges);
        var locationRanges = problem.HumidityToLocationMap.Map(humidRanges);

        return locationRanges.Select(tuple => tuple.Item1).Min();
    }

    public static ulong Puzzle2RunItAll(List<string> inputLines)
    {
        var problem = ParseProblem(inputLines);

        var seedList = problem.Seeds;
        var seedRanges = new List<Tuple<ulong, ulong>>();
        for (var i = 0; i < seedList.Count; i += 2)
        {
            seedRanges.Add(new Tuple<ulong, ulong>(seedList[i], seedList[i] + seedList[i + 1] - 1));
        }

        
        var max = ulong.MaxValue;

        foreach (var seedRange in seedRanges)
        {
            for (var seed = seedRange.Item1; seed <= seedRange.Item2; seed++)
            {
                var soil = problem.SeedToSoilMap.Map(seed);
                var fertilizer = problem.SoilToFertilizerMap.Map(soil);
                var water = problem.FertilizerToWaterMap.Map(fertilizer);
                var light = problem.WaterToLightMap.Map(water);
                var temp = problem.LightToTemperatureMap.Map(light);
                var humid = problem.TemperatureToHumidityMap.Map(temp);
                var location = problem.HumidityToLocationMap.Map(humid);

                if (location < max)
                {
                    max = location;
                }
            }
        }

        return max;
    }

    //From https://github.com/andrewscodedump/Advent/blob/master/2023/Done/Days01-05/Day05.cs
    public static long DoWork(List<string> Inputs)
    {
        char[] spaceOrHyphen = {' ', '-'};
        Dictionary<string, string> mappingTypes = new();
        List<(string sourceType, long sourceFrom, long sourceTo, long destOffset)> mappings = new();
        long lowestDest = long.MaxValue;
        string type = "", destType = "";
        long[] seeds = Array.ConvertAll(Inputs[0].Split(' ')[1..], long.Parse);

        foreach (string line in Inputs.Skip(2))
        {
            if (line == "") continue;
            if (line.Contains('-'))
            {
                string[] words = line.Split(spaceOrHyphen, StringSplitOptions.RemoveEmptyEntries);
                type = words[0]; destType = words[2];
                mappingTypes[type] = destType;
            }
            else
            {
                long[] numbers = Array.ConvertAll(line.Split(' '), long.Parse);
                mappings.Add((type, numbers[1], numbers[1] + numbers[2] - 1, numbers[0] - numbers[1]));
            }
        }

        if (false)
        {
            foreach (long seedNumber in seeds)
            {
                type = "seed";
                long sourceNumber = seedNumber;
                do
                {
                    (string sourceType, long sourceFrom, long sourceTo, long destOffset)
                        = mappings.LastOrDefault(m => m.sourceType == type && m.sourceFrom <= sourceNumber && sourceNumber <= m.sourceTo, ("", 0, 0, 0));
                    sourceNumber += destOffset;
                    type = mappingTypes[type];
                } while (type != "location");
                lowestDest = Math.Min(lowestDest, sourceNumber);
            }
        }
        else
        {
            List<(long from, long to)> ranges = new();
            for (int i = 0; i < seeds.Length; i += 2)
            {
                ranges.Add((seeds[i], seeds[i] + seeds[i + 1] - 1));
            }
            type = "seed";
            do
            {
                var filteredMappings = mappings.Where(m => m.sourceType == type).OrderBy(m => m.sourceFrom);
                ranges = GetDestRanges(ranges.OrderBy(r => r.from).ToList(), new List<(string sourceType, long sourceFrom, long sourceTo, long destOffset)>(filteredMappings));
                type = mappingTypes[type];
            } while (type != "location");
            lowestDest = Math.Min(lowestDest, ranges.Min(r => r.from));
        }
        return lowestDest;
    }

    static List<(long, long)> GetDestRanges(List<(long, long)> sourceRanges, List<(string sourceType, long sourceFrom, long sourceTo, long destOffset)> mappings)
    {
        List<(long, long)> result = new();
        foreach ((long , long) sourceRange in sourceRanges)
        {
            (long from, long to) testRange = sourceRange;
            bool allDone = false;
            do
            {
                // Find the last mapping where the start is less than the start of the range
                (string sourceType, long sourceFrom, long sourceTo, long destOffset)
                    = mappings.LastOrDefault(m => m.sourceFrom <= testRange.from && testRange.from <= m.sourceTo, ("", 0, 0, 0));
                // There aren't any
                if (sourceType == "")
                {
                    // Does the end fit in any mappings?
                    (sourceType, sourceFrom, sourceTo, destOffset)
                        = mappings.LastOrDefault(m => m.sourceFrom <= testRange.to && testRange.to <= m.sourceTo, ("", 0, 0, 0));
                    if (sourceType == "")
                    {
                        // If there aren't any, add the whole range and end
                        result.Add(testRange);
                        allDone = true;
                    }
                    else
                    {
                        // Add the start of the range, set the range to the end and continue
                        result.Add((testRange.from, sourceFrom - 1));
                        testRange = (sourceFrom, testRange.to);
                    }
                }
                // If the end of the mapping is greater than the end of the range, add the whole range (with offset) and end
                else if (sourceTo >= testRange.to)
                {
                    result.Add((testRange.from + destOffset, testRange.to + destOffset));
                    allDone = true;
                }
                // Otherwise, add from the start of the range to the end of the mapping (with offsets), set the range start to the mapping end plus one and continue
                else
                {
                    //sourceNumber = sourceNumber + destFrom - sourceFrom;
                    result.Add((testRange.from + destOffset, sourceTo + destOffset));
                    testRange = (sourceTo + 1, testRange.to);
                }
            } while (!allDone);
        }
        return result;
    }

    // public static ulong Puzzle1OpenCL(List<string> inputLines)
    // {
    //     var problem = ParseProblem(inputLines);
    //     var data = problem.Seeds.ToArray();
    //     var size = problem.Seeds.Count;
    //
    //     Device[] device = new Device[1];
    //     Cl.GetDeviceIDs(null, DeviceType.Gpu, 1,device, null);
    //     var context = Cl.CreateContext(null, 1, device, null, null, null);
    //     var queue = Cl.CreateCommandQueue(context, device[0], CommandQueueProperties.None, null);
    //
    //     string[] source = {
    //         ""
    //     };
    //     var program = Cl.CreateProgramWithSource(context, 1, source, null, null);
    //     Cl.BuildProgram(program, 0, null, null, null, null);
    //     var kernel = Cl.CreateKernel(program, "maps", null);
    //     var buffer = Cl.CreateBuffer<ulong>(context, MemFlags.ReadWrite, size, null);
    //
    //     Cl.EnqueueWriteBuffer(queue, buffer, Bool.False, 0, size, data, 0, null, null);
    //     Cl.SetKernelArg<ulong>(kernel, 0, buffer);
    //     IntPtr[] globalDimensions = {LENGTH, 0, 0};
    //     Cl.EnqueueNDRangeKernel(queue, kernel, 1, null, globalDimensions, null, 0, null, null);
    //     
    //     Event evnt;
    //     Cl.EnqueueReadBuffer(queue, buffer, Bool.False, 0, 0, data, 0, null, out evnt);
    //
    //     Cl.Finish(queue);
    //
    //     var max = ulong.MaxValue;
    //     
    //     var seedList = problem.Seeds;
    //     var seedRanges = new List<Tuple<ulong, ulong>>();
    //     for (var i = 0; i < seedList.Count; i += 2)
    //     {
    //         seedRanges.Add(new Tuple<ulong, ulong>(seedList[i], seedList[i] + seedList[i + 1] - 1));
    //     }
    //
    //     foreach (var seed in problem.Seeds)
    //     {
    //         var soil = problem.SeedToSoilMap.Map(seed);
    //         var fertilizer = problem.SoilToFertilizerMap.Map(soil);
    //         var water = problem.FertilizerToWaterMap.Map(fertilizer);
    //         var light = problem.WaterToLightMap.Map(water);
    //         var temp = problem.LightToTemperatureMap.Map(light);
    //         var humid = problem.TemperatureToHumidityMap.Map(temp);
    //         var location = problem.HumidityToLocationMap.Map(humid);
    //
    //         if (location < max)
    //         {
    //             max = location;
    //         }
    //     }
    //
    //     return max;
    // }
}