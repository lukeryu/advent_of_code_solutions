namespace AdventOfCode2023;

public class Day11
{
    private class Point
    {
        public Point(long X, long Y)
        {
            this.X = X;
            this.Y = Y;
        }

        public long DistanceBetween(Point other)
        {
            return long.Abs(X - other.X) + long.Abs(Y - other.Y);
        }

        public long X { get; set; }
        public long Y { get; set; }

        public void Deconstruct(out long X, out long Y)
        {
            X = this.X;
            Y = this.Y;
        }
    }

    public static ulong Puzzle1(List<string> input, long expansionDistance)
    {
        var galaxies = new List<Point>();
        var emptyRows = new List<long>();
        var emptyColumns = new List<long>();
        for (long i = 0; i < input[0].Length; i++)
        {
            emptyColumns.Add(i);
        }
        
        long rowCount = 0;
        foreach (var str in input)
        {
            long columnCount = 0;

            bool hasGalaxy = false;
            foreach (var point in str)
            {
                if (point == '#')
                {
                    galaxies.Add(new Point(columnCount, rowCount));
                    hasGalaxy = true;
                    emptyColumns.Remove(columnCount);
                }

                columnCount++;
            }

            if (!hasGalaxy)
            {
                emptyRows.Add(rowCount);
            }

            rowCount++;
        }
        
        emptyColumns.Sort();
        emptyColumns.Reverse();
        emptyRows.Sort();
        emptyRows.Reverse();

        foreach (var columnIndex in emptyColumns)
        {
            galaxies.Where((point) => point.X > columnIndex).ToList().ForEach((point) => point.X += expansionDistance);
        }
        
        foreach (var rowIndex in emptyRows)
        {
            galaxies.Where((point) => point.Y > rowIndex).ToList().ForEach((point) => point.Y += expansionDistance);
        }

        long totalDistance = 0;
        for (int i = 0; i < galaxies.Count; i++)
        {
            for (int j = i + 1; j < galaxies.Count; j++)
            {
                var lhsGal = galaxies[i];
                var rhsGal = galaxies[j];
                totalDistance += lhsGal.DistanceBetween(rhsGal);

            }
        }

        return (ulong) totalDistance;
    }

}