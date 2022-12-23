//
// Created by luker on 2/2/2021.
//
#include "day25.hpp"
#include <gtest/gtest.h>

TEST(DAY25TEST, PUZZLE1Test1) {

    EXPECT_EQ(14897079, Day25::puzzle1(5764801, 17807724));
}

TEST(DAY25TEST, PUZZLE1Test2) {

    EXPECT_EQ(4968512, Day25::puzzle1(10604480,4126658));
}

