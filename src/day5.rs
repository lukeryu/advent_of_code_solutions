const OP_ADD: i32 = 1;
const OP_MULTIPLY: i32 = 2;
const OP_SAVE_INPUT_AT: i32 = 3;
const OP_OUTPUT_VALUE_AT: i32 = 4;
const OP_JUMP_IF_TRUE: i32 = 5;
const OP_JUMP_IF_FALSE: i32 = 6;
const OP_LESS_THAN: i32 = 7;
const OP_EQUALS: i32 = 8;
const OP_END_PROGRAM: i32 = 99;

#[derive(Clone, Debug)]
enum ParameterMode {
    POSITION,
    IMMEDIATE,
}

#[derive(Clone, Debug)]
enum OpCode {
    ADD,
    MULTIPLY,
    SAVE_INPUT_AT,
    OUTPUT_VALUE_AT,
    JUMP_IF_TRUE,
    JUMP_IF_FALSE,
    LESS_THAN,
    EQUALS,
    END_PROGRAM,
    UNKNOWN,
}

#[derive(Clone, Debug)]
struct ParameterOpCode {
    parameter_1_mode: ParameterMode,
    parameter_2_mode: ParameterMode,
    parameter_3_mode: ParameterMode,
    op_code: OpCode,
}

fn toParameterMode(charInput: char) -> ParameterMode {
    if (charInput == '1') {
        return ParameterMode::IMMEDIATE;
    }
    return ParameterMode::POSITION;
}

fn toOpCode(char1: char, char2: char) -> OpCode {
    if (char1 == '0') {
        if (char2 == '1') {
            return OpCode::ADD;
        } else if (char2 == '2') {
            return OpCode::MULTIPLY;
        } else if (char2 == '3') {
            return OpCode::SAVE_INPUT_AT;
        } else if (char2 == '4') {
            return OpCode::OUTPUT_VALUE_AT;
        } else if (char2 == '5') {
            return OpCode::JUMP_IF_TRUE;
        } else if (char2 == '6') {
            return OpCode::JUMP_IF_FALSE;
        } else if (char2 == '7') {
            return OpCode::LESS_THAN;
        } else if (char2 == '8') {
            return OpCode::EQUALS;
        }
    } else if (char1 == '9' && char2 == '9') {
        return OpCode::END_PROGRAM;
    }
    println!("{:?}", char2);
    return OpCode::UNKNOWN;
}

fn parseParameterOpCode(code: &i32) -> ParameterOpCode {
    let mut formattedString = format!("{:05}", code);
    let mut chars = formattedString.chars();
    let char1 = chars.next().unwrap();
    let char2 = chars.next().unwrap();
    let char3 = chars.next().unwrap();
    let char4 = chars.next().unwrap();
    let char5 = chars.next().unwrap();
    return ParameterOpCode {
        parameter_1_mode: toParameterMode(char3),
        parameter_2_mode: toParameterMode(char2),
        parameter_3_mode: toParameterMode(char1),
        op_code: toOpCode(char4, char5),
    };
}

fn getValueAtIndex(index: usize, mode: ParameterMode, values: &Vec<i32>) -> i32 {
    return match mode {
        ParameterMode::POSITION => {
            let index_a = values[index] as usize;
            return values[index_a];
        }
        ParameterMode::IMMEDIATE => {
            return values[index];
        }
    };
}

fn setValueAtIndex(value: i32, index: usize, mode: ParameterMode, values: &mut Vec<i32>) {
    return match mode {
        ParameterMode::POSITION => {
            let index_a = values[index] as usize;
            values[index_a] = value;
        }
        ParameterMode::IMMEDIATE => {
            values[index] = value;
        }
    };
}

