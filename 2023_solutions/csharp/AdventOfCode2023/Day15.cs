namespace AdventOfCode2023;

public static class Day15
{
    public static ulong Puzzle1(List<string> input)
    {
        ulong sum = 0;

        var instructions = input[0].Split(',');
        foreach (var instruction in instructions)
        {
            sum += (ulong)Hash(instruction);
        }

        return sum;
    }

    private static int Hash(string str)
    {
        int currentValue = 0;
        foreach (var character in str)
        {
            var asciiCode = (int)character;
            currentValue += asciiCode;
            currentValue *= 17;
            currentValue %= 256;
        }

        return currentValue;
    }

    private class Lens
    {
        public Lens(string Label, int FocalLength)
        {
            this.Label = Label;
            this.FocalLength = FocalLength;
        }

        public string Label { get; init; }
        public int FocalLength { get; set; }

        public void Deconstruct(out string Label, out int FocalLength)
        {
            Label = this.Label;
            FocalLength = this.FocalLength;
        }
    }

    private class Box
    {
        public readonly List<Lens> Lenses = new();

        public void AddLens(string label, int focalLength)
        {
            bool found = false;
            foreach (var lens in Lenses)
            {
                if (lens.Label.Equals(label))
                {
                    lens.FocalLength = focalLength;
                    found = true;
                    break;
                }
            }

            if (!found)
            {
                Lenses.Add(new Lens(label, focalLength));
            }
        }

        public void RemoveAndShift(string label)
        {
            for (int index = 0; index < Lenses.Count; index++)
            {
                if (Lenses[index].Label.Equals(label))
                {
                    Lenses.RemoveAt(index);
                }
            }
        }
    }

    public static ulong Puzzle2(List<string> input)
    {
        var boxes = new List<Box>(256);

        for (int index = 0; index < 256; index++)
        {
            boxes.Add(new Box());
        }

        var instructions = input[0].Split(',');
        foreach (var instruction in instructions)
        {
            if (instruction.Contains('='))
            {
                var split = instruction.Split('=');
                var label = split[0];
                var focalLength = int.Parse(split[1]);
                var boxNumber = Hash(label);
                boxes[boxNumber].AddLens(label, focalLength);
            }
            else
            {
                var label = instruction.Split('-')[0];
                var boxNumber = Hash(label);

                boxes[boxNumber].RemoveAndShift(label);
            }
        }

        ulong totalFocusingPower = 0;
        for (int boxNumber = 0; boxNumber < boxes.Count; boxNumber++)
        {
            var lenses = boxes[boxNumber].Lenses;
            for (int lensIndex = 0; lensIndex < lenses.Count; lensIndex++)
            {
                int lensFocusingPower = (1 + boxNumber) * (1 + lensIndex) * lenses[lensIndex].FocalLength;
                totalFocusingPower += (ulong)lensFocusingPower;
            }
        }

        return totalFocusingPower;
    }
}