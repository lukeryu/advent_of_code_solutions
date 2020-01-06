fn puzzle1(input: Vec<String>) -> i32 {
    return input.iter()
        .map(|s| -> i32 { s.parse::<i32>().unwrap() })
        .map(|mass| -> i32 { (mass / 3) - 2 })
        .sum();
}

fn puzzle2(input: Vec<String>) -> i32 {
    return puzzle2_recursive_sum(input.iter().map(|s| -> i32 { s.parse::<i32>().unwrap() }).collect());
}

fn puzzle2_recursive_sum(input: Vec<i32>) -> i32 {
    let mut sum = 0;
    for mass in input {
        let fuel_mass = (mass / 3) - 2;
        if fuel_mass > 0 {
            sum += fuel_mass;
            let recursive_mass = puzzle2_recursive_sum(vec![fuel_mass]);
            if recursive_mass > 0 {
                sum += recursive_mass;
            }
        }
    }
    if sum <= 0 {
        return 0;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use crate::utils;
    use crate::day1::{puzzle1, puzzle2};

    struct Puzzle1Test {
        test_data: Vec<String>,
        expected_result: i32,
    }

    #[test]
    fn test_puzzle_1() {
        let mut tests: Vec<Puzzle1Test> = Vec::new();
        tests.push(Puzzle1Test {
            test_data: vec![String::from("12")],
            expected_result: 2,
        });
        tests.push(Puzzle1Test {
            test_data: vec![String::from("14")],
            expected_result: 2,
        });
        tests.push(Puzzle1Test {
            test_data: vec![String::from("1969")],
            expected_result: 654,
        });
        tests.push(Puzzle1Test {
            test_data: vec![String::from("100756")],
            expected_result: 33583,
        });
        match utils::read_lines("data/Day1.txt") {
            Ok(lines) => {
                tests.push(Puzzle1Test {
                    test_data: lines,
                    expected_result: 3126794,
                });
                for test in tests {
                    let result = puzzle1(test.test_data);
                    assert_eq!(result, test.expected_result);
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }

    struct Puzzle2Test {
        test_data: Vec<String>,
        expected_result: i32,
    }

    #[test]
    fn test_puzzle_2() {
        let mut tests: Vec<Puzzle2Test> = Vec::new();
        tests.push(Puzzle2Test {
            test_data: vec![String::from("14")],
            expected_result: 2,
        });
        tests.push(Puzzle2Test {
            test_data: vec![String::from("1969")],
            expected_result: 966,
        });
        tests.push(Puzzle2Test {
            test_data: vec![String::from("100756")],
            expected_result: 50346,
        });
        match utils::read_lines("data/Day1.txt") {
            Ok(lines) => {
                tests.push(Puzzle2Test {
                    test_data: lines,
                    expected_result: 4687331,
                });
                for test in tests {
                    let result = puzzle2(test.test_data);
                    assert_eq!(result, test.expected_result);
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}