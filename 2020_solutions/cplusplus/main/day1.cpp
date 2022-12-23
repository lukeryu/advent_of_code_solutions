#include <algorithm>
#include "day1.hpp"
//
// Created by luker on 12/1/2020.
//

namespace Day1 {
    long puzzle1(const std::vector<int> &inputValues) {

        const int sum_to_find = 2020;

        for (const auto &item : inputValues) {
            const int other_value = sum_to_find - item;
            const auto &it = std::find(inputValues.begin(), inputValues.end(), other_value);
            if (it != inputValues.end()) {
                return item * other_value;
            }
        }

        return -1;

    }

    long puzzle2(const std::vector<int> &inputValues) {

        for (const auto &firstValue : inputValues) {
            for (const auto &secondValue : inputValues) {
                for (const auto &thirdValue : inputValues) {
                    if (firstValue + secondValue + thirdValue == 2020) {
                        return firstValue * secondValue * thirdValue;
                    }
                }
            }
        }

        return 0;

    }
}