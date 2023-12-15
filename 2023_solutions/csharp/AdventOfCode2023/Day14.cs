namespace AdventOfCode2023;

public class Day14
{
    private class MapCache
    {
        public BoulderType[][] CacheValues { get; }

        public MapCache(BoulderType[][] cacheValues)
        {
            CacheValues = cacheValues;
        }

        public override bool Equals(object? obj)
        {
            if (obj is null)
            {
                return false;
            }
            
            if (obj is MapCache mapCache)
            {
                if (CacheValues.Length != mapCache.CacheValues.Length)
                {
                    return false;
                }

                for (var i = 0; i < CacheValues.Length; i++)
                {
                    var lhs = CacheValues[i];
                    var rhs = mapCache.CacheValues[i];

                    if (!lhs.SequenceEqual(rhs))
                    {
                        return false;
                    }
                }
            }
            else
            {
                return false;
            }

            return true;
        }

        public override int GetHashCode()
        {
            int hash = 1;

            foreach (var val in CacheValues)
            {
                hash += (val.GetHashCode() * 37);
            }
            
            return hash;
        }
    }

    public enum BoulderType
    {
        ROUNDED,
        CUBED,
        NONE
    }

    public static ulong Puzzle1(List<string> input)
    {
        var parsedBoulders = ParseBoulders(input);
        var shiftedBoulders = ShiftNorth(parsedBoulders);

        return CalculateNorthSupportLoad(shiftedBoulders);
    }

    private static ulong CalculateNorthSupportLoad(BoulderType[][] boulderTypes)
    {
        ulong totalLoad = 0;

        var boulderRowCount = boulderTypes.Length;
        for (var i = boulderRowCount - 1; i >= 0; i--)
        {
            var boulderRow = boulderTypes[i];
            var roundedBoulderCount = boulderRow.Count(type => type == BoulderType.ROUNDED);
            var rowPointWorth = (boulderRowCount - i);
            totalLoad += (ulong)(roundedBoulderCount * rowPointWorth);
        }

        return totalLoad;
    }

    private static BoulderType[][] ParseBoulders(List<string> input)
    {
        return input.Select(inString => inString.Select(ParseBoulderType).ToArray()).ToArray();
    }

    private static BoulderType ParseBoulderType(char arg)
    {
        return arg switch
        {
            'O' => BoulderType.ROUNDED,
            '#' => BoulderType.CUBED,
            _ => BoulderType.NONE
        };
    }

    public static ulong Puzzle2(List<string> input)
    {
        var shiftedBoulders = ParseBoulders(input);

        var cache = new Dictionary<MapCache, long>();

        for (long cycle = 1; cycle < 1_000_000_000; cycle++)
        {
            shiftedBoulders = ShiftNorth(shiftedBoulders);
            shiftedBoulders = ShiftWest(shiftedBoulders);
            shiftedBoulders = ShiftSouth(shiftedBoulders);
            shiftedBoulders = ShiftEast(shiftedBoulders);

            if (cache.TryGetValue(new MapCache(shiftedBoulders), out var cached))
            {
                var remaining = 1_000_000_000 - cycle - 1;
                var loop = cycle - cached;

                var loopRemaining = remaining % loop;
                cycle = 1_000_000_000 - loopRemaining - 1;
            }

            var cacheValues = (BoulderType[][]) shiftedBoulders.Clone();
            cache[new MapCache(cacheValues)] = cycle;
        }

        // List<List<BoulderType>> endBoulders = new();
        // foreach (var values in cache)
        // {
        //     if (values.Value == endingStateValue)
        //     {
        //         endBoulders = values.Key.CacheValues;
        //     }
        // }

        return CalculateNorthSupportLoad(shiftedBoulders);
    }

