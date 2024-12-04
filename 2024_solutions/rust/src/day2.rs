enum Direction {
    Ascending,
    Descending,
}

fn parse_values(input_array: &[&str]) -> Vec<Vec<i64>> {
    let mut tokens = Vec::with_capacity(input_array.len());
    for input_string in input_array {
        let token_numbers = input_string
            .split_whitespace()
            .map(|token| token.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        tokens.push(token_numbers);
    }
    return tokens;
}

fn puzzle1(input_array: &[&str]) -> usize {
    let reports = parse_values(input_array);

    let mut valid_count = 0;

    for report in reports {
        let is_valid = validate_report(&report);

        if is_valid {
            valid_count = valid_count + 1;
        }
    }

    return valid_count;
}

fn validate_report(report: &Vec<i64>) -> bool {
    if report.is_empty() {
        return false;
    }

    if report.len() == 1 {
        return true;
    }

    let first_value = report[0];
    let second_value = report[1];

    if first_value == second_value {
        return false;
    }

    let direction: Direction = if first_value > second_value {
        Direction::Descending
    } else {
        Direction::Ascending
    };

    for (index, value) in report.iter().enumerate() {
        if index < report.len() - 1 {
            let next_value = report[index + 1];
            let current_value = value.clone();

            let diff = next_value - current_value;
            match direction {
                Direction::Ascending => {
                    if diff < 1 || diff > 3 {
                        return false;
                    }
                }
                Direction::Descending => {
                    if diff > -1 || diff < -3 {
                        return false;
                    }
                }
            }
        }
    }

    return true;
}

fn puzzle2(input_array: &[&str]) -> i64 {
    let reports = parse_values(input_array);

    let mut valid_count = 0;

    for report in reports {
        let is_valid = validate_report_2(&report);

        if is_valid {
            valid_count = valid_count + 1;
        }
    }

    return valid_count;
}

fn validate_report_2(report: &Vec<i64>) -> bool {
    let is_valid = validate_report(&report);

    if is_valid {
        return true;
    }

    for index in 0..report.len() {
        let mut fixed_report = report.clone();
        fixed_report.remove(index);

        let fix_is_valid = validate_report(&fixed_report);
        if fix_is_valid {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use crate::day2::puzzle1;
    use crate::day2::puzzle2;
    use crate::utils::read_file_strings;

    const TEST_INPUT: [&str; 6] = [
        "7 6 4 2 1",
        "1 2 7 8 9",
        "9 7 6 2 1",
        "1 3 2 4 5",
        "8 6 4 4 1",
        "1 3 6 7 9",
    ];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_INPUT);
        assert_eq!(return_value, 2);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day2.txt");
        vec.iter().for_each(|string| data.push(string.as_str()));

        let return_value = puzzle1(&data[..]);
        assert_eq!(return_value, 624);
    }

    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_INPUT);
        assert_eq!(return_value, 4);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day2.txt");
        vec.iter().for_each(|string| data.push(string.as_str()));

        let return_value = puzzle2(&data[..]);
        assert_eq!(return_value, 658);
    }
}
