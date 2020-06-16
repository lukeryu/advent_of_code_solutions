fn puzzle1(string: String) -> i32 {
    let mut max_value = 0;

    let phase_settings_list = generate_phase_settings_list();
    println!("{:?}", phase_settings_list);

    for phase_settings in phase_settings_list {
        let result = run_phase(string.clone(), phase_settings);
        if (result > max_value) {
            max_value = result;
        }
    }

    return max_value;
}

fn generate_phase_settings_list() -> Vec<String> {
    return phase_thing(vec![0, 1, 2, 3, 4]);
}

fn generate_feedback_phase_settings_list() -> Vec<String> {
    return phase_thing(vec![5, 6, 7, 8, 9]);
}

fn phase_thing(vec: Vec<i32>) -> Vec<String> {
    if (vec.is_empty()) {
        return vec![String::from("")];
    }
    let mut rows = Vec::<String>::new();
    for num in &vec {
        let mut clonevec = vec.clone();
        clonevec.retain(|vale| vale.clone() != num.clone());
        let output = phase_thing(clonevec);
        output.iter()
            .map(|str| {
                let mut string = num.to_string();
                string.push_str(",");
                string.push_str(str);
                return string;
            })
            .for_each(|str| rows.push(str));
    }
    return rows;
}

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

fn to_parameter_mode(charInput: char) -> ParameterMode {
    if (charInput == '1') {
        return ParameterMode::IMMEDIATE;
    }
    return ParameterMode::POSITION;
}

