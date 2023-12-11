using System.Numerics;

namespace AdventOfCode2023;

public static class Day10
{
    private record Point<T>(T X, T Y) : IComparable<Point<T>>, IEquatable<Point<T>> where T : IComparable<T>, IEquatable<T>
    {
        public int CompareTo(Point<T>? other)
        {
            if (other == null)
            {
                return 1;
            }

            var comparable = other.X.CompareTo(this.X);
            return comparable != 0 ? comparable : other.Y.CompareTo(this.Y);
        }
        
    }

    private class Node<T> : IComparable<Node<T>>, IEquatable<Node<T>> where T : IComparable<T>, IAdditionOperators<T, T, T>, INumberBase<T>
    {
        public Node(Point<T> point, PipeType pipeType)
        {
            this.Point = point;
            this.PipeType = pipeType;
            this.Distance = long.MaxValue;
            this.Visited = false;
        }

        public int CompareTo(Node<T>? other)
        {
            return other == null ? 1 : other.Point.CompareTo(Point);
        }

        public Point<T> Point { get; init; }
        public PipeType PipeType { get; init; }
        public long Distance { get; set; }
        
        public bool Visited { get; set; }

        public void Deconstruct(out Point<T> point, out PipeType pipeType)
        {
            point = this.Point;
            pipeType = this.PipeType;
        }

        public List<Point<T>> GetConnectedPoints()
        {
            var connectedPoints = new List<Point<T>>();
            switch (PipeType)
            {
                case PipeType.Vertical:
                    connectedPoints.Add(Point with { Y = Point.Y - T.One });
                    connectedPoints.Add(Point with { Y = Point.Y + T.One });
                    break;
                case PipeType.Horizontal:
                    connectedPoints.Add(Point with { X = Point.X - T.One });
                    connectedPoints.Add(Point with { X = Point.X + T.One });
                    break;
                case PipeType.L:
                    connectedPoints.Add(Point with { Y = Point.Y - T.One });
                    connectedPoints.Add(Point with { X = Point.X + T.One });
                    break;
                case PipeType.J:
                    connectedPoints.Add(Point with { Y = Point.Y - T.One });
                    connectedPoints.Add(Point with { X = Point.X - T.One });
                    break;
                case PipeType.Seven:
                    connectedPoints.Add(Point with { Y = Point.Y + T.One });
                    connectedPoints.Add(Point with { X = Point.X - T.One });
                    break;
                case PipeType.F:
                    connectedPoints.Add(Point with { Y = Point.Y + T.One });
                    connectedPoints.Add(Point with { X = Point.X + T.One });
                    break;
            }

            return connectedPoints;
        }

        public bool Equals(Node<T>? other)
        {
            return other != null && other.Point.Equals(Point);
        }

        public override bool Equals(object obj)
        {
            return Equals(obj as Node<T>);
        }

        public override int GetHashCode()
        {
            return Point.GetHashCode();
        }
    }

    public enum PipeType
    {
        Vertical,
        Horizontal,
        L,
        J,
        Seven,
        F
    }

    private class PipeGraph
    {
        internal Point<long> StartingPoint { get; }
        internal SortedSet<Node<long>> Nodes { get; }


        public PipeGraph(Point<long> startingPoint, SortedSet<Node<long>> nodes)
        {
            StartingPoint = startingPoint;
            Nodes = nodes;
        }

        public Node<long>? NodeForPoint(Point<long> point)
        {
            var value = new Node<long>(point, PipeType.Seven);
            var nodes = Nodes.GetViewBetween(value, value);
            
            return nodes.Max;
            
        }

        public Point<long> GetNextPoint()
        {
            return Nodes.Where((node) => node is { Visited: false, Distance: < long.MaxValue })
                .OrderBy(node => node.Distance)
                .Min()
                ?.Point;
        }
    }

    public static long Puzzle1(List<string> inputs, PipeType startingPipeType)
    {
        var graph = ParseMap(inputs, startingPipeType);
        var currentPoint = graph.StartingPoint;

        // var distances = Dijkstra(graph.NodeForPoint(currentPoint), graph);
        //
        // return distances.Values.Sum();

        while (currentPoint != null)
        {
            IterateDj(graph, currentPoint);
            currentPoint = graph.GetNextPoint();
        }
        
        var maxDistance = graph.Nodes.Select((node) => node.Distance)
            .Where((distance) => distance < long.MaxValue)
            .Max();
        return maxDistance;
    }
    
    private static Dictionary<Point<long>, long> Dijkstra(Node<long> start,  PipeGraph graph) {
        var queue = new PriorityQueue<Node<long>, long>();
        queue.Enqueue(start, 0);
        var distances = new Dictionary<Point<long>, long> { { start.Point, 0 } };
        while (queue.Count > 0) {
            var current = queue.Peek();
            long dist = distances[current.Point];
            List<Point<long>> neighbours = current.GetConnectedPoints();
            foreach (Point<long> n in neighbours) {
                long ndist = dist;
                ndist++;
                if (ndist < distances.GetValueOrDefault(n, long.MaxValue)) {
                    distances.Add(n, ndist);
                    var node = graph.NodeForPoint(n);
                    queue.Enqueue(node, ndist);
                }
            }
        }

        return distances;
    }

    private static void IterateDj(PipeGraph graph, Point<long> currentPoint)
    {
        if (currentPoint == null)
        {
            return;
        }
        var currentNode = graph.NodeForPoint(currentPoint);
        if (currentNode == null || currentNode.Visited)
        {
            return;
        }

        currentNode.Visited = true;
        
        var points = currentNode.GetConnectedPoints();

        foreach (var point in points)
        {
            var node = graph.NodeForPoint(point);
            if (node != null)
            {
                var nextDistance = currentNode.Distance + 1;
                if (node.Distance > nextDistance)
                {
                    node.Distance = nextDistance;
                }
            }
        }


    }

    private static PipeGraph ParseMap(List<string> inputs, PipeType startingPipeType)
    {
        var sortedSet = new SortedSet<Node<long>>();

        long rowIndex = 0;
        Point<long> startingPoint = null;
        foreach (var input in inputs)
        {
            long columnIndex = 0;
            foreach (var character in input)
            {
                PipeType? pipeType = ToPipeType(character);
                if (pipeType.HasValue)
                {
                    var node = new Node<long>(new Point<long>( columnIndex,rowIndex), pipeType.Value);
                    sortedSet.Add(node);
                }

                if (character == 'S')
                {
                    startingPoint = new Point<long>(columnIndex,rowIndex);
                }

                columnIndex++;
            }

            rowIndex++;
        }

        var startingNode = new Node<long>(startingPoint, startingPipeType)
        {
            Distance = 0
        };
        sortedSet.Add(startingNode);

        //Figure out adjacent nodes and create a pipe type.  Insert node into sortedSet

        return new PipeGraph(startingPoint, sortedSet);
    }

    private static PipeType? ToPipeType(char character)
    {
        switch (character)
        {
            case '|':
                return PipeType.Vertical;
            case '-':
                return PipeType.Horizontal;
            case 'L':
                return PipeType.L;
            case 'J':
                return PipeType.J;
            case '7':
                return PipeType.Seven;
            case 'F':
                return PipeType.F;
        }

        return null;
    }

    public static long Puzzle2(List<string> inputs)
    {
        return 0;
    }
}