using System.Text.RegularExpressions;

namespace AdventOfCode2023;

public static partial class Day3
{
    private record Point(int X, int Y, char CharacterType) : IComparable<Point>
    {
        public int CompareTo(Point other)
        {
            var result = other.X - X;
            if (result == 0)
            {
                result = other.Y - Y;
            }

            return result;
        }

        public virtual bool Equals(Point? other)
        {
            if (ReferenceEquals(null, other)) return false;
            if (ReferenceEquals(this, other)) return true;
            return X == other.X && Y == other.Y;
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(X, Y);
        }
    }

    private record GridNumber(int Number, Point Begin, int Length)
    {
        public List<Point> GetAdjacentPoints()
        {
            var points = new List<Point>
            {
                new(Begin.X - 1, Begin.Y - 1,'x'),
                new(Begin.X - 1, Begin.Y,'x'),
                new(Begin.X - 1, Begin.Y + 1,'x'),
                new(Begin.X + Length, Begin.Y - 1,'x'),
                new(Begin.X + Length, Begin.Y,'x'),
                new(Begin.X + Length, Begin.Y + 1,'x')
            };
            for (var i = 0; i <= Length; i++)
            {
                points.Add(new Point(Begin.X + i, Begin.Y - 1,'x'));
                points.Add(new Point(Begin.X + i, Begin.Y + 1,'x'));
            }
            
            return points;
        }
    }

    private static Tuple<SortedSet<Point>, List<GridNumber>> ParseInput(List<string> inputLines)
    {
        var points = new SortedSet<Point>();
        var numbers = new List<GridNumber>();
        
        for (var rowIndex = 0; rowIndex < inputLines.Count; rowIndex++)
        {
            var row = inputLines[rowIndex];
            var matchCollection = NumberRegex().Matches(row);

            foreach (Match match in matchCollection)
            {
                var parsedValue = int.Parse(match.Captures[0].Value);
                numbers.Add(new GridNumber(parsedValue, new Point(match.Index, rowIndex ,'x'), match.Length));
            }
            
            for (var columnIndex = 0; columnIndex < row.Length; columnIndex++)
            {
                
                var currentChar = row[columnIndex];
                if (!currentChar.Equals('.') && (char.IsSymbol(currentChar) || char.IsPunctuation(currentChar)))
                {
                    points.Add(new Point( columnIndex, rowIndex, currentChar));
                }
            }   
        }

        return new Tuple<SortedSet<Point>, List<GridNumber>>(points, numbers);
    }

    public static ulong Puzzle1(List<string> inputLines)
    {
        var (points, numbers) = ParseInput(inputLines);

        return (from number in numbers let isAdjacentToPoint = IsAdjacentToPoint(number, points) where isAdjacentToPoint select number).Aggregate<GridNumber, ulong>(0, (current, number) => current + (ulong)number.Number);
    }

    private static bool IsAdjacentToPoint(GridNumber gridNumber, IReadOnlySet<Point> points)
    {
        return gridNumber.GetAdjacentPoints().Any(points.Contains);
    }

    public static ulong Puzzle2(List<string> inputLines)
    {
        var (points, numbers) = ParseInput(inputLines);

        var gears = points.Where(point => point.CharacterType == '*').ToList();

        ulong sum = 0;
        foreach (var gear in gears)
        {
            var adjacentNumbers = (from number in numbers let adjacentPoints = number.GetAdjacentPoints() where adjacentPoints.Contains(gear) select number).ToList();

            if (adjacentNumbers.Count == 2)
            {
                int? product = null;
                foreach (var number in adjacentNumbers)
                {
                    if (product == null)
                    {
                        product = number.Number;
                    }
                    else
                    {
                        product *= number.Number;
                    }
                }

                sum += (ulong) product.Value;
            }
            
        }

        return sum;
    }

    [GeneratedRegex("\\d+")]
    private static partial Regex NumberRegex();
}