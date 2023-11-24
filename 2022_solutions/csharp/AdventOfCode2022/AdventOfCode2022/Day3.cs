namespace AdventOfCode2022;

public class Day3
{
    private static readonly Dictionary<char, ulong> PRIORITY;

    static Day3()
    {
        var hash_map = new Dictionary<char, ulong>();

        hash_map.Add('a', 1);
        hash_map.Add('b', 2);
        hash_map.Add('c', 3);
        hash_map.Add('d', 4);
        hash_map.Add('e', 5);
        hash_map.Add('f', 6);
        hash_map.Add('g', 7);
        hash_map.Add('h', 8);
        hash_map.Add('i', 9);
        hash_map.Add('j', 10);
        hash_map.Add('k', 11);
        hash_map.Add('l', 12);
        hash_map.Add('m', 13);
        hash_map.Add('n', 14);
        hash_map.Add('o', 15);
        hash_map.Add('p', 16);
        hash_map.Add('q', 17);
        hash_map.Add('r', 18);
        hash_map.Add('s', 19);
        hash_map.Add('t', 20);
        hash_map.Add('u', 21);
        hash_map.Add('v', 22);
        hash_map.Add('w', 23);
        hash_map.Add('x', 24);
        hash_map.Add('y', 25);
        hash_map.Add('z', 26);
        hash_map.Add('A', 27);
        hash_map.Add('B', 28);
        hash_map.Add('C', 29);
        hash_map.Add('D', 30);
        hash_map.Add('E', 31);
        hash_map.Add('F', 32);
        hash_map.Add('G', 33);
        hash_map.Add('H', 34);
        hash_map.Add('I', 35);
        hash_map.Add('J', 36);
        hash_map.Add('K', 37);
        hash_map.Add('L', 38);
        hash_map.Add('M', 39);
        hash_map.Add('N', 40);
        hash_map.Add('O', 41);
        hash_map.Add('P', 42);
        hash_map.Add('Q', 43);
        hash_map.Add('R', 44);
        hash_map.Add('S', 45);
        hash_map.Add('T', 46);
        hash_map.Add('U', 47);
        hash_map.Add('V', 48);
        hash_map.Add('W', 49);
        hash_map.Add('X', 50);
        hash_map.Add('Y', 51);
        hash_map.Add('Z', 52);

        PRIORITY = hash_map;
    }

    private static ulong get_priority_cost(string datum)
    {
        var half_length = datum.Length / 2;
        var first_compartment = datum[..half_length];
        var second_compartment = datum[half_length..];

        foreach (var datum_element_char in first_compartment.ToCharArray())
        {
            if (second_compartment.Contains(datum_element_char))
            {
                var priority_element = PRIORITY[datum_element_char];
                return priority_element;
            }
        }

        return 0;
    }

    public static ulong Puzzle1(List<string> data)
    {
        ulong sum = 0;

        foreach (var datum in data)
        {
            sum += get_priority_cost(datum);
        }

        return sum;
    }

    public static ulong Puzzle2(List<string> data)
    {
        ulong sum = 0;

        for (int index = 0; index < data.Count(); index += 3)
        {
            var slice = data.GetRange(index, 3);
            sum += get_badge_const(slice);
        }

        return sum;
    }

    private static ulong get_badge_const(List<string> data)
    {
        var first_pack = data[0];
        var second_pack = data[1];
        var third_pack = data[2];

        foreach (var datum_element_char in first_pack.ToCharArray())
        {
            if (second_pack.Contains(datum_element_char) && third_pack.Contains(datum_element_char))
            {
                var priority_element = PRIORITY[datum_element_char];
                return priority_element;
            }
        }

        return 0;
    }
}