fn to_op_code(char1: char, char2: char) -> OpCode {
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

fn parse_parameter_op_code(code: &i32) -> ParameterOpCode {
    let mut formatted_string = format!("{:05}", code);
    let mut chars = formatted_string.chars();
    let char1 = chars.next().unwrap();
    let char2 = chars.next().unwrap();
    let char3 = chars.next().unwrap();
    let char4 = chars.next().unwrap();
    let char5 = chars.next().unwrap();
    return ParameterOpCode {
        parameter_1_mode: to_parameter_mode(char3),
        parameter_2_mode: to_parameter_mode(char2),
        parameter_3_mode: to_parameter_mode(char1),
        op_code: to_op_code(char4, char5),
    };
}

fn get_value_at_index(index: usize, mode: ParameterMode, values: &Vec<i32>) -> i32 {
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

fn set_value_at_index(value: i32, index: usize, mode: ParameterMode, values: &mut Vec<i32>) {
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

fn run_phase(instruction_string: String, phase_string: String) -> i32 {
    let mut instructions: Vec<i32> = instruction_string.split(",")
        .map(|s| -> i32 { s.parse::<i32>().unwrap() })
        .collect();

    let phase_order: Vec<i32> = phase_string.split(",")
        .filter(|s| -> bool { s.len() > 0 })
        .map(|s| -> i32 { s.parse::<i32>().unwrap() })
        .collect();

    let mut value = 0;
    for phase in phase_order {
        value = run_program(&mut instructions.clone(), &mut vec![value, phase]);
    }

    return value;
}

fn run_program(instructions: &mut Vec<i32>, input: &mut Vec<i32>) -> i32 {
    // let mut instructions = values.clone();
    let mut output = Vec::new();
    println!("Input: {:?}", input);

    let mut index = 0;
    while index < instructions.len() {
        let op_code = parse_parameter_op_code(&instructions[index]);
        println!("{:?}", &op_code);
        match op_code.op_code {
            OpCode::ADD => {
                println!("{:?}", &instructions[index..index + 4]);

                let value_a = get_value_at_index(index + 1, op_code.parameter_1_mode.clone(), instructions);
                let value_b = get_value_at_index(index + 2, op_code.parameter_2_mode.clone(), instructions);
                set_value_at_index(value_a + value_b, index + 3, op_code.parameter_3_mode.clone(), instructions);
                index += 4;
            }
            OpCode::MULTIPLY => {
                println!("{:?}", &instructions[index..index + 4]);

                let value_a = get_value_at_index(index + 1, op_code.parameter_1_mode.clone(), instructions);
                let value_b = get_value_at_index(index + 2, op_code.parameter_2_mode.clone(), instructions);
                set_value_at_index(value_a * value_b, index + 3, op_code.parameter_3_mode.clone(), instructions);
                index += 4;
            }
            OpCode::SAVE_INPUT_AT => {
                println!("{:?}", &instructions[index..index + 2]);
                match input.pop() {
                    Some(value) => {
                        println!("Popped Value: {:?}", value);
                        set_value_at_index(value, index + 1, op_code.parameter_1_mode.clone(), instructions)
                    }
                    None => { println!("Cannot Read from input") }
                }

                index += 2;
            }
            OpCode::OUTPUT_VALUE_AT => {
                println!("{:?}", &instructions[index..index + 2]);
                let register = get_value_at_index(index + 1, op_code.parameter_1_mode.clone(), instructions);
                output.push(register);
                index += 2;
            }
            OpCode::JUMP_IF_TRUE => {
                println!("{:?}", &instructions[index..index + 3]);
                let value_a = get_value_at_index(index + 1, op_code.parameter_1_mode.clone(), instructions);
                if (value_a != 0) {
                    let value_b = get_value_at_index(index + 2, op_code.parameter_2_mode.clone(), instructions);
                    index = value_b as usize;
                } else {
                    index += 3;
                }
            }
            OpCode::JUMP_IF_FALSE => {
                println!("{:?}", &instructions[index..index + 3]);
                let value_a = get_value_at_index(index + 1, op_code.parameter_1_mode.clone(), instructions);
                if (value_a == 0) {
                    let value_b = get_value_at_index(index + 2, op_code.parameter_2_mode.clone(), instructions);
                    index = value_b as usize;
                } else {
                    index += 3;
                }
            }
            OpCode::LESS_THAN => {
                println!("{:?}", &instructions[index..index + 4]);
                let value_a = get_value_at_index(index + 1, op_code.parameter_1_mode.clone(), instructions);
                let value_b = get_value_at_index(index + 2, op_code.parameter_2_mode.clone(), instructions);
                let save_value: i32;
                if (value_a < value_b) {
                    save_value = 1;
                } else {
                    save_value = 0;
                }
                set_value_at_index(save_value, index + 3, op_code.parameter_3_mode.clone(), instructions);
                index += 4;
            }
            OpCode::EQUALS => {
                println!("{:?}", &instructions[index..index + 4]);
                let value_a = get_value_at_index(index + 1, op_code.parameter_1_mode.clone(), instructions);
                let value_b = get_value_at_index(index + 2, op_code.parameter_2_mode.clone(), instructions);
                let save_value: i32;
                if (value_a == value_b) {
                    save_value = 1;
                } else {
                    save_value = 0;
                }
                set_value_at_index(save_value, index + 3, op_code.parameter_3_mode.clone(), instructions);
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

fn puzzle2(string: String) -> i32 {
    let mut max_value = 0;

    let phase_settings_list = generate_feedback_phase_settings_list();
    println!("{:?}", phase_settings_list);

    for phase_settings in phase_settings_list {
        let result = run_phase(string.clone(), phase_settings);
        if (result > max_value) {
            max_value = result;
        }
    }

    return max_value;
}

#[cfg(test)]
mod tests {
    use crate::utils;
    use crate::day7::{puzzle1, puzzle2, run_phase};

    struct Puzzle1Test {
        test_data: String,
        phase_setting: String,
        expected_result: i32,
    }

    #[test]
    fn test_puzzle_1() {
        // let mut tests: Vec<Puzzle1Test> = Vec::new();
        // tests.push(Puzzle1Test {
        //     test_data: String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
        //     phase_setting: String::from("4,3,2,1,0"),
        //     expected_result: 43210,
        // });
        // tests.push(Puzzle1Test {
        //     test_data: String::from("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
        //     phase_setting: String::from("0,1,2,3,4"),
        //     expected_result: 54321,
        // });
        // tests.push(Puzzle1Test {
        //     test_data: String::from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"),
        //     phase_setting: String::from("1,0,4,3,2"),
        //     expected_result: 65210,
        // });
        //
        // for test in tests {
        //     let result = run_phase(test.test_data, test.phase_setting);
        //     assert_eq!(result, test.expected_result);
        // }


        match utils::read_lines("data/Day7.txt") {
            Ok(lines) => {
                println!("{}", "test");
                let result = puzzle1(lines.get(0).unwrap().clone());
                assert_eq!(result, 34686);
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }

    struct Puzzle2Test {
        test_data: Vec<String>,
        phase_setting: String,
        expected_result: u32,
    }

    #[test]
    fn test_puzzle_2() {
        // let mut tests: Vec<Puzzle2Test> = Vec::new();
        // tests.push(Puzzle2Test {
        //     test_data: vec![String::from("COM)B"),
        //                     String::from("B)C"),
        //                     String::from("C)D"),
        //                     String::from("D)E"),
        //                     String::from("E)F"),
        //                     String::from("B)G"),
        //                     String::from("G)H"),
        //                     String::from("D)I"),
        //                     String::from("E)J"),
        //                     String::from("J)K"),
        //                     String::from("K)L"),
        //                     String::from("K)YOU"),
        //                     String::from("I)SAN")],
        //     expected_result: 4,
        // });
        //
        // match utils::read_lines("data/Day6.txt") {
        //     Ok(lines) => {
        //         tests.push(Puzzle2Test {
        //             test_data: lines,
        //             expected_result: 496,
        //         });
        //         for test in tests {
        //             let result = puzzle2(test.test_data);
        //             assert_eq!(result, test.expected_result);
        //         }
        //     }
        //     Err(error) => {
        //         println!("{}", error);
        //     }
        // }
    }
}