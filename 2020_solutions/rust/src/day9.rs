use std::collections::VecDeque;
use std::ops::Index;

fn find_sum(value: u64, preamble_values: &VecDeque<u64>) -> Option<u64> {
    for val1 in preamble_values.iter().enumerate() {
        for val2 in preamble_values.iter().enumerate() {
            if (val1.0 != val2.0) {
                let sum = val1.1 + val2.1;
                if (sum == value) {
                    return Option::Some(sum);
                }
            }
        }
    }
    return Option::None;
}

fn puzzle1(preamble: usize, input: Vec<String>) -> u64 {
    let mut values = Vec::<u64>::with_capacity(input.len());
    input.iter()
        .map(|stng|  stng.parse::<u64>().unwrap())
        .for_each(|val| values.push(val));


    let mut preamble_values = VecDeque::<u64>::with_capacity(preamble);

    for val in values.iter().take(preamble) {
        preamble_values.push_back(*val);
    }

    for val in values.iter().skip(preamble) {
        let found_sum = find_sum(*val, &preamble_values);
        match found_sum {
            None => return *val,
            Some(sum) => {
                preamble_values.pop_front();
                preamble_values.push_back(*val);
            }
        }
    }

    return 0;
}

fn puzzle2(preamble: usize, input: Vec<String>) -> u64 {
    let mut values = Vec::<u64>::with_capacity(input.len());
    input.iter()
        .map(|stng|  stng.parse::<u64>().unwrap())
        .for_each(|val| values.push(val));


    let invalid_number = puzzle1(preamble, input);

    let invalid_number_index = values.iter().position(|number| *number == invalid_number).unwrap();

    for size in 2..20 as usize {

        for start_index in 0..(invalid_number_index - size) {

            let sum: u64 = values.iter().skip(start_index)
                .take(size)
                .sum();

            if (sum == invalid_number) {
                let max_range_value = values.iter().skip(start_index)
                    .take(size)
                    .max().unwrap();

                let min_range_value = values.iter().skip(start_index)
                    .take(size)
                    .min().unwrap();

                return *max_range_value + *min_range_value;
            }

        }

    }


    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day9::{puzzle1, puzzle2};
    use crate::utils::read_lines;

    #[test]
    fn test_puzzle_1_1() {
        let strs = vec!["35",
                        "20",
                        "15",
                        "25",
                        "47",
                        "40",
                        "62",
                        "55",
                        "65",
                        "95",
                        "102",
                        "117",
                        "150",
                        "182",
                        "127",
                        "219",
                        "299",
                        "277",
                        "309",
                        "576"];

        let strings = strs.into_iter()
            .map(String::from)
            .collect();

        let result = puzzle1(5, strings);
        assert_eq!(result, 127);
    }

    #[test]
    fn test_puzzle_1_2() {
        let strs = read_lines("data/Day9.txt").unwrap();

        let result = puzzle1(25,strs);
        assert_eq!(result, 556543474);
    }

    #[test]
    fn test_puzzle_2_1() {
        let strs = vec!["35",
                        "20",
                        "15",
                        "25",
                        "47",
                        "40",
                        "62",
                        "55",
                        "65",
                        "95",
                        "102",
                        "117",
                        "150",
                        "182",
                        "127",
                        "219",
                        "299",
                        "277",
                        "309",
                        "576"];

        let strings = strs.into_iter()
            .map(String::from)
            .collect();

        let result = puzzle2(5, strings);
        assert_eq!(result, 62);
    }

    #[test]
    fn test_puzzle_2_2() {
        let strs = read_lines("data/Day9.txt").unwrap();

        let result = puzzle2(25,strs);
        assert_eq!(result, 76096372);
    }
}