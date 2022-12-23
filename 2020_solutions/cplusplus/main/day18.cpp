#include "day18.hpp"
#include <algorithm>
#include <numeric>
#include <cctype>
#include <iostream>
#include <stack>
#include <deque>
//
// Created by luker on 2/28/2021.
//

namespace Day18 {
    enum class Operation {
        ADD,
        MULT
    };

    u_long applyOperation(const u_long lhs, const u_long rhs, const Operation op) {
        if (op == Operation::ADD) {
            return lhs + rhs;
        } else if (op == Operation::MULT) {
            return lhs * rhs;
        }

        return lhs + rhs;
    }

    u_long doMath(const std::string &mathString) {
        u_long balance = 0;
        Operation currentOp = Operation::ADD;
        u_long parentDepth = 0;
        u_long parenStartIndex = 0;

        for (auto character = mathString.cbegin();
             character != mathString.cend();
             character++) {

            if (*character == '+') {
                if (parentDepth == 0) {
                    currentOp = Operation::ADD;
                }
            } else if (*character == '*') {
                if (parentDepth == 0) {
                    currentOp = Operation::MULT;
                }
            } else if (*character == '(') {
                if (parentDepth == 0) {
                    parenStartIndex = std::distance(mathString.cbegin(), character + 1);
                }
                parentDepth++;
            } else if (*character == ')') {
                parentDepth--;
                if (parentDepth == 0) {
                    const auto parenEndIndex = std::distance(mathString.cbegin(), character);
                    const auto val = doMath(mathString.substr(parenStartIndex, parenEndIndex - parenStartIndex));
                    balance = applyOperation(balance, val, currentOp);
                    parenStartIndex = 0;
                }
            } else if (isdigit(*character)) {
                if (parentDepth == 0) {
                    const auto val = *character - '0';
                    balance = applyOperation(balance, val, currentOp);
                }
            }
        }
        std::cout << mathString << " = " << balance << std::endl;
        return balance;
    }

    u_long doMath2(const std::string &mathString) {
        u_long balance = 0;
        Operation currentOp = Operation::ADD;
        u_long parentDepth = 0;
        u_long parenStartIndex = 0;


        for (auto character = mathString.cbegin();
             character != mathString.cend();
             character++) {

            if (*character == '+') {
                if (parentDepth == 0) {
                    currentOp = Operation::ADD;
                }
            } else if (*character == '*') {
                if (parentDepth == 0) {
                    currentOp = Operation::MULT;
                }
            } else if (*character == '(') {
                if (parentDepth == 0) {
                    parenStartIndex = std::distance(mathString.cbegin(), character + 1);
                }
                parentDepth++;
            } else if (*character == ')') {
                parentDepth--;
                if (parentDepth == 0) {
                    const auto parenEndIndex = std::distance(mathString.cbegin(), character);
                    const auto val = doMath(mathString.substr(parenStartIndex, parenEndIndex - parenStartIndex));
                    balance = applyOperation(balance, val, currentOp);
                    parenStartIndex = 0;
                }
            } else if (isdigit(*character)) {
                if (parentDepth == 0) {
                    const auto val = *character - '0';
                    balance = applyOperation(balance, val, currentOp);
                }
            }
        }
        std::cout << mathString << " = " << balance << std::endl;
        return balance;
    }

    u_long puzzle1(const std::vector<std::string> &input) {

        u_long sum = 0;
        for (auto inputString = input.cbegin(); inputString != input.cend(); inputString++) {
            sum += doMath(*inputString);
        }

        return sum;
    }

    u_long puzzle2(const std::vector<std::string> &input) {
        u_long sum = 0;
        for (auto inputString = input.cbegin(); inputString != input.cend(); inputString++) {
            sum += doMath2(*inputString);
        }

        return sum;
    }
}