fn puzzle_1(input: String, first_input: i32) -> i32 {
    let mut values: Vec<i32> = input.split(",")
        .map(|s| -> i32 { s.parse::<i32>().unwrap() })
        .collect();

    let mut input = vec![first_input];
    let mut output = Vec::new();

    let mut index = 0;
    while index < values.len() {
        let opCode = parseParameterOpCode(&values[index]);
        println!("{:?}", &opCode);
        match opCode.op_code {
            OpCode::ADD => {
                println!("{:?}", &values[index..index + 4]);

                let value_a = getValueAtIndex(index + 1, opCode.parameter_1_mode.clone(), &values);
                let value_b = getValueAtIndex(index + 2, opCode.parameter_2_mode.clone(), &values);
                setValueAtIndex(value_a + value_b, index + 3, opCode.parameter_3_mode.clone(), &mut values);
                index += 4;
            }
            OpCode::MULTIPLY => {
                println!("{:?}", &values[index..index + 4]);

                let value_a = getValueAtIndex(index + 1, opCode.parameter_1_mode.clone(), &values);
                let value_b = getValueAtIndex(index + 2, opCode.parameter_2_mode.clone(), &values);
                setValueAtIndex(value_a * value_b, index + 3, opCode.parameter_3_mode.clone(), &mut values);
                index += 4;
            }
            OpCode::SAVE_INPUT_AT => {
                println!("{:?}", &values[index..index + 2]);
                match input.pop() {
                    Some(value) => setValueAtIndex(value, index + 1, opCode.parameter_1_mode.clone(), &mut values),
                    None => { println!("Cannot Read from input") }
                }

                index += 2;
            }
            OpCode::OUTPUT_VALUE_AT => {
                println!("{:?}", &values[index..index + 2]);
                let register = getValueAtIndex(index + 1, opCode.parameter_1_mode.clone(), &values);
                output.push(register);
                index += 2;
            }
            OpCode::JUMP_IF_TRUE => {
                println!("{:?}", &values[index..index + 3]);
                let value_a = getValueAtIndex(index + 1, opCode.parameter_1_mode.clone(), &values);
                if (value_a != 0) {
                    let value_b = getValueAtIndex(index + 2, opCode.parameter_2_mode.clone(), &values);
                    index = value_b as usize;
                } else {
                    index += 3;
                }
            }
            OpCode::JUMP_IF_FALSE => {
                println!("{:?}", &values[index..index + 3]);
                let value_a = getValueAtIndex(index + 1, opCode.parameter_1_mode.clone(), &values);
                if (value_a == 0) {
                    let value_b = getValueAtIndex(index + 2, opCode.parameter_2_mode.clone(), &values);
                    index = value_b as usize;
                } else {
                    index += 3;
                }
            }
            OpCode::LESS_THAN => {
                println!("{:?}", &values[index..index + 4]);
                let value_a = getValueAtIndex(index + 1, opCode.parameter_1_mode.clone(), &values);
                let value_b = getValueAtIndex(index + 2, opCode.parameter_2_mode.clone(), &values);
                let save_value: i32;
                if (value_a < value_b) {
                    save_value = 1;
                } else {
                    save_value = 0;
                }
                setValueAtIndex(save_value, index + 3, opCode.parameter_3_mode.clone(), &mut values);
                index += 4;
            }
            OpCode::EQUALS => {
                println!("{:?}", &values[index..index + 4]);
                let value_a = getValueAtIndex(index + 1, opCode.parameter_1_mode.clone(), &values);
                let value_b = getValueAtIndex(index + 2, opCode.parameter_2_mode.clone(), &values);
                let save_value: i32;
                if (value_a == value_b) {
                    save_value = 1;
                } else {
                    save_value = 0;
                }
                setValueAtIndex(save_value, index + 3, opCode.parameter_3_mode.clone(), &mut values);
                index += 4;
            }
            OpCode::END_PROGRAM => break,
            _ => {
                index += 4;
            }
        }
    }
    // let x = output.join(",");
    println!("{:?}", output);
    return output.last().unwrap().clone();
}

fn puzzle_2(input: String) -> i32 {
    return puzzle_1(input, 5);
}

#[cfg(test)]
mod tests {
    use crate::utils;
    use crate::day5::{puzzle_1, puzzle_2};

    struct Puzzle1Test {
        test_data: String,
        expected_result: i32,
    }

    #[test]
    fn test_puzzle_1() {
        let mut tests = Vec::new();
        match utils::read_lines("data/Day5.txt") {
            Ok(read_file_lines) => {
                let input_line: String = read_file_lines.get(0).unwrap().clone();

                tests.push(Puzzle1Test {
                    test_data: input_line,
                    expected_result: 7566643,
                });
                for test in tests {
                    let result = puzzle_1(test.test_data, 1);
                    assert_eq!(result, test.expected_result);
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }

    struct Puzzle2Test {
        test_data: String,
        expected_result: i32,
    }

    #[test]
    fn test_puzzle_2() {
        let mut tests: Vec<Puzzle2Test> = Vec::new();
        match utils::read_lines("data/Day5.txt") {
            Ok(lines) => {
                let lines_mut = lines.clone();

                let input_line = lines_mut.get(0).unwrap().clone();

                tests.push(Puzzle2Test {
                    test_data: lines_mut.get(0).unwrap().clone(),
                    expected_result: 9265694,
                });
                for test in tests {
                    let result = puzzle_2(test.test_data);
                    assert_eq!(result, test.expected_result);
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}