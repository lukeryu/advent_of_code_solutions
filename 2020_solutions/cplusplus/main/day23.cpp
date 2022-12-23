//
// Created by luker on 6/4/2021.
//
#include "day23.hpp"
#include "algorithm.hpp"
#include <algorithm>
#include <string>

namespace Day23 {

    template<typename R>
    bool matches_value(const R testingValue, const R value1, const R value2, const R value3) {
        bool matches = testingValue == value1;
        matches = matches || testingValue == value2;
        matches = matches || testingValue == value3;

        return matches;
    }

    template<typename R>
    void shuffle_cups(Algorithm::CircularQueue<R> &queue, const R minvalue, const R maxvalue) {
        queue.next();
        const auto a = queue.dequeue().value();
        const auto b = queue.dequeue().value();
        const auto c = queue.dequeue().value();
        queue.prev();

        const auto currentCup = queue.currentValue().value();
        auto destinationCup = queue.currentValue().value();


        do {
            if (destinationCup == minvalue) {
                destinationCup = maxvalue;
            } else {
                destinationCup--;
            };
        } while (matches_value(destinationCup, a, b, c));

        queue.setAtValue(destinationCup);
        queue.next();

        queue.enqueue(c);
        queue.enqueue(b);
        queue.enqueue(a);

        queue.setAtValue(currentCup);
        queue.next();
    }

    std::string puzzle1(const std::string &input, const uint64_t iterations) {
        Algorithm::CircularQueue<char> queue;
        std::for_each(input.rbegin(), input.rend(), [&queue](const char character) {
            queue.enqueue(character);
        });

        for (uint64_t iter = 0; iter < iterations; iter++) {
            shuffle_cups<char>(queue, '1', '9');
        }

        queue.setAtValue('1');
        queue.next();

        auto result = std::string();

        while (queue.size() > 1) {
            auto value = queue.dequeue();
            result = result + value.value();
        }

        return result;

    }

    uint64_t puzzle2(const std::string &input, const uint64_t iterations) {
        Algorithm::CircularQueue<uint64_t> queue;

        for (uint64_t value = 1000000; value >= 10; value--) {
            queue.enqueue(value);
        }

        std::for_each(input.rbegin(), input.rend(), [&queue](const char character) {
            const std::string string = {character};
            const auto val = std::stoull(string);
            queue.enqueue(val);
        });

        for (uint64_t iter = 0; iter < iterations; iter++) {
            shuffle_cups<uint64_t>(queue, 1, 1000000);
        }

        queue.setAtValue(1);
        queue.next();
        auto after1 = queue.currentValue().value();
        queue.next();
        auto after2 = queue.currentValue().value();
        return after1 * after2;
    }
}
