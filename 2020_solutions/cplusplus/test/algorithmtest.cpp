//
// Created by luker on 6/5/2021.
//
#include "algorithm.hpp"
#include <gtest/gtest.h>

TEST(ALGORITHMTEST, EMPTYTEST)
{
    auto queue = Algorithm::CircularQueue<char>();
    EXPECT_EQ(0, queue.size());
    EXPECT_EQ(true, queue.isEmpty());
    EXPECT_EQ(false, queue.currentValue().has_value());
    EXPECT_EQ(false, queue.contains('A'));
}

TEST(ALGORITHMTEST, ENQUEUETEST)
{
    auto queue = Algorithm::CircularQueue<char>();
    queue.enqueue('1');
    EXPECT_EQ(1, queue.size());
    EXPECT_EQ(false, queue.isEmpty());
    EXPECT_EQ(true, queue.currentValue().has_value());
    EXPECT_EQ('1', queue.currentValue().value());
    EXPECT_EQ(false, queue.contains('A'));
    EXPECT_EQ(true, queue.contains('1'));
    const auto value = queue.dequeue();
    EXPECT_EQ(true, value.has_value());
    EXPECT_EQ('1', value.value());
    EXPECT_EQ(0, queue.size());
    EXPECT_EQ(true, queue.isEmpty());
    EXPECT_EQ(false, queue.currentValue().has_value());
    EXPECT_EQ(false, queue.contains('A'));
}

TEST(ALGORITHMTEST, NEXTPREVIOUSTEST)
{
    auto queue = Algorithm::CircularQueue<char>();
    queue.enqueue('1');
    queue.enqueue('2');
    queue.enqueue('3');
    EXPECT_EQ(3, queue.size());
    EXPECT_EQ(false, queue.isEmpty());
    EXPECT_EQ('3', queue.currentValue().value());
    queue.next();
    EXPECT_EQ('2', queue.currentValue().value());
    queue.next();
    EXPECT_EQ('1', queue.currentValue().value());
    queue.next();
    EXPECT_EQ('3', queue.currentValue().value());
    EXPECT_EQ(false, queue.contains('A'));
    EXPECT_EQ(true, queue.contains('1'));
    EXPECT_EQ(true, queue.contains('2'));
    EXPECT_EQ(true, queue.contains('3'));
    queue.dequeue();
    queue.dequeue();
    queue.dequeue();
    EXPECT_EQ(0, queue.size());
    EXPECT_EQ(true, queue.isEmpty());
    EXPECT_EQ(false, queue.currentValue().has_value());
    EXPECT_EQ(false, queue.contains('A'));
}