using System.Text.RegularExpressions;
using Nito.Collections;

namespace AdventOfCode2022;

public delegate void PuzzleSolverDelegate(int a, int b, int c, List<Deque<string>> d);

public class Day5
{
    private static readonly Regex REGEX = new(@"^move (\d+) from (\d+) to (\d+)$");

    static List<Deque<string>> initialize_stacks(List<string> inputArray, int blankIndex)
    {
        var stacks = new List<Deque<string>>(10);
        stacks.Insert(0, new Deque<string>());
        var source = inputArray[blankIndex - 1];
        for (var index = 0; index < source.Length; index++)
        {
            var stackHeader = source[index];
            if (char.IsWhiteSpace(stackHeader))
            {
                continue;
            }

            var stackHeaderNum = stackHeader - '0';
            var currentStack = new Deque<string>();

            for (var stackLevelIndex = blankIndex - 1; stackLevelIndex >= 0; stackLevelIndex--)
            {
                var currentLevel = inputArray[stackLevelIndex];
                var currentValue = currentLevel[index];
                if (!Char.IsWhiteSpace(currentValue))
                {
                    var unwrappedValue = currentValue;
                    if (Char.IsLetterOrDigit(unwrappedValue))
                    {
                        currentStack.AddToFront("" + unwrappedValue);
                    }
                }
            }

            stacks.Insert(stackHeaderNum, currentStack);
        }

        return stacks;
    }

    public static void puzzle1_move_items(int quantity, int source_stack_index, int target_stack_index,
        List<Deque<string>> stacks)
    {
        var temp_stack = new Deque<string>();

        {
            var source_stack = stacks[source_stack_index];
            for (var i = 0; i < quantity; i++)
            {
                var value = source_stack.RemoveFromFront();
                temp_stack.AddToFront(value);
            }
        }

        {
            var target_stack = stacks[target_stack_index];
            for (var i = 0; i < quantity; i++)
            {
                var value = temp_stack.RemoveFromBack();
                target_stack.AddToFront(value);
            }
        }
    }

    public static void puzzle2_move_items(int quantity, int source_stack_index, int target_stack_index,
        List<Deque<string>> stacks)
    {
        var temp_stack = new Deque<string>();

        {
            var source_stack = stacks[source_stack_index];
            for (var i = 0; i < quantity; i++)
            {
                var value = source_stack.RemoveFromFront();
                temp_stack.AddToFront(value);
            }
        }

        {
            var target_stack = stacks[target_stack_index];
            for (var i = 0; i < quantity; i++)
            {
                var value = temp_stack.RemoveFromFront();
                target_stack.AddToFront(value);
            }
        }
    }

    static string DoThePuzzle(List<string> input_array, PuzzleSolverDelegate mover)
    {
        var blank_index = input_array.Count();

        for (int index = 0; index < input_array.Count; index++)
        {
            if (input_array[index].Trim().Length == 0)
            {
                blank_index = index;
            }
        }

        var stacks = initialize_stacks(input_array, blank_index);

        for (int index = blank_index + 1; index < input_array.Count; index++)
        {
            var input_string = input_array[index];
            var tag = REGEX.Match(input_string.Trim());

            var groups = tag.Groups;
            var captures = groups[0].Captures;

            var quantity = int.Parse(groups[1].Captures[0].Value);
            var source_stack_index = int.Parse(groups[2].Captures[0].Value);
            var target_stack_index = int.Parse(groups[3].Captures[0].Value);

            mover(quantity, source_stack_index, target_stack_index, stacks);
        }

        var result = "";

        foreach (var stack in stacks)
        {
            if (stack.Any())
            {
                var top_element = stack.First();
                result = top_element.Aggregate(result, (current, VARIABLE) => current + VARIABLE);
            }
        }

        return result;
    }

    public static string Puzzle1(List<string> data)
    {
        var del = new PuzzleSolverDelegate(puzzle1_move_items);
        return DoThePuzzle(data, del);
    }

    public static string Puzzle2(List<string> data)
    {
        var del = new PuzzleSolverDelegate(puzzle2_move_items);
        return DoThePuzzle(data, del);
    }
}