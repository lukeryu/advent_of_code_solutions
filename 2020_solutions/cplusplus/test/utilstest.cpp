//
// Created by luker on 6/6/2021.
//
#include "utils.hpp"
#include <gtest/gtest.h>

TEST(UTILTEST, SPLIT_STRING1) {

    const auto result = Utils::split_string("1 23  4567", ' ');

    EXPECT_EQ(3, result.size());
    EXPECT_EQ("1", result[0]);
    EXPECT_EQ("23", result[1]);
    EXPECT_EQ("4567", result[2]);
}