    private static BoulderType[][] ShiftNorth(BoulderType[][] parsedBoulders)
    {
        BoulderType[][] tiltedBoulders = (BoulderType[][]) parsedBoulders.Clone();

        for (var row = 1; row < tiltedBoulders.Length; row++)
        {
            for (var col = 0; col < tiltedBoulders[row].Length; col++)
            {
                var c = tiltedBoulders[row][col];
 
                if (c != BoulderType.ROUNDED)
                {
                    continue;
                }
 
                var previous = 1;
                while (tiltedBoulders[row - previous][col] == BoulderType.NONE)
                {
                    tiltedBoulders[row - previous][col] = BoulderType.ROUNDED;
                    tiltedBoulders[row - previous + 1][col] = BoulderType.NONE;
                    previous++;
 
                    if (row - previous < 0)
                    {
                        break;
                    }
                }
            }
        }

        return tiltedBoulders;
    }

    private static BoulderType[][] ShiftWest(BoulderType[][] shiftedBoulders)
    {
        BoulderType[][] tiltedBoulders = (BoulderType[][]) shiftedBoulders.Clone();

        for (var row = 0; row < tiltedBoulders.Length; row++)
        {
            for (var col = 1; col < tiltedBoulders[row].Length; col++)
            {
                var c = tiltedBoulders[row][col];
 
                if (c != BoulderType.ROUNDED)
                {
                    continue;
                }
 
                var previous = 1;
                while (tiltedBoulders[row][col - previous] == BoulderType.NONE)
                {
                    tiltedBoulders[row][col - previous] = BoulderType.ROUNDED;
                    tiltedBoulders[row][col - previous + 1] = BoulderType.NONE;
                    previous++;
 
                    if (col - previous < 0)
                    {
                        break;
                    }
                }
            }
        }

        return tiltedBoulders;
    }

    private static BoulderType[][] ShiftSouth(BoulderType[][] shiftedBoulders)
    {
        BoulderType[][] tiltedBoulders = (BoulderType[][]) shiftedBoulders.Clone();
        for (var row = tiltedBoulders.Length - 2; row >= 0; row--)
        {
            for (var col = 0; col < tiltedBoulders[row].Length; col++)
            {
                var c = tiltedBoulders[row][col];
 
                if (c != BoulderType.ROUNDED)
                {
                    continue;
                }
 
                var previous = 1;
                while (tiltedBoulders[row + previous][col] == BoulderType.NONE)
                {
                    tiltedBoulders[row + previous][col] = BoulderType.ROUNDED;
                    tiltedBoulders[row + previous - 1][col] = BoulderType.NONE;
                    previous++;
 
                    if (row + previous >= tiltedBoulders.Length)
                    {
                        break;
                    }
                }
            }
        }

        return tiltedBoulders;
    }

    private static BoulderType[][] ShiftEast(BoulderType[][] shiftedBoulders)
    {
        BoulderType[][] tiltedBoulders = (BoulderType[][]) shiftedBoulders.Clone();
        for (var row = 0; row < tiltedBoulders.Length; row++)
        {
            for (var col = tiltedBoulders[row].Length - 2; col >= 0; col--)
            {
                var c = tiltedBoulders[row][col];
 
                if (c != BoulderType.ROUNDED)
                {
                    continue;
                }
 
                var previous = 1;
                while (tiltedBoulders[row][col + previous] == BoulderType.NONE)
                {
                    tiltedBoulders[row][col + previous] = BoulderType.ROUNDED;
                    tiltedBoulders[row][col + previous - 1] = BoulderType.NONE;
                    previous++;
 
                    if (col + previous >= tiltedBoulders[row].Length)
                    {
                        break;
                    }
                }
            }
        }

        return tiltedBoulders;
    }

    private static void FilterNorthRoundedBoulders(List<List<BoulderType>> tiltedBoulders, int boulderRow)
    {
        var rowLength = tiltedBoulders[0].Count;

        for (var currentRow = boulderRow; currentRow >= 1; currentRow--)
        {
            for (var columnLocation = 0; columnLocation < rowLength; columnLocation++)
            {
                if (tiltedBoulders[currentRow][columnLocation] == BoulderType.ROUNDED
                    && tiltedBoulders[currentRow - 1][columnLocation] == BoulderType.NONE)
                {
                    tiltedBoulders[currentRow - 1][columnLocation] = BoulderType.ROUNDED;
                    tiltedBoulders[currentRow][columnLocation] = BoulderType.NONE;
                }
            }
        }
    }
}