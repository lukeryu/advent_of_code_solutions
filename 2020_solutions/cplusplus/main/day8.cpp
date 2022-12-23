#include "day8.hpp"
#include <algorithm>
//
// Created by luker on 2/3/2021.
//
namespace Day8 {
    enum class Operation {
        ACC,
        JMP,
        NOP
    };

    class Instruction {
        Operation operation;
        int32_t argument;
        bool visited;
    public:
        Instruction(Operation op, int32_t arg) : operation(op), argument(arg), visited(false) {}

        bool isVisited() const {
            return visited;
        };

        void visit() {
            visited = true;
        };

        int32_t getAccumulator() const {
            if (operation == Operation::ACC) {
                return argument;
            }
            return 0;
        };

        size_t getIndexAdjustment() const {
            if (operation == Operation::JMP) {
                return argument;
            }
            return 1;
        };

        bool canSwitch() const {
            return operation != Operation::ACC && argument != 0;
        };

        Operation getOperation() const {
            return operation;
        };

        void setOperation(Operation op) {
            operation = op;
        };

        int32_t getArgument() const {
            return argument;
        };

        void switchOperation() {
            if (operation == Operation::JMP) {
                operation = Operation::NOP;
            } else if (operation == Operation::NOP) {
                operation = Operation::JMP;
            }
        };

    };

    Instruction toInstruction(const std::string &string) {
        auto operationString = string.substr(0, 3);
        Operation operation;
        if (operationString == "acc") {
            operation = Operation::ACC;
        } else if (operationString == "jmp") {
            operation = Operation::JMP;
        } else {
            operation = Operation::NOP;
        }

        const int32_t argument = static_cast<int32_t>(std::stol(string.substr(4)));

        return Instruction(operation, argument);
    }

    struct InstructionResult {
        const int32_t accumulator;
        const bool succeeds;
    };

    InstructionResult runInstructions(std::vector<Instruction> &instructions) {
        int32_t accumulator = 0;
        size_t instructionIndex = 0;

        while (!instructions[instructionIndex].isVisited()) {
            instructions[instructionIndex].visit();

            accumulator += instructions[instructionIndex].getAccumulator();
            instructionIndex += instructions[instructionIndex].getIndexAdjustment();
        }

        return InstructionResult{accumulator, false};
    }

    std::vector<Instruction> parseInstructions(const std::vector<std::string> &inputValues) {
        std::vector<Instruction> instructions;

        for (auto val1Iter = inputValues.begin();
             val1Iter != inputValues.end();
             val1Iter++) {
            instructions.push_back(toInstruction(*val1Iter));
        }

        return instructions;
    }

    int32_t puzzle1(const std::vector<std::string> &inputValues) {
        auto instructions = parseInstructions(inputValues);

        return runInstructions(instructions).accumulator;
    }

    int32_t puzzle2(const std::vector<std::string> &inputValues) {
        const auto originalInstructions = parseInstructions(inputValues);

        for (size_t index = 0; index < originalInstructions.size(); index++) {
            const auto &originalInstruction = originalInstructions[index];
            if (originalInstruction.canSwitch()) {
                std::vector<Instruction> instructionCopy(originalInstructions);
                const Instruction &modifyingInstruction = originalInstructions[index];
                if (modifyingInstruction.getOperation() == Operation::JMP) {
                    instructionCopy[index].switchOperation();// = Instruction(Operation::NOP, modifyingInstruction.getArgument());
//                    modifyingInstruction.setOperation(Operation::NOP);
                } else {
                    instructionCopy[index].switchOperation();// = Instruction(Operation::JMP, modifyingInstruction.getArgument());
//                    modifyingInstruction.setOperation(Operation::JMP);
                }
                const auto result = runInstructions(instructionCopy);
                if (result.succeeds) {
                    return result.accumulator;
                }
            }
        }
        return 0;
    }
}
