#include "day2.hpp"
#include "utils.hpp"
#include <gtest/gtest.h>
//
// Created by luker on 1/31/2021.
//
TEST(DAY2TEST, PUZZLE1Test1) {
    const std::vector<std::string> values = {"1-3 a: abcde",
                                             "1-3 b: cdefg",
                                             "2-9 c: ccccccccc"};

    EXPECT_EQ(2, Day2::puzzle1(values));
}

TEST(DAY2TEST, PUZZLE1Test2) {
    const std::vector<std::string> values = Utils::readLines("/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day2.txt");

    EXPECT_EQ(528, Day2::puzzle1(values));
}

TEST(DAY2TEST, PUZZLE2Test1) {
    const std::vector<std::string> values = {"1-3 a: abcde",
                                             "1-3 b: cdefg",
                                             "2-9 c: ccccccccc"};

    EXPECT_EQ(1, Day2::puzzle2(values));
}

TEST(DAY2TEST, PUZZLE2Test2) {
    const std::vector<std::string> values = Utils::readLines("/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day2.txt");

    EXPECT_EQ(497, Day2::puzzle2(values));
}
