const OP_ADD: i32 = 1;
const OP_MULTIPLY: i32 = 2;
const OP_SAVE_INPUT_AT: i32 = 3;
const OP_OUTPUT_VALUE_AT: i32 = 4;
const OP_END_PROGRAM: i32 = 99;

fn puzzle_1(input: String) -> i32 {
    let mut values: Vec<i32> = input.split(",")
        .map(|s| -> i32 { s.parse::<i32>().unwrap() })
        .collect();

    let mut index = 0;
    while index < values.len() {
        match values[index] {
            OP_ADD => {
                let index_a = values[index + 1] as usize;
                let index_b = values[index + 2] as usize;
                let index_result = values[index + 3] as usize;
                let value_a = values[index_a];
                let value_b = values[index_b];
                values[index_result] = value_a + value_b;
            }
            OP_MULTIPLY => {
                let index_a = values[index + 1] as usize;
                let index_b = values[index + 2] as usize;
                let index_result = values[index + 3] as usize;
                let value_a = values[index_a];
                let value_b = values[index_b];
                values[index_result] = value_a * value_b;
            }
            OP_END_PROGRAM => break,
            _ => {}
        }

        index += 4;
    }
    return values[0];
}

fn puzzle_2(input: String, desired_output: i32) -> i32 {
    let mut values: Vec<i32> = input.split(",")
        .map(|s| -> i32{ s.parse::<i32>().unwrap() })
        .collect();

    for noun in 1..99 {
        for verb in 1..99 {
            values[1] = noun.clone();
            values[2] = verb.clone();
            let string_vec: Vec<String> = values.iter()
                .map(|i| -> String { return i.to_string(); })
                .collect();

            let input1 = string_vec
                .join(",");

            let output = puzzle_1(input1);
            if output == desired_output {
                return &noun * 100 + &verb;
            }
        }
    }
    return 0;
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
        let mut tests: Vec<Puzzle1Test> = Vec::new();
        tests.push(Puzzle1Test {
            test_data: String::from("1,0,0,0,99"),
            expected_result: 2,
        });
        tests.push(Puzzle1Test {
            test_data: String::from("2,3,0,3,99"),
            expected_result: 2,
        });
        tests.push(Puzzle1Test {
            test_data: String::from("2,4,4,5,99,0"),
            expected_result: 2,
        });
        tests.push(Puzzle1Test {
            test_data: String::from("1,1,1,4,99,5,6,0,99"),
            expected_result: 30,
        });
        tests.push(Puzzle1Test {
            test_data: String::from("1,9,10,3,2,3,11,0,99,30,40,50"),
            expected_result: 3500,
        });

        match utils::read_lines("data/Day5.txt") {
            Ok(read_file_lines) => {
                let input_line: String = read_file_lines.get(0).unwrap().clone();

                let mut parsed_values: Vec<&str> = input_line.split(",")
                    .collect();

                parsed_values[1] = "12";
                parsed_values[2] = "2";

                tests.push(Puzzle1Test {
                    test_data: parsed_values.join(","),
                    expected_result: 5290681,
                });
                for test in tests {
                    let result = puzzle_1(test.test_data);
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
        desired_output: i32,
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
                    desired_output: 19690720,
                    expected_result: 5741,
                });
                for test in tests {
                    let result = puzzle_2(test.test_data, test.desired_output);
                    assert_eq!(result, test.expected_result);
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}