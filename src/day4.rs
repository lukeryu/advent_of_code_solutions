use std::collections::HashMap;

fn has_ajacent_equal_characters(string: &String) -> bool {
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

fn has_ajacent_equal_characters_2(string: &String) -> bool {
    let digits: Vec<_> = string.chars().map(|n| n.to_digit(10).unwrap()).collect();
    // Check never decreases
    for i in 1..digits.len() {
        if &digits[i] < &digits[i-1] { return false; }
    }

    // Check for existence of pair
    let mut value_count = HashMap::new();
    for i in digits.iter() {
        let current_count = value_count.entry(i).or_insert(0);
        *current_count += 1;
    }

    return value_count.values().into_iter()
        .filter(|&&count| count == 2)
        .count() > 0;
}

fn puzzle1(start: &u32, end: &u32) -> usize {
    //recursive_build(0, 6)
    return (start.clone()..end.clone())
        .filter(|x| x > start)
        .filter(|x| x < end)
        .map(|number| -> String { number.to_string() })
        .filter(has_ajacent_equal_characters)
        .count();
}

fn puzzle2(start: &u32, end: &u32) -> usize {
    return (start.clone()..end.clone())
        .filter(|x| x > start)
        .filter(|x| x < end)
        .map(|number| -> String { number.to_string() })
        .filter(has_ajacent_equal_characters_2)
        .count();
}

#[cfg(test)]
mod tests {
    use crate::day4::{puzzle1, puzzle2, has_ajacent_equal_characters_2};
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
            expected_result: 475,
        });

        for test in tests {
            let result = puzzle1(&test.startPassword, &test.endPassword);
            assert_eq!(result, test.expected_result);
        }
    }

    struct Puzzle2Test {
        startPassword: u32,
        endPassword: u32,
        expected_result: usize,
    }

    #[test]
    fn test_puzzle_2() {
        let mut tests: Vec<Puzzle2Test> = Vec::new();
        tests.push(Puzzle2Test {
            startPassword: 372304,
            endPassword: 847060,
            expected_result: 0,
        });
        for test in tests {
            let result = puzzle2(&test.startPassword, &test.endPassword);
            assert_eq!(result, test.expected_result);
        }

    }

    #[test]
    fn test_has_ajacent_equal_characters_2() {
        assert_eq!(has_ajacent_equal_characters_2(&String::from("112233")), true);
        assert_eq!(has_ajacent_equal_characters_2(&String::from("123444")), false);
        assert_eq!(has_ajacent_equal_characters_2(&String::from("111122")), true);
    }
}