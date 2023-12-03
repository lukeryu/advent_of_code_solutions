using System.Text.RegularExpressions;

namespace AdventOfCode2023;

public static partial class Day2
{
    [GeneratedRegex(@"((\d+) ([a-z]+))")]
    private static partial Regex GetCubeColorRegex();

    private const int MaxRedCubes = 12;
    private const int MaxGreenCubes = 13;
    private const int MaxBlueCubes = 14;

    private record Game(string Id, List<Grab> Grabs);

    private record Grab(uint Red, uint Green, uint Blue);

    public static ulong Puzzle1(IEnumerable<string> inputLines)
    {
        var gameIdList =
            (from line in inputLines select ParseGame(line) into game where GameIsValid(game) select game.Id).ToList();

        var sum = gameIdList.Select(long.Parse).Sum();
        return (ulong)sum;
    }

    private static Game ParseGame(string line)
    {
        var stuff = line.Split(":");
        var gameAndId = stuff[0];
        var allGrabs = stuff[1];

        var grabArray = allGrabs.Split(";");
        var parsedGrabs = grabArray.Select(ParseGrabObject).ToList();

        return new Game(gameAndId[5..], parsedGrabs);
    }

    private static Grab ParseGrabObject(string grabString)
    {
        uint red = 0;
        uint blue = 0;
        uint green = 0;

        var grabMatches = GetCubeColorRegex().Matches(grabString);

        foreach (Match match in grabMatches)
        {
            var group = match.Groups;

            var numberOfCubes = group[2].Captures[0].Value;
            var color = group[3].Captures[0].Value;
            var cubes = uint.Parse(numberOfCubes);
            switch (color)
            {
                case "blue":
                    blue += cubes;
                    break;
                case "red":
                    red += cubes;
                    break;
                case "green":
                    green += cubes;
                    break;
            }
        }

        return new Grab(red, green, blue);
    }

    private static bool GameIsValid(Game game)
    {
        return game.Grabs.All(grab => grab is { Red: <= MaxRedCubes, Blue: <= MaxBlueCubes, Green: <= MaxGreenCubes });
    }

    private static ulong CalculateGameScore(Game game)
    {
        uint red = 0;
        uint green = 0;
        uint blue = 0;
        foreach (var grab in game.Grabs)
        {
            if (grab.Red > 0 && grab.Red > red)
            {
                red = grab.Red;
            }

            if (grab.Green > 0 && grab.Green > green)
            {
                green = grab.Green;
            }

            if (grab.Blue > 0 && grab.Blue > blue)
            {
                blue = grab.Blue;
            }
        }


        return red * green * blue;
    }

    public static ulong Puzzle2(IEnumerable<string> inputLines)
    {
        return inputLines.Select(ParseGame).Aggregate<Game?, ulong>(0, (current, game) => current + CalculateGameScore(game));
    }
}