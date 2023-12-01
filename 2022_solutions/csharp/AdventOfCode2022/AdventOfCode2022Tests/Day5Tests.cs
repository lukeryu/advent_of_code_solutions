using AdventOfCode2022;

namespace AdventOfCode2022Tests;

public class Day5Tests
{
    private static readonly List<string> TEST_DATA = new List<string> {
        "    [D]    ",
        "[N] [C]    ",
        "[Z] [M] [P]",
        " 1   2   3 ",
        "",
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2"
    };

    [Test]
    public void test_puzzle1() {
        var return_value = Day5.Puzzle1(TEST_DATA);
        Assert.AreEqual(return_value, "CMZ");
    }

    [Test]
    public void test_puzzle1_realdata() {
        var data = TestUtils.GetDataFromFile("Day5.txt");

        var return_value = Day5.Puzzle1(data);
        Assert.AreEqual(return_value, "TLFGBZHCN");
    }

    [Test]
    public void test_puzzle2() {
        var return_value = Day5.Puzzle2(TEST_DATA);
        Assert.AreEqual(return_value, "MCD");
    }

    [Test]
    public void test_puzzle2_realdata() {
        var data = TestUtils.GetDataFromFile("Day5.txt");

        var return_value = Day5.Puzzle2(data);
        Assert.AreEqual(return_value, "QRQFHFWCL");
    }
}