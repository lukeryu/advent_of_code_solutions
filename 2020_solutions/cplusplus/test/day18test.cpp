#include "day18.hpp"
#include "utils.hpp"
#include <gtest/gtest.h>

TEST(DAY18TEST, PUZZLE1Test1) {

    const std::vector<std::string> input = {
            "2 * 3 + (4 * 5)"
    };
    EXPECT_EQ(26, Day18::puzzle1(input));
}

TEST(DAY18TEST, PUZZLE1Test2) {

    const std::vector<std::string> input = {
            "5 + (8 * 3 + 9 + 3 * 4 * 3)"
    };
    EXPECT_EQ(437, Day18::puzzle1(input));
}

TEST(DAY18TEST, PUZZLE1Test3) {

    const std::vector<std::string> input = {
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
    };
    EXPECT_EQ(12240, Day18::puzzle1(input));
}

TEST(DAY18TEST, PUZZLE1Test4) {

    const std::vector<std::string> input = {
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
    };
    EXPECT_EQ(13632, Day18::puzzle1(input));
}

TEST(DAY18TEST, PUZZLE1Test5) {

    const std::vector<std::string> input = {
            "2 * 3 + (4 * 5)",
            "5 + (8 * 3 + 9 + 3 * 4 * 3)",
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
    };
    EXPECT_EQ(26335, Day18::puzzle1(input));
}

TEST(DAY18TEST, PUZZLE1Test6) {
    const std::vector<std::string> input = Utils::readLines(
            "/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day18.txt");
    EXPECT_EQ(14208061823964, Day18::puzzle1(input));
}

TEST(DAY18TEST, PUZZLE1Test7) {

    const std::vector<std::string> input = {
            "1 + (2 * 3) + (4 * (5 + 6))"
    };
    EXPECT_EQ(51, Day18::puzzle1(input));
}

TEST(DAY18TEST, PUZZLE1Test8) {

    const std::vector<std::string> input = {
            "1 + 2 * 3 + 4 * 5 + 6"
    };
    EXPECT_EQ(71, Day18::puzzle1(input));
}

TEST(DAY18TEST, PUZZLE2Test1) {

    const std::vector<std::string> input = {
            "1 + 2 * 3 + 4 * 5 + 6"
    };
    EXPECT_EQ(231, Day18::puzzle2(input));
}

TEST(DAY18TEST, PUZZLE2Test2) {

    const std::vector<std::string> input = {
            "1 + (2 * 3) + (4 * (5 + 6))"
    };
    EXPECT_EQ(51, Day18::puzzle2(input));
}

TEST(DAY18TEST, PUZZLE2Test3) {

    const std::vector<std::string> input = {
            "2 * 3 + (4 * 5)"
    };
    EXPECT_EQ(46, Day18::puzzle2(input));
}

TEST(DAY18TEST, PUZZLE2Test4) {

    const std::vector<std::string> input = {
            "5 + (8 * 3 + 9 + 3 * 4 * 3)"
    };
    EXPECT_EQ(1445, Day18::puzzle2(input));
}

TEST(DAY18TEST, PUZZLE2Test5) {

    const std::vector<std::string> input = {
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
    };
    EXPECT_EQ(669060, Day18::puzzle2(input));
}

TEST(DAY18TEST, PUZZLE2Test6) {

    const std::vector<std::string> input = {
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
    };
    EXPECT_EQ(23340, Day18::puzzle2(input));
}

TEST(DAY18TEST, PUZZLE2Test7) {

    const std::vector<std::string> input = {
            "1 + 2 * 3 + 4 * 5 + 6",
            "1 + (2 * 3) + (4 * (5 + 6))",
            "2 * 3 + (4 * 5)",
            "5 + (8 * 3 + 9 + 3 * 4 * 3)",
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
    };
    EXPECT_EQ(694173, Day18::puzzle2(input));
}

TEST(DAY18TEST, PUZZLE2Test8) {
    const std::vector<std::string> input = Utils::readLines(
            "/mnt/c/Users/luker/CLionProjects/advent_of_code_2020/data/Day18.txt");
    EXPECT_EQ(1896, Day18::puzzle2(input));
}