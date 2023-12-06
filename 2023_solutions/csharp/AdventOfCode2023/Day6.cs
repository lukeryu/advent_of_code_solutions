using System.Text;
using System.Text.RegularExpressions;

namespace AdventOfCode2023;

public static class Day6
{
    public static ulong Puzzle1(List<string> inputs)
    {
        var times = Regex.Matches(inputs[0], "(\\d+)");
        var distances = Regex.Matches(inputs[1], "(\\d+)");
        ulong product = 1;
        for (var gameNumber = 0; gameNumber < times.Count; gameNumber++)
        {
            var time = int.Parse(times[gameNumber].Captures[0].Value);
            var currentDistanceRecord = int.Parse(distances[gameNumber].Captures[0].Value);

            ulong beatCount = 0;

            for (var holdSeconds = 0; holdSeconds <= time; holdSeconds++)
            {
                var distance = (time - holdSeconds) * holdSeconds;

                if (distance > currentDistanceRecord)
                {
                    beatCount++;
                }
            }

            product *= beatCount;
        }

        return product;
    }

    public static ulong Puzzle2(List<string> inputs)
    {
        var times = Regex.Matches(inputs[0], "(\\d+)");
        var distances = Regex.Matches(inputs[1], "(\\d+)");

        var totalTimeString = times.Select((match) => match.Captures[0].Value)
            .Aggregate((current, next) => current + next);
        var totalDistanceString = distances.Select((match) => match.Captures[0].Value)
            .Aggregate((current, next) => current + next);

        var totalTime = ulong.Parse(totalTimeString);
        var totalDistance = ulong.Parse(totalDistanceString);

        ulong beatCount = 0;

        for (ulong holdSeconds = 0; holdSeconds <= totalTime; holdSeconds++)
        {
            var distance = (totalTime - holdSeconds) * holdSeconds;

            if (distance > totalDistance)
            {
                beatCount++;
            }
        }

        return beatCount;
    }
}