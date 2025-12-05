namespace AdventOfCode2023;

public static partial class Day18
{
    private enum Direction
    {
        Up,
        Down,
        Left,
        Right
    };
    
    private record Point(long x, long y);
        
    private record D18Instruction(Direction direction, long number, string color);

    private static D18Instruction ParseInstruction(string str)
    {
        var values = str.Split(" ");

        var direction = values[0] switch
        {
            "U" => Direction.Up,
            "D" => Direction.Down,
            "L" => Direction.Left,
            "R" => Direction.Right,
            _ => Direction.Up
        };

        var amount = long.Parse(values[1]);

        var color = values[2].Substring(2, 6);

        return new D18Instruction(direction, amount, color);
    }
    
    public static ulong Puzzle1(List<string> input)
    {
        var points = new SortedSet<Point>();
        var instructions = input.Select(ParseInstruction).ToList();

        var currentPoint = new Point(0, 0);
        points.Add(currentPoint);

        foreach (var inst in instructions)
        {
            switch (inst.direction)
            {
                case Direction.Down:
                    break;
                case Direction.Left:
                    break;
                case Direction.Right:
                    break;
                case Direction.Up:
                default:
                    break;
            }
                
        }
        
        return 0;
    }

    public static ulong Puzzle2(IEnumerable<string> input)
    {
        return 0;
    }
}