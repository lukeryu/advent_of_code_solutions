//
// Created by luker on 2/4/2021.
//
//
// Created by luker on 2/2/2021.
//
#include "day14.hpp"
#include "utils.hpp"
#include <gtest/gtest.h>

TEST(DAY14TEST, PUZZLE1Test1) {
    const std::vector<std::string> values = {"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
                                             "mem[8] = 11",
                                             "mem[7] = 101",
                                             "mem[8] = 0"};

    EXPECT_EQ(165, Day14::puzzle1(values));
}

TEST(DAY14TEST, PUZZLE1Test2) {
    const auto values = Utils::readLines("/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day14.txt");

    EXPECT_EQ(9967721333886, Day14::puzzle1(values));
}

TEST(DAY14TEST, PUZZLE2Test1) {
    const std::vector<std::string> values = {"mask = 000000000000000000000000000000X1001X",
                                             "mem[42] = 100",
                                             "mask = 00000000000000000000000000000000X0XX",
                                             "mem[26] = 1",};

    EXPECT_EQ(208, Day14::puzzle2(values));
}

TEST(DAY14TEST, PUZZLE2Test2) {
    const auto values = Utils::readLines("/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day14.txt");

    EXPECT_EQ(4355897790573, Day14::puzzle2(values));
}


