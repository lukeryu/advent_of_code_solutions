#include "day17.hpp"
#include "utils.hpp"
#include <gtest/gtest.h>

TEST(DAY17TEST, PUZZLE1Test1) {

    const std::vector<std::string> input = {
            ".#.",
            "..#",
            "###"
    };
    EXPECT_EQ(112, Day17::puzzle1(input, 6));
}

TEST(DAY17TEST, PUZZLE1Test2) {
    const std::vector<std::string> input = Utils::readLines(
            "/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day17.txt");
    EXPECT_EQ(372, Day17::puzzle1(input, 6));
}

TEST(DAY17TEST, PUZZLE2Test1) {

    const std::vector<std::string> input = {
        ".#.",
        "..#",
        "###"
    };
    EXPECT_EQ(848, Day17::puzzle2(input, 6));
}

TEST(DAY17TEST, PUZZLE2Test2) {
    const std::vector<std::string> input = Utils::readLines(
            "/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day17.txt");
    EXPECT_EQ(1896, Day17::puzzle2(input, 6));
}