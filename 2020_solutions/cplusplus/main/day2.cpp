#include "day2.hpp"
#include <algorithm>
#include <regex>
//
// Created by luker on 1/31/2021.
//
namespace Day2 {
    const std::regex PASSWORD_FORMAT("(\\d+)-(\\d+) (\\w): (\\w+)");

    class Password {
    private:
        long minimum_count;
        long maximum_count;
        char character;
        std::string password;
    public:
        Password(std::string string) {
            std::smatch sm;
            std::regex_match(string, sm, PASSWORD_FORMAT);
            this->password = sm[4];
            this->character = sm[3].str()[0];
            this->maximum_count = std::stol(sm[2]);
            this->minimum_count = std::stol(sm[1]);
        }

        bool isValid() const {
            const auto cha = this->character;
            const auto matchingcharacters
                = std::count_if(this->password.cbegin(),
                                this->password.cend(),
                                [&, cha](char ch) {
                                    return cha == ch;
                                }
            );
            return this->minimum_count <= matchingcharacters && matchingcharacters <= this->maximum_count;
        }

        bool isValid2() const {
            const auto val1 = this->password[this->minimum_count - 1];
            const auto val2 = this->password[this->maximum_count - 1];

            return (val1 == this->character) ^ (val2 == this->character);
        }
    };



    int64_t puzzle1(const std::vector<std::string> &inputValues) {
        return std::count_if(inputValues.begin(), inputValues.end(), [](std::string input) {
            const Password password(input);
            return password.isValid();
        });
    }

    long puzzle2(const std::vector<std::string> &inputValues) {
        return std::count_if(inputValues.begin(), inputValues.end(), [](std::string input) {
            const Password password(input);
            return password.isValid2();
        });
    }
}
