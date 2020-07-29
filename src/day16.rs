fn puzzle1(input: String, iterations: usize) -> String {
    let mut digits: Vec<u128> = input.chars()
        .map(|char| char.to_string().parse::<u128>().unwrap())
        .collect();

    for iteration_count in 0..iterations {
        let mut iteration_digits = vec![0u128; digits.len()];
        for output_digit_index in 0..digits.len() {
            let pattern = calculate_pattern(output_digit_index, digits.len());
            let mut value = 0i128;
            for calculating_digit_index in 0..digits.len() {
                value = value + digits[calculating_digit_index] as i128 * pattern[calculating_digit_index];
            }

            iteration_digits[output_digit_index] = value.abs() as u128 % 10;
        }
        digits = iteration_digits;
    }

    let vec: Vec<String> = digits[..8].iter()
        .map(u128::to_string)
        .collect();

    return vec.join("");
}

fn puzzle2(input: String, iterations: usize) -> String {
    let mut digits: Vec<u128> = input.chars()
        .map(|char| char.to_string().parse::<u128>().unwrap())
        .collect();

    let start_index = digits[..7].iter()
        .fold(0,|total, object | total * 10 + *object as usize);

    for _ in 0..iterations {
        let mut iteration_digits = vec![0u128; digits.len()];
        for output_digit_index in 0..digits.len() {
            let pattern = calculate_pattern(output_digit_index, digits.len());
            let mut value = 0i128;
            for calculating_digit_index in 0..digits.len() {
                value = value + digits[calculating_digit_index] as i128 * pattern[calculating_digit_index];
            }

            iteration_digits[output_digit_index] = value.abs() as u128 % 10;
        }
        digits = iteration_digits;
    }

    let vec: Vec<String> = digits[start_index..start_index + 8].iter()
        .map(u128::to_string)
        .collect();

    return vec.join("");
}

const BASE_PATTERN: [i128; 4] = [0, 1, 0, -1];

fn calculate_pattern(digit_index: usize, count_needed: usize) -> Vec<i128> {
    let mut pattern = Vec::<i128>::with_capacity(count_needed);

    for i in 1..count_needed + 1 {
        pattern.push(BASE_PATTERN[(i / (digit_index + 1)) % 4]);
    }

    return pattern;
}

#[cfg(test)]
mod tests {
    use crate::day16::{puzzle1, puzzle2};
    use crate::utils::read_lines;

    #[test]
    fn test_puzzle1() {
        let result = puzzle1(String::from("12345678"), 4);
        assert_eq!(result, String::from("01029498"));

        let result2 = puzzle1(String::from("80871224585914546619083218645595"), 100);
        assert_eq!(result2, String::from("24176176"));

        let result3 = puzzle1(String::from("19617804207202209144916044189917"), 100);
        assert_eq!(result3, String::from("73745418"));

        let result4 = puzzle1(String::from("69317163492948606335995924319873"), 100);
        assert_eq!(result4, String::from("52432133"));

        let puzzle_input = read_lines("data/Day16.txt").unwrap();
        let result5 = puzzle1(puzzle_input[0].clone(), 100);
        assert_eq!(result5, String::from("37153056"));
    }

    #[test]
    #[ignore]
    fn test_puzzle2() {
        let puzzle_input = read_lines("data/Day16.txt").unwrap();
        let read_input = puzzle_input[0].clone();
        let input_str = read_input.as_str();
        let mut calculated_input = String::with_capacity(read_input.len() * 10000);

        for i in 0..10000 {
            calculated_input.push_str(input_str);
        }

        let result5 = puzzle2(calculated_input, 100);
        assert_eq!(result5, String::from("37153056"));
    }
}