fn puzzle1(input: Vec<String>) -> i32 {
    return 0;
}

fn puzzle2(input: Vec<String>) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::template::{puzzle1, puzzle2};
    use crate::utils;

    struct Puzzle1Test {
        test_data: Vec<String>,
        expected_result: i32,
    }

    #[test]
    fn test_puzzle_1() {
        let mut tests: Vec<Puzzle1Test> = Vec::new();
        tests.push(Puzzle1Test {
            test_data: vec![String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"), String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")],
            expected_result: 135,
        });
        match utils::read_lines("data/Day3.txt") {
            Ok(lines) => {
                tests.push(Puzzle1Test {
                    test_data: lines,
                    expected_result: 0,
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
        match utils::read_lines("data/Day1.txt") {
            Ok(lines) => {
                tests.push(Puzzle2Test {
                    test_data: lines,
                    expected_result: 0,
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