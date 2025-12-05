use std::usize;
use anyhow::Result;

fn puzzle1(input: Vec<String>) -> Result<usize> {
    let result: usize = input
        .into_iter()
        .map(|string: String| -> usize {
            let mut asdf: usize = 0;
            for (index, character) in string.char_indices() {
                for character2 in string[index + 1..].chars() {
                    let mut new_string = String::from(character);
                    new_string.push(character2);
                    let new_asdf = new_string.parse::<usize>().unwrap();
                    if new_asdf > asdf {
                        asdf = new_asdf;
                    }
                }
            }

            return asdf;
        })
        .sum();
    Ok(result)
}

fn find_biggest_combination(n_digits: usize, mut slice: &[u8]) -> usize {
    let mut sol = 0;
    for i in (0..n_digits).rev() {
        let (best_index, n) = slice
            .iter()
            .enumerate()
            .rev()
            .skip(i)
            .max_by_key(|a| a.1)
            .unwrap();

        slice = &slice[best_index+1..];
        sol = sol * 10 + (n - b'0') as usize;
    }
    sol
}

fn puzzle2(input: Vec<String>) -> Result<usize> {

    let result: usize = input
        .into_iter()
        .map(|string: String| -> usize {
            let str = string.as_bytes();
            let i = find_biggest_combination(12, str);
            return i;
        })
        .sum();
    Ok(result)
}

mod tests {
    use super::*;
    use crate::utils::read_file_strings;
    #[test]
    fn test_puzzle1() {
        let input: Vec<String> = vec![
            String::from("987654321111111"),
            String::from("811111111111119"),
            String::from("234234234234278"),
            String::from("818181911112111"),
        ];

        assert_eq!(puzzle1(input).unwrap(), 357);
    }

    #[test]
    fn test_puzzle1_data() {
        let input: Vec<String> = read_file_strings("../data/Day3.txt");
        let filter_input = input
            .into_iter()
            .filter(|line| !line.is_empty())
            .collect::<Vec<String>>();

        assert_eq!(puzzle1(filter_input).unwrap(), 16993);
    }

    #[test]
    fn test_puzzle2() {
        let input: Vec<String> = vec![
            String::from("987654321111111"),
            String::from("811111111111119"),
            String::from("234234234234278"),
            String::from("818181911112111"),
        ];

        assert_eq!(puzzle2(input).unwrap(), 3121910778619);
    }

    #[test]
    fn test_puzzle2_data() {
        let input: Vec<String> = read_file_strings("../data/Day3.txt");
        let filter_input = input
            .into_iter()
            .filter(|line| !line.is_empty())
            .collect::<Vec<String>>();

        assert_eq!(puzzle2(filter_input).unwrap(), 168617068915447);
    }
}
