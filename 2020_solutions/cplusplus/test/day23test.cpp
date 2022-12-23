//
// Created by luker on 6/4/2021.
//
#include <day23.hpp>
#include <gtest/gtest.h>

TEST(DAY23TEST, PUZZLE1Test0) {

    const auto input = std::string("389125467");
    auto string = Day23::puzzle1(input, 2);
    EXPECT_EQ("325467891", string);
}

TEST(DAY23TEST, PUZZLE1Test1) {

    const auto input = std::string("389125467");
    auto string = Day23::puzzle1(input, 10);
    EXPECT_EQ("92658374", string);
}

TEST(DAY23TEST, PUZZLE1Test2) {

    const auto input = std::string("389125467");
    auto string = Day23::puzzle1(input, 100);
    EXPECT_EQ("67384529", string);
}

TEST(DAY23TEST, PUZZLE1Test3) {

    const auto input = std::string("158937462");
    auto string = Day23::puzzle1(input, 100);
    EXPECT_EQ("69473825", string);
}

TEST(DAY23TEST, PUZZLE2Test1) {

    const auto input = std::string("389125467");
    auto result = Day23::puzzle2(input, 10000000);
    EXPECT_EQ(149245887792, result);
}

TEST(DAY23TEST, PUZZLE2Test2) {

    const auto input = std::string("158937462");
    auto result = Day23::puzzle2(input, 10000000);
    EXPECT_EQ(96604396189, result);
}