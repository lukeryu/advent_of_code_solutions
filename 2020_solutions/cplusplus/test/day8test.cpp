//
// Created by luker on 2/2/2021.
//
#include "day8.hpp"
#include "utils.hpp"
#include <gtest/gtest.h>

TEST(DAY8TEST, PUZZLE1Test1) {
    const std::vector<std::string> values = {"nop +0",
                                             "acc +1",
                                             "jmp +4",
                                             "acc +3",
                                             "jmp -3",
                                             "acc -99",
                                             "acc +1",
                                             "jmp -4",
                                             "acc +6"};

    EXPECT_EQ(5, Day8::puzzle1(values));
}

TEST(DAY8TEST, PUZZLE1Test2) {
    const auto values = Utils::readLines("/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day8.txt");

    EXPECT_EQ(1859, Day8::puzzle1(values));
}

//TEST(DAY8TEST, PUZZLE2Test1) {
//    const std::vector<std::string> values = {"nop +0",
//                                             "acc +1",
//                                             "jmp +4",
//                                             "acc +3",
//                                             "jmp -3",
//                                             "acc -99",
//                                             "acc +1",
//                                             "jmp -4",
//                                             "acc +6"};
//
//    EXPECT_EQ(62, Day8::puzzle2(values));
//}
//
//TEST(DAY8TEST, PUZZLE2Test2) {
//    const auto values = Utils::readLines("/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day8.txt");
//
//    EXPECT_EQ(76096372, Day8::puzzle2(values));
//}

