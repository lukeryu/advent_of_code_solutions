namespace AdventOfCode2023;

public static class Day24
{
    private record Intersection(double px, double py, double pz, double lhst, double rhst);

    private record Ray(long px, long py, long pz, long vx, long vy, long vz)
    {
        public Intersection? Intersects2D(Ray otherRay)
        {
            throw new NotImplementedException();
        }
    }

    private static List<Ray> ParseRays(List<string> inputs)
    {
        List<Ray> rays = new List<Ray>();

        foreach (var input in inputs)
        {
            var rayParts = input.Split('@');
            var positions = rayParts[0];
            var velocities = rayParts[1];

            var positionComps = positions.Split(',');
            var velocityComps = velocities.Split(',');

            rays.Add(new Ray(
                long.Parse(positionComps[0].Trim()),
                long.Parse(positionComps[1].Trim()),
                long.Parse(positionComps[2].Trim()),
                long.Parse(velocityComps[0].Trim()),
                long.Parse(velocityComps[1].Trim()),
                long.Parse(velocityComps[2].Trim())));
        }

        return rays;
    }

    public static ulong Puzzle1(List<string> inputs, long min, long max)
    {
        var rays = ParseRays(inputs);

        ulong intersectionCount = 0;
        for (var i = 0; i < rays.Count; i++)
        {
            for (var j = i + 1; j < rays.Count; j++)
            {
                var iRay = rays[i];
                var jRay = rays[j];
                var intersection = iRay.Intersects2D(jRay);

                if (intersection is { lhst: >= 0, rhst: >= 0 } && intersection.px >= min &&
                    intersection.px <= max && intersection.py >= min && intersection.py <= max)
                {
                    intersectionCount++;
                }
            }
        }

        return intersectionCount;
    }

    public static ulong Puzzle2(List<string> inputs)
    {
        return 0;
    }
}