#include "day9.hpp"
#include <deque>
#include <algorithm>
#include <numeric>
//
// Created by luker on 2/2/2021.
//

namespace Day9 {
    uint32_t toUint32_t(const std::string string) {
        return static_cast<uint32_t>(std::stoul(string));
    }

    bool findSum(const uint32_t value, const std::deque<uint32_t> &preambleValues) {
        for (auto val1Iter = preambleValues.begin();
             val1Iter != preambleValues.end();
             val1Iter++) {
            for (auto val2Iter = preambleValues.begin();
                 val2Iter != preambleValues.end();
                 val2Iter++) {
                if (val1Iter != val2Iter) {
                    const auto sum = *val1Iter + *val2Iter;
                    if (sum == value) {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    uint32_t puzzle1(const size_t preamble, const std::vector<std::string> &inputValues) {
        std::vector<uint32_t> values(inputValues.size());
        std::transform(inputValues.begin(), inputValues.end(), values.begin(), *toUint32_t);
        std::deque<uint32_t> preambleValues;

        for (size_t i = 0; i < preamble; i++) {
            preambleValues.push_back(values[i]);
        }

        for (size_t i = preamble; i < values.size(); i++) {
            uint32_t val = values[i];
            const auto found_sum = findSum(val, preambleValues);
            if (found_sum) {
                preambleValues.pop_front();
                preambleValues.push_back(val);
            } else {
                return val;
            }
        }

        return 0;
    }

    uint32_t puzzle2(const size_t preamble, const std::vector<std::string> &inputValues) {
        std::vector<uint32_t> values(inputValues.size());
        std::transform(inputValues.begin(), inputValues.end(), values.begin(), *toUint32_t);
        const uint32_t invalidNumber = puzzle1(preamble, inputValues);
        const auto invalidNumberIter = std::find(values.begin(), values.end(), invalidNumber);
        const auto invalid_number_index = std::distance(values.begin(), invalidNumberIter);

        for (auto size = 2; size < 20; size++) {
            for (auto start_index = 0; start_index < (invalid_number_index - size); start_index++) {
                    const uint32_t sum = std::accumulate(values.begin() + start_index, values.begin() + start_index + size, 0);

                    if (sum == invalidNumber) {
                        const auto range_values = std::minmax_element(values.begin() + start_index, values.begin() + start_index + size);

                        return *range_values.second + *range_values.first;
                    }

                }

        }

        return 0;
    }
}

