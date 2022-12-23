#include "day25.hpp"
//
// Created by luker on 2/2/2021.
//
namespace Day25 {
    const int64_t LOOP_DIVISOR = 20201227;
    int64_t findLoopSize(int64_t publicKey) {
        int64_t value = 1;
        int64_t subjectNumber = 7;
        int64_t loopCount = 0;
        while (value != publicKey) {
            value = value * subjectNumber;
            value = value % LOOP_DIVISOR;

            loopCount++;
        }

        return loopCount;
    }

    int64_t run_iterations(int64_t subjectNumber, int64_t loopCount) {
        int64_t value = 1;
        for (auto i = 0; i< loopCount; i++) {
            value = value * subjectNumber;
            value = value % LOOP_DIVISOR;
        }

        return value;
    }

    int64_t puzzle1(int64_t cardPublicKey, int64_t doorPublicKey) {
        auto cardLoopSize = findLoopSize(cardPublicKey);
        return run_iterations(doorPublicKey, cardLoopSize);
    }
}

