#include "day5.hpp"
#include <algorithm>
//
// Created by luker on 2/2/2021.
//
namespace Day5 {
    uint32_t getSeatNumber(const std::string &string) {
        uint32_t seatNumber = 0;

        for (size_t i = 0; i < 7; i++) {
            seatNumber = seatNumber << 1;
            if (string[i] == 'B') {
                seatNumber = seatNumber | 1;
            }
        }

        for (size_t i = 7; i < string.length(); i++) {
            seatNumber = seatNumber << 1;
            if (string[i] == 'R') {
                seatNumber = seatNumber | 1;
            }
        }

        return seatNumber;
    }

    uint32_t puzzle1(const std::vector<std::string> &inputValues) {
        std::vector<uint32_t> seatNumbers(inputValues.size());
        std::transform(inputValues.begin(), inputValues.end(), seatNumbers.begin(), &getSeatNumber);
        return *std::max_element(seatNumbers.begin(), seatNumbers.end());
    }

    uint32_t puzzle2(const std::vector<std::string> &inputValues) {
        std::vector<uint32_t> seatNumbers(inputValues.size());
        std::transform(inputValues.begin(), inputValues.end(), seatNumbers.begin(), &getSeatNumber);

        for (uint32_t value = 1; value < 1024; value++) {
            if (!std::any_of(seatNumbers.begin(), seatNumbers.end(), [value](uint32_t i) {
                return i == value;
            })) {
                if (std::any_of(seatNumbers.begin(), seatNumbers.end(), [value](uint32_t i) {
                    return i == (value + 1);
                }) && std::any_of(seatNumbers.begin(), seatNumbers.end(), [value](uint32_t i) {
                    return i == (value - 1);
                })) {
                    return value;
                }
            }
        }
        return 0;
    }

}