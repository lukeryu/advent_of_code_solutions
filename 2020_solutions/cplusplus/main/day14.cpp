//
// Created by luker on 2/4/2021.
//
#include "day14.hpp"
#include <regex>
#include <cmath>
#include <bitset>

namespace Day14 {
    const std::regex MEM_FORMAT("mem\\[(\\d+)\\] = (\\d+)");

    uint64_t puzzle1(const std::vector<std::string> &inputValues) {
        uint64_t andMask = 0;
        uint64_t orMask = 0;
        std::map<size_t, uint64_t> values;

        for (auto it = inputValues.cbegin();
             it != inputValues.cend();
             it++) {
            if ((*it)[1] == 'e') {
                std::smatch sm;
                std::regex_match(*it, sm, MEM_FORMAT);

                const auto arrayIndex = static_cast<size_t>(std::stoull(sm[1].str()));
                auto value = static_cast<uint64_t>(std::stoull(sm[2].str()));
                value = value & andMask;
                value = value | orMask;

                values[arrayIndex] = value;

            } else {
                const auto mask = (*it).substr(7);
                andMask = 0;
                orMask = 0;

                size_t index = 0;
                for (auto charaterIter = mask.crbegin();
                     charaterIter != mask.crend();
                     charaterIter++) {

                    if (*charaterIter == '0') {
                        orMask |= (1 << index);
                    } else if (*charaterIter == '1') {
                        andMask |= (1 << index);
                    }

                    index++;
                }
                andMask = ~andMask;
            }
        }

        uint64_t sum = 0;
        for (auto it = values.cbegin();
             it != values.cend();
             it++) {
            sum += it->second;
        }

        return sum;
    }

    uint64_t puzzle2(const std::vector<std::string> &inputValues) {
        std::string mask;
        uint64_t xCountSq(0);

        std::map<uint64_t, uint64_t> values;

        for (auto it = inputValues.cbegin();
             it != inputValues.cend();
             it++) {
            if ((*it)[1] == 'e') {
                std::smatch sm;
                std::regex_match(*it, sm, MEM_FORMAT);

                auto arrayIndex = static_cast<uint64_t>(std::stoull(sm[1].str()));
                const auto value = static_cast<uint64_t>(std::stoull(sm[2].str()));

                std::string maskedIndex = std::bitset<36>(arrayIndex).to_string();

                for (auto charIter = maskedIndex.begin();
                     charIter != maskedIndex.end();
                     charIter++) {

                    const auto index =  std::distance(maskedIndex.begin(), charIter);
                    if (mask[index] != '0') {
                        *charIter = mask[index];
                    }
                }


                for (uint64_t i = 0; i < xCountSq; i++) {
                    const auto bitmaskString = std::bitset<36>(i).to_string();
                    auto appliedString(maskedIndex);

                    uint64_t bitmaskIndex = 35;
                    for (auto charIter = appliedString.rbegin();
                         charIter != appliedString.rend();
                         charIter++) {

                        if (*charIter == 'X') {
                            *charIter = bitmaskString[bitmaskIndex];
                            bitmaskIndex--;
                        }
                    }
                    const auto appliedIndex = static_cast<uint64_t>(std::stoull(appliedString, nullptr, 2));
                    values[appliedIndex] = value;
                }

            } else {
                mask = (*it).substr(7);
                const uint64_t xCount = std::count_if(mask.cbegin(), mask.cend(), [](const char ch) {
                    return ch == 'X';
                });
                xCountSq = 1 << xCount;
            }
        }

        uint64_t sum = 0;
        for (auto it = values.cbegin();
             it != values.cend();
             it++) {
            sum += it->second;
        }

        return sum;
    }
}
