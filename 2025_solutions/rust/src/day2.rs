use std::collections::HashSet;
use anyhow::Result;

fn puzzle1(input: String) -> Result<usize> {
    let lines = input.split(',')
        .map(|l| l.trim())
        .collect::<Vec<&str>>();

    let mut invalid_sum = 0;

    for line in lines {
        let range = parse_range(line).unwrap();

        for id_number in range.0..=range.1 {
            if is_invalid(id_number) {
                invalid_sum = invalid_sum + id_number;
            }
        }
    }

    Ok(invalid_sum)
}

fn is_invalid(p0: usize) -> bool {
    let string = p0.to_string();

    if string.len() % 2 == 1 {
        return false;
    }

    let half = string.len() / 2;

    if string[0..half] == string[half..] {
        return true;
    }

    return false;
}

fn is_invalid_2(p0: usize) -> bool {
    let string = p0.to_string();

    for thing in 2..=string.len() {
        if (string.len() % thing) != 0 {
            continue;
        }

        let mut hash_set = HashSet::<String>::new();
        let length = string.len() / thing;

        for asdf in 0..thing {
            hash_set.insert(string[(length * asdf)..(length * asdf + length)].to_string());
        }


        if hash_set.len() == 1 {
            return true;
        }
    }


    return false;
}

fn parse_range(p0: &str) -> Result<(usize, usize)> {
    println!("value: {}",p0);
    let option = p0.find('-').unwrap();
    println!("option: {}",option);

    let lhs = p0[0..option].parse::<usize>().unwrap();
    let rhs = p0[option + 1..].parse::<usize>().unwrap();

    return Ok((lhs, rhs));
}

fn puzzle2(input: String) -> Result<usize> {
    let lines = input.split(',')
        .map(|l| l.trim())
        .collect::<Vec<&str>>();

    let mut invalid_sum = 0;

    for line in lines {
        let range = parse_range(line).unwrap();

        for id_number in range.0..=range.1 {
            if is_invalid_2(id_number) {
                invalid_sum = invalid_sum + id_number;
            }
        }
    }

    Ok(invalid_sum)
}

mod tests {
    use super::*;
    use crate::utils::read_file_strings;
    #[test]
    fn test_puzzle1() {
        let input: Vec<String> = vec![String::from(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124",
        )];

        let line = input.get(0).unwrap();

        assert_eq!(puzzle1(line.clone()).unwrap(), 1227775554);
    }

    #[test]
    fn test_puzzle1_data() {
        let input: Vec<String> = read_file_strings("../data/Day2.txt");
        let filter_input = input
            .into_iter()
            .filter(|line| !line.is_empty())
            .collect::<Vec<String>>();

        assert_eq!(puzzle1(filter_input.get(0).unwrap().clone()).unwrap(), 29818212493);
    }

    #[test]
    fn test_puzzle2() {
        let input: Vec<String> = vec![String::from(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124",
        )];

        assert_eq!(puzzle2(input.get(0).unwrap().clone()).unwrap(), 4174379265);
    }

    #[test]
    fn test_puzzle2_data() {
        let input: Vec<String> = read_file_strings("../data/Day2.txt");
        let filter_input = input
            .into_iter()
            .filter(|line| !line.is_empty())
            .collect::<Vec<String>>();

        assert_eq!(puzzle2(filter_input.get(0).unwrap().clone()).unwrap(), 37432260594);
    }
}
