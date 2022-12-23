//
// Created by luker on 6/5/2021.
//
#include <day22.hpp>
#include <gtest/gtest.h>

TEST(DAY22TEST, PUZZLE1Test1) {

    std::deque<size_t> player1;
    player1.push_back(9);
    player1.push_back(2);
    player1.push_back(6);
    player1.push_back(3);
    player1.push_back(1);

    std::deque<size_t> player2;
    player2.push_back(5);
    player2.push_back(8);
    player2.push_back(4);
    player2.push_back(7);
    player2.push_back(10);

    auto result = Day22::puzzle1(player1, player2);
    EXPECT_EQ(306, result);
}

TEST(DAY22TEST, PUZZLE1Test2) {

    std::deque<size_t> player1;
    player1.push_back(18);
    player1.push_back(50);
    player1.push_back(9);
    player1.push_back(4);
    player1.push_back(25);
    player1.push_back(37);
    player1.push_back(39);
    player1.push_back(40);
    player1.push_back(29);
    player1.push_back(6);
    player1.push_back(41);
    player1.push_back(28);
    player1.push_back(3);
    player1.push_back(11);
    player1.push_back(31);
    player1.push_back(8);
    player1.push_back(1);
    player1.push_back(38);
    player1.push_back(33);
    player1.push_back(30);
    player1.push_back(42);
    player1.push_back(15);
    player1.push_back(26);
    player1.push_back(36);
    player1.push_back(43);

    std::deque<size_t> player2;
    player2.push_back(32);
    player2.push_back(44);
    player2.push_back(19);
    player2.push_back(47);
    player2.push_back(12);
    player2.push_back(48);
    player2.push_back(14);
    player2.push_back(2);
    player2.push_back(13);
    player2.push_back(10);
    player2.push_back(35);
    player2.push_back(45);
    player2.push_back(34);
    player2.push_back(7);
    player2.push_back(5);
    player2.push_back(17);
    player2.push_back(46);
    player2.push_back(21);
    player2.push_back(24);
    player2.push_back(49);
    player2.push_back(16);
    player2.push_back(22);
    player2.push_back(20);
    player2.push_back(27);
    player2.push_back(23);

    auto result = Day22::puzzle1(player1, player2);
    EXPECT_EQ(33561, result);
}

TEST(DAY22TEST, PUZZLE2Test1) {

    std::deque<size_t> player1;
    player1.push_back(9);
    player1.push_back(2);
    player1.push_back(6);
    player1.push_back(3);
    player1.push_back(1);

    std::deque<size_t> player2;
    player2.push_back(5);
    player2.push_back(8);
    player2.push_back(4);
    player2.push_back(7);
    player2.push_back(10);

    auto result = Day22::puzzle2(player1, player2, 1);
    EXPECT_EQ(291, result.count);
}

TEST(DAY22TEST, PUZZLE2Test2) {

    std::deque<size_t> player1;
    player1.push_back(18);
    player1.push_back(50);
    player1.push_back(9);
    player1.push_back(4);
    player1.push_back(25);
    player1.push_back(37);
    player1.push_back(39);
    player1.push_back(40);
    player1.push_back(29);
    player1.push_back(6);
    player1.push_back(41);
    player1.push_back(28);
    player1.push_back(3);
    player1.push_back(11);
    player1.push_back(31);
    player1.push_back(8);
    player1.push_back(1);
    player1.push_back(38);
    player1.push_back(33);
    player1.push_back(30);
    player1.push_back(42);
    player1.push_back(15);
    player1.push_back(26);
    player1.push_back(36);
    player1.push_back(43);

    std::deque<size_t> player2;
    player2.push_back(32);
    player2.push_back(44);
    player2.push_back(19);
    player2.push_back(47);
    player2.push_back(12);
    player2.push_back(48);
    player2.push_back(14);
    player2.push_back(2);
    player2.push_back(13);
    player2.push_back(10);
    player2.push_back(35);
    player2.push_back(45);
    player2.push_back(34);
    player2.push_back(7);
    player2.push_back(5);
    player2.push_back(17);
    player2.push_back(46);
    player2.push_back(21);
    player2.push_back(24);
    player2.push_back(49);
    player2.push_back(16);
    player2.push_back(22);
    player2.push_back(20);
    player2.push_back(27);
    player2.push_back(23);

    auto result = Day22::puzzle2(player1, player2, 1);
    EXPECT_EQ(33561, result.count);
}