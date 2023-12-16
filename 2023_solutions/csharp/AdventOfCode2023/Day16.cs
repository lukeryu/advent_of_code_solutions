namespace AdventOfCode2023;

public static class Day16
{
    private enum Direction
    {
        NORTH,
        SOUTH,
        EAST,
        WEST
    }

    private record Point(int X, int Y) : IComparable<Point>
    {
        public int CompareTo(Point? other)
        {
            if (other == null)
            {
                return 1;
            }

            var comp = X.CompareTo(other.X);
            if (comp == 0)
            {
                comp = Y.CompareTo(other.Y);
            }

            return comp;
        }
    }

    private record Vector(Point Location, Direction Dir) : IComparable<Vector>
    {
        public Vector Move()
        {
            switch (Dir)
            {
                case Direction.NORTH:
                    return new Vector(new Point(Location.X, Location.Y - 1), Dir);
                case Direction.SOUTH:
                    return new Vector(new Point(Location.X, Location.Y + 1), Dir);
                case Direction.EAST:
                    return new Vector(new Point(Location.X + 1, Location.Y), Dir);
                case Direction.WEST:
                    return new Vector(new Point(Location.X - 1, Location.Y), Dir);
            }

            return this;
        }

        public int CompareTo(Vector? other)
        {
            if (other == null)
            {
                return 1;
            }

            var cmp = Location.CompareTo(other.Location);
            if (cmp == 0)
            {
                cmp = Dir - other.Dir;
            }

            return cmp;
        }
    }

    public static ulong Puzzle1(List<string> inputs)
    {
        return RunBeams(inputs, new Vector(new Point(0, 0), Direction.EAST));
    }

    private static ulong RunBeams(List<string> inputs, Vector initialVector)
    {
        var visitedPoints = new SortedSet<Vector>();
        visitedPoints.Add(initialVector);

        var rows = inputs.Count;
        var columns = inputs[0].Length;

        var vectors = new List<Vector>();

        vectors.Add(initialVector);

        while (vectors.Any())
        {
            var nextVectors = new List<Vector>();
            foreach (var vector in vectors)
            {
                var space = inputs[vector.Location.Y][vector.Location.X];
                switch (space)
                {
                    case '.':
                        nextVectors.Add(vector.Move());
                        break;
                    case '/':
                        nextVectors.Add(new Vector(vector.Location, ChangeForwardSlashDirection(vector.Dir)).Move());
                        break;
                    case '\\':
                        nextVectors.Add(new Vector(vector.Location, ChangeBackSlashDirection(vector.Dir)).Move());
                        break;
                    case '|':
                        if (vector.Dir == Direction.NORTH || vector.Dir == Direction.SOUTH)
                        {
                            nextVectors.Add(vector.Move());
                        }
                        else
                        {
                            nextVectors.Add(new Vector(vector.Location, Direction.NORTH).Move());
                            nextVectors.Add(new Vector(vector.Location, Direction.SOUTH).Move());
                        }

                        break;
                    case '-':
                        if (vector.Dir == Direction.EAST || vector.Dir == Direction.WEST)
                        {
                            nextVectors.Add(vector.Move());
                        }
                        else
                        {
                            nextVectors.Add(new Vector(vector.Location, Direction.EAST).Move());
                            nextVectors.Add(new Vector(vector.Location, Direction.WEST).Move());
                        }

                        break;
                }
            }

            vectors = nextVectors.Where(vec =>
                    vec.Location.X >= 0 && vec.Location.X < columns && vec.Location.Y >= 0 && vec.Location.Y < rows)
                .Where(vec => !visitedPoints.Contains(vec))
                .ToList();
            
            vectors.ForEach(vec => visitedPoints.Add(vec));
        }


        return (ulong)visitedPoints.Select(vec => vec.Location).Distinct().Count();
    }

    private static Direction ChangeBackSlashDirection(Direction vectorDir)
    {
        return vectorDir switch
        {
            Direction.NORTH => Direction.WEST,
            Direction.SOUTH => Direction.EAST,
            Direction.EAST => Direction.SOUTH,
            Direction.WEST => Direction.NORTH,
            _ => vectorDir
        };
    }

    private static Direction ChangeForwardSlashDirection(Direction vectorDir)
    {
        return vectorDir switch
        {
            Direction.NORTH => Direction.EAST,
            Direction.SOUTH => Direction.WEST,
            Direction.EAST => Direction.NORTH,
            Direction.WEST => Direction.SOUTH,
            _ => vectorDir
        };
    }

    public static ulong Puzzle2(List<string> inputs)
    {
        var maxRow = inputs.Count - 1;
        var maxColumn = inputs[0].Length - 1;

        var initialVectors = new List<Vector>();

        for (int i = 0; i <= maxRow; i++)
        {
            initialVectors.Add(new Vector(new Point(0, i), Direction.EAST));
            initialVectors.Add(new Vector(new Point(maxColumn, i), Direction.WEST));
        }
        
        for (int i = 0; i <= maxColumn; i++)
        {
            initialVectors.Add(new Vector(new Point(i, 0), Direction.SOUTH));
            initialVectors.Add(new Vector(new Point(i, maxRow), Direction.NORTH));
        }

        return initialVectors.Select(vec => RunBeams(inputs, vec)).Max();
    }
}