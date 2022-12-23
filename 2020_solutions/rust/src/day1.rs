fn puzzle1(input: Vec<u32>) -> u32 {
    let final_value: u32 = 2020;

    for value in &input {
        let current_val = final_value - value;

        if (&input).into_iter().any(|v| *v == current_val) {
            return value * current_val;
        }
    }

    return 0;
}

fn puzzle2(input: Vec<u32>) -> u64 {
    for first_value in &input {
        for second_value in &input {
            for third_value in &input {
                if *first_value + *second_value + *third_value == 2020 {
                    return *first_value as u64 * *second_value as u64 * *third_value as u64;
                }
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day1::puzzle1;
    use crate::day1::puzzle2;
    use crate::utils::read_lines;

    #[test]
    fn test_puzzle_1() {
        let values = vec![1721, 979, 366, 299, 675, 1456];

        let result = puzzle1(values);

        assert_eq!(result, 514579)
    }

    #[test]
    fn test_puzzle_1_2() {
        let lines = read_lines("data/Day1.txt");

        match lines {
            Ok(string_vec) => {
                let values = string_vec.iter()
                    .map(|read_string| read_string.parse::<u32>().unwrap())
                    .collect();
                let result = puzzle1(values);

                assert_eq!(result, 1014624);
            }
            Err(err) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_puzzle_2() {
        let values = vec![1721, 979, 366, 299, 675, 1456];

        let result = puzzle2(values);

        assert_eq!(result, 241861950)
    }

    #[test]
    fn test_puzzle_2_2() {
        let lines = read_lines("data/Day1.txt");

        match lines {
            Ok(string_vec) => {
                let values = string_vec.iter()
                    .map(|read_string| read_string.parse::<u32>().unwrap())
                    .collect();
                let result = puzzle2(values);

                assert_eq!(result, 80072256);
            }
            Err(err) => {
                assert!(false);
            }
        }
    }
}