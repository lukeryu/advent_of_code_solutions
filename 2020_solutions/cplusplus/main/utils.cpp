#include <algorithm>
#include <fstream>
#include "utils.hpp"
//
// Created by luker on 2/1/2021.
//

namespace Utils {
    void ltrim(std::string &s) {
        s.erase(s.begin(), std::find_if(s.begin(), s.end(), [](unsigned char ch) {
            return !std::isspace(ch);
        }));
    }

    void rtrim(std::string &s) {
        s.erase(std::find_if(s.rbegin(), s.rend(), [](unsigned char ch) {
            return !std::isspace(ch);
        }).base(), s.end());
    }

    void trim(std::string &s) {
        ltrim(s);
        rtrim(s);
    }

    std::vector<std::string> readLines(std::string location) {

        std::vector<std::string> returnValues;
        std::ifstream infile(location);
        std::string line;
        while (std::getline(infile, line)) {
            trim(line);
            returnValues.push_back(line);
        }
        return returnValues;
    }

    std::vector<std::string> split_string(const std::string &string, const char toSplitOn) {
        std::vector<std::string> results;

        std::string currentString;

        for (const auto &character : string) {
            if (toSplitOn == character) {
                if (!currentString.empty()) {
                    results.push_back(currentString);
                }
                currentString.clear();
            } else {
                currentString.append(std::string(1, character));
            }
        }

        if (!currentString.empty()) {
            results.push_back(currentString);
        }

        return results;
    }

}