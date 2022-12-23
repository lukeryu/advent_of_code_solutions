//
// Created by luker on 6/6/2021.
//
#include "day19.hpp"
#include <regex>
#include "utils.hpp"

namespace Day19 {
    std::string regexForRule(const std::vector<std::string> &rules, size_t row);

    std::string lookupRules(const std::vector<std::string> &rules, const std::string &string) {
        const auto split_strings = Utils::split_string(string, ' ');

        std::string result;
        for (const auto &item : split_strings) {
            const auto index = std::stoll(item);
            result += regexForRule(rules, index);
        }

        return result;
    }

    std::string regexForRule(const std::vector<std::string> &rules, size_t row) {
        const std::string &rule = rules[row];
        if (rule == "\"a\"") {
            return "a";
        }
        if (rule == "\"b\"") {
            return "b";
        }
        const auto pipe_index = rule.find('|');
        if (pipe_index != std::string::npos) {
            const auto lhs = lookupRules(rules, rule.substr(0, pipe_index));
            const auto rhs = lookupRules(rules, rule.substr(pipe_index + 1));
            return "(?:" + lhs + "|" + rhs + ")";
        } else {
            return lookupRules(rules, rule);
        }

    }

    std::regex buildRegex(const std::vector<std::string> &rules) {
        const std::string regexRule = regexForRule(rules, 0);
        return std::regex("^" + regexRule + "$");
    }

    std::vector<std::string> filterRules(const std::vector<std::string> &input) {
        std::vector<std::string> rules;
        rules.resize(300);

        for (const auto &line : input) {
            if (!line.empty() && line[0] != 'a' && line[0] != 'b') {
                const auto colon_index = line.find(':');
                const auto rule_index_string = line.substr(0, colon_index);
                const auto rule_index = std::stoll(rule_index_string);
                auto rul = line.substr(colon_index + 2);
                Utils::trim(rul);
                rules[rule_index] = rul;
            }
        }

        return rules;
    }

    std::vector<std::string> filterMessages(const std::vector<std::string> &input) {
        std::vector<std::string> messages;

        for (const auto &line : input) {
            if (!line.empty() && (line[0] == 'a' || line[0] == 'b')) {
                messages.push_back(line);
            }
        }

        return messages;
    }

    uint64_t puzzle1(const std::vector<std::string> &input) {
        const std::vector<std::string> rules = filterRules(input);

        const std::regex regex = buildRegex(rules);

        const std::vector<std::string> messages = filterMessages(input);

        uint64_t count = 0;
        for (const auto &message : messages) {
            if (std::regex_match(message, regex)) {
                count++;
            }
        }

        return count;
    }

    std::tuple<std::regex, std::regex, std::regex> buildRegexPuzzle2(const std::vector<std::string> &rules) {
        const auto regexRule42 = regexForRule(rules, 42);
        const auto regex42 = std::regex(regexRule42);

        const auto regexRule31 = regexForRule(rules, 31);
        const auto regex31 = std::regex(regexRule31);

        const std::string lineRegexString = "^" + regexRule42 + "+" + regexRule31 + "+" + "$";
        const auto lineRegex = std::regex(lineRegexString);

        return {lineRegex, regex42, regex31};
    }

    uint64_t puzzle2(const std::vector<std::string> &input) {
        const std::vector<std::string> rules = filterRules(input);

        const auto [regex, regex42, regex31] = buildRegexPuzzle2(rules);

        const std::vector<std::string> messages = filterMessages(input);

        uint64_t count = 0;
        for (const auto &message : messages) {
            if (std::regex_match(message, regex)) {
                const auto begin42 = std::sregex_iterator(message.begin(), message.end(), regex42);
                const auto end42 = std::sregex_iterator();

                const auto count42 = std::distance(begin42, end42);


                const auto begin31 = std::sregex_iterator(message.begin(), message.end(), regex31);
                const auto end31 = std::sregex_iterator();

                const auto count31 = std::distance(begin31, end31);

                if (count42 >= count31 + 1) {
                    count++;
                }

            }
        }

        return count;
    }

}
