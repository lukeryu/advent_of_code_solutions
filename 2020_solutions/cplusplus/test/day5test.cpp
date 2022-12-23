//
// Created by luker on 2/2/2021.
//
#include "day5.hpp"
#include "utils.hpp"
#include <gtest/gtest.h>

TEST(DAY5TEST, PUZZLE1Test1) {
    const std::vector<std::string> values = {"BFFFBBFRRR"};

    EXPECT_EQ(567, Day5::puzzle1(values));
}

TEST(DAY5TEST, PUZZLE1Test1b) {
    const std::vector<std::string> values = {"FFFBBBFRRR"};

    EXPECT_EQ(119, Day5::puzzle1(values));
}

TEST(DAY5TEST, PUZZLE1Test1c) {
    const std::vector<std::string> values = {"BBFFBBFRLL"};

    EXPECT_EQ(820, Day5::puzzle1(values));
}

TEST(DAY5TEST, PUZZLE1Test1d) {
    const std::vector<std::string> values = {"FBFBBFFRLR"};

    EXPECT_EQ(357, Day5::puzzle1(values));
}

TEST(DAY5TEST, PUZZLE1Test2) {
    const std::vector<std::string> values = Utils::readLines("/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day5.txt");

    EXPECT_EQ(874, Day5::puzzle1(values));
}

TEST(DAY5TEST, PUZZLE2Test2) {
    const std::vector<std::string> values = Utils::readLines("/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day5.txt");

    EXPECT_EQ(594, Day5::puzzle2(values));
}
