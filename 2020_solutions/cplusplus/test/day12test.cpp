//
// Created by luker on 2/2/2021.
//
#include "day12.hpp"
#include "utils.hpp"
#include <gtest/gtest.h>

TEST(DAY12TEST, PUZZLE1Test1) {
    const std::vector<std::string> values = {"F10",
                                             "N3",
                                             "F7",
                                             "R90",
                                             "F11"};

    EXPECT_EQ(25, Day12::puzzle1(values));
}

TEST(DAY12TEST, PUZZLE1Test2) {
    const auto values = Utils::readLines("/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day12.txt");

    EXPECT_EQ(381, Day12::puzzle1(values));
}

TEST(DAY12TEST, PUZZLE2Test1) {
    const std::vector<std::string> values = {"F10",
                                             "N3",
                                             "F7",
                                             "R90",
                                             "F11"};

    EXPECT_EQ(286, Day12::puzzle2(values));
}

TEST(DAY12TEST, PUZZLE2Test2) {
    const auto values = Utils::readLines("/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day12.txt");

    EXPECT_EQ(28591, Day12::puzzle2(values));
}

