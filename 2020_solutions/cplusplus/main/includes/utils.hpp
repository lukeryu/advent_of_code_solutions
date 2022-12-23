//
// Created by luker on 2/1/2021.
//

#ifndef ADVENT_OF_CODE_2020_UTILS_HPP
#define ADVENT_OF_CODE_2020_UTILS_HPP

#include <vector>
#include <string>

namespace Utils {
    void trim(std::string &s);

    std::vector<std::string> readLines(std::string location);

    std::vector<std::string> split_string(const std::string &string, const char toSplitOn);
}

#endif //ADVENT_OF_CODE_2020_UTILS_HPP
