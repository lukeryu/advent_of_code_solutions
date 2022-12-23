#include "day16.hpp"
#include "utils.hpp"
#include <gtest/gtest.h>

TEST(DAY16TEST, PUZZLE1Test1) {

    const std::vector<std::string> input = {
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
            "",
            "your ticket:",
            "7,1,14",
            "",
            "nearby tickets:",
            "7,3,47",
            "40,4,50",
            "55,2,20",
            "38,6,12"
    };
    EXPECT_EQ(71, Day16::puzzle1(input));
}

TEST(DAY16TEST, PUZZLE1Test2) {
    const std::vector<std::string> input = Utils::readLines(
            "/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day16.txt");
    EXPECT_EQ(372, Day16::puzzle1(input));
}

TEST(DAY16TEST, PUZZLE2Test1) {

    const std::vector<std::string> input = {
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
            "",
            "your ticket:",
            "7,1,14",
            "",
            "nearby tickets:",
            "7,3,47",
            "40,4,50",
            "55,2,20",
            "38,6,12"
    };
    EXPECT_EQ(848, Day16::puzzle2(input));
}

TEST(DAY16TEST, PUZZLE2Test2) {
    const std::vector<std::string> input = Utils::readLines(
            "/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day16.txt");
    EXPECT_EQ(1896, Day16::puzzle2(input));
}

