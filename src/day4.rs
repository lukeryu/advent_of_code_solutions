fn hasAjacentEqualCharacters(string: &String) -> bool {
    let digits: Vec<_> = string.chars().map(|n| n.to_digit(10).unwrap()).collect();

    // Check never decreases
    for i in 1..digits.len() {
        if &digits[i] < &digits[i-1] { return false; }
    }

    // Check for existence of pair
    for i in 1..digits.len() {
        if &digits[i] == &digits[i-1] { return true; }
    }
    false
}

fn recursive_build(value: u32, recursive: u32) -> impl Iterator<Item=u32> {
//    return (value..10)
//        .flat_map(|thisVal| {
//            let ten: u32 = 10;
//            let amount = thisVal * ten.pow(recursive);
//            return recursive_build(thisVal, recursive - 1)
//                .map(|isAThing| -> u32 { isAThing + amount });
//        });

    return (1..10);
}

fn puzzle1(start: &u32, end: &u32) -> usize {
    return recursive_build(0, 6)
        .filter(|x| x > start)
        .filter(|x| x < end)
        .map(|number| -> String { number.to_string() })
        .filter(hasAjacentEqualCharacters)
        .count();
}

fn puzzle2(input: Vec<String>) -> i32 {
    for val in 1..10 {}
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day4::{puzzle1, puzzle2};
    use crate::utils;

    struct Puzzle1Test {
        startPassword: u32,
        endPassword: u32,
        expected_result: usize,
    }

    #[test]
    fn test_puzzle_1() {
        let mut tests: Vec<Puzzle1Test> = Vec::new();
        tests.push(Puzzle1Test {
            startPassword: 372304,
            endPassword: 847060,
            expected_result: 0,
        });

        for test in tests {
            let result = puzzle1(&test.startPassword, &test.endPassword);
            assert_eq!(result, test.expected_result);
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