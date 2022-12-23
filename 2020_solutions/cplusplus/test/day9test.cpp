//
// Created by luker on 2/2/2021.
//
#include "day9.hpp"
#include "utils.hpp"
#include <gtest/gtest.h>

TEST(DAY9TEST, PUZZLE1Test1) {
    const std::vector<std::string> values = {"35",
                                             "20",
                                             "15",
                                             "25",
                                             "47",
                                             "40",
                                             "62",
                                             "55",
                                             "65",
                                             "95",
                                             "102",
                                             "117",
                                             "150",
                                             "182",
                                             "127",
                                             "219",
                                             "299",
                                             "277",
                                             "309",
                                             "576"};

    EXPECT_EQ(127, Day9::puzzle1(5, values));
}

TEST(DAY9TEST, PUZZLE1Test2) {
    const auto values = Utils::readLines("/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day9.txt");

    EXPECT_EQ(556543474, Day9::puzzle1(25, values));
}

TEST(DAY9TEST, PUZZLE2Test1) {
    const std::vector<std::string> values = {"35",
                                             "20",
                                             "15",
                                             "25",
                                             "47",
                                             "40",
                                             "62",
                                             "55",
                                             "65",
                                             "95",
                                             "102",
                                             "117",
                                             "150",
                                             "182",
                                             "127",
                                             "219",
                                             "299",
                                             "277",
                                             "309",
                                             "576"};

    EXPECT_EQ(62, Day9::puzzle2(5, values));
}

TEST(DAY9TEST, PUZZLE2Test2) {
    const auto values = Utils::readLines("/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day9.txt");

    EXPECT_EQ(76096372, Day9::puzzle2(25, values));
}

