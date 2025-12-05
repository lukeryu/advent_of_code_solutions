fn puzzle1(input_array: &[&str]) -> usize {
    if (!input_array.len() != 1) {
        panic!("Input array must have equal length");
    }
    let input = input_array[0];

    let parsed_chars = input.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();
    let mut vec = Vec::<String>::with_capacity(parsed_chars.iter().sum());
    let current_file_id: usize = 0;
    for (index, value) in parsed_chars.iter().enumerate() {
        let char = (index % 2 == 0) ? * value.to_string(): ".";

        for _ in 0..*value {
            vec.push(char);
        }
    }
}


0
}



fn puzzle2(input_array: &[&str]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::day9::*;
    use crate::utils::read_file_strings;

    const EXAMPLE_INPUT: [&str; 1] = [
        "2333133121414131402"
    ];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&EXAMPLE_INPUT);
        assert_eq!(return_value, 1928);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day7.txt");
        vec.iter().for_each(|string| data.push(string.as_str()));

        let return_value = puzzle1(&data[..]);
        assert_eq!(return_value, 12553187650171);
    }

    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&EXAMPLE_INPUT);
        assert_eq!(return_value, 11387);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day7.txt");
        vec.iter().for_each(|string| data.push(string.as_str()));

        let return_value = puzzle2(&data[..]);
        assert_eq!(return_value, 96779702119491);
    }
}
