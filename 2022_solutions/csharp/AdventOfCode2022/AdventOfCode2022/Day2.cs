namespace AdventOfCode2022;

public class Day2
{
    private enum RPS
    {
        ROCK,
        PAPER,
        SCISSORS
    }

    private record RPSRecord(string code, RPS rps, RPS WinAgainst, RPS LoseTo, ulong score);

    static RPSRecord[] RECORD_VALUES =
    {
        new RPSRecord(
            "A",
            RPS.ROCK,
            RPS.SCISSORS,
            RPS.PAPER,
            1
        ),
        new RPSRecord(
            "X",
            RPS.ROCK,
            RPS.SCISSORS,
            RPS.PAPER,
            1
        ),
        new RPSRecord(
            "B",
            RPS.PAPER,
            RPS.ROCK,
            RPS.SCISSORS,
            2
        ),
        new RPSRecord(
            "Y",
            RPS.PAPER,
            RPS.ROCK,
            RPS.SCISSORS,
            2
        ),
        new RPSRecord(
            "C",
            RPS.SCISSORS,
            RPS.PAPER,
            RPS.ROCK,
            3
        ),
        new RPSRecord(
            "Z",
            RPS.SCISSORS,
            RPS.PAPER,
            RPS.ROCK,
            3
        )
    };

    private static RPSRecord GetRps(string inputString)
    {
        foreach (var record in RECORD_VALUES)
        {
            if (record.code == inputString)
            {
                return record;
            }
        }

        return null;
    }

    private static RPSRecord GetStategicRps(RPSRecord opponentRps, string codeString)
    {
        if (codeString == "X")
        {
            //LOSE
            foreach (var record in RECORD_VALUES)
            {
                if (record.LoseTo == opponentRps.rps)
                {
                    return record;
                }
            }
        }

        if (codeString == "Y")
        {
            //DRAW
            foreach (var record in RECORD_VALUES)
            {
                if (record.rps == opponentRps.rps)
                {
                    return record;
                }
            }
        }

        if (codeString == "Z")
        {
            //WIN
            foreach (var record in RECORD_VALUES)
            {
                if (record.WinAgainst == opponentRps.rps)
                {
                    return record;
                }
            }
        }

        return null;
    }

    public static ulong Puzzle1(IList<string> input)
    {
        ulong sum = 0;
        foreach (var datum in input)
        {
            var strings = datum.Trim().Split(" ");
            var oppString = strings[0];
            var myString = strings[1];
            var opponentRps = GetRps(oppString);
            var myRps = GetRps(myString);
            sum += myRps.score;
            if (myRps.WinAgainst == opponentRps.rps)
            {
                sum += 6;
            }
            else if (myRps.rps == opponentRps.rps)
            {
                sum += 3;
            }
        }

        return sum;
    }

    public static ulong Puzzle2(IList<string> input)
    {
        ulong sum = 0;
        foreach (var datum in input)
        {
            var strings = datum.Trim().Split(" ");
            var oppString = strings[0];
            var myString = strings[1];
            var opponentRps = GetRps(oppString);
            var myRps = GetStategicRps(opponentRps, myString);
            sum += myRps.score;
            if (myRps.WinAgainst == opponentRps.rps)
            {
                sum += 6;
            }
            else if (myRps.rps == opponentRps.rps)
            {
                sum += 3;
            }
        }

        return sum;
    }
}