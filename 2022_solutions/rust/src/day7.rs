use std::collections::{HashMap, LinkedList};
use std::ops::AddAssign;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^(\d+) (\w+\.?\w*)$").unwrap();
}

fn get_directories(current_directory: &LinkedList<&str>) -> Vec<String> {
    let mut directories = Vec::<String>::new();
    for index in (1..=current_directory.len()).rev() {
        let slice = current_directory.iter()
            .take(index)
            .map(|string| String::from(*string))
            .reduce(|accum, item| {
                return format!("{}/{}", accum, item);
            }).unwrap();

        directories.push(slice);
    }

    return directories;
}

fn get_directory_sizes(input_array: &[&str]) -> HashMap<String, usize> {
    let mut current_directory = LinkedList::<&str>::new();
    let mut hash_map = HashMap::<String, usize>::new();

    for input_string in input_array {
        if (*input_string).trim().starts_with("dir") {
            continue;
        } else if (*input_string).trim().starts_with("$ cd") {
            if "$ cd .." == *input_string {
                current_directory.pop_back();
            } else {
                current_directory.push_back(&input_string[5..]);
            }
        } else if REGEX.is_match(*input_string) {
            let tag = REGEX.captures((*input_string).trim()).unwrap();

            let file_size = tag.get(1).unwrap().as_str().parse::<usize>().unwrap();

            let directories = get_directories(&current_directory);

            for dir in directories {
                let value = hash_map.entry(dir).or_insert(0);
                value.add_assign(file_size);
            }
        }
    }

    return hash_map;
}

fn puzzle1(input_array: &[&str]) -> usize {
    let hash_map = get_directory_sizes(input_array);

    let sum = hash_map.into_values()
        .filter(|value| *value < (100000 as usize))
        .sum();


    return sum;
}

const TOTAL_DISK_SPACE: usize = 70000000;
const REQUIRED_DISK_SPACE: usize = 30000000;

fn puzzle2(input_array: &[&str]) -> usize {
    let hash_map = get_directory_sizes(input_array);

    let root_value = hash_map.get("/").unwrap();
    let remaining_space = TOTAL_DISK_SPACE - *root_value;

    let space_needed = REQUIRED_DISK_SPACE - remaining_space;

    let min_thing_value = hash_map.values()
        .filter(|value| **value > space_needed)
        .min();

    return *min_thing_value.unwrap();
}

#[cfg(test)]
mod tests {
    use crate::day7::puzzle1;
    use crate::day7::puzzle2;
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 23] = [
        "$ cd /",
        "$ ls",
        "dir a",
        "14848514 b.txt",
        "8504156 c.dat",
        "dir d",
        "$ cd a",
        "$ ls",
        "dir e",
        "29116 f",
        "2557 g",
        "62596 h.lst",
        "$ cd e",
        "$ ls",
        "584 i",
        "$ cd ..",
        "$ cd ..",
        "$ cd d",
        "$ ls",
        "4060174 j",
        "8033020 d.log",
        "5626152 d.ext",
        "7214296 k"];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, 95437);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day7.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle1(&data[..]);
        assert_eq!(return_value, 1243729);
    }

    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA);
        assert_eq!(return_value, 24933642);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day7.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data[..]);
        assert_eq!(return_value, 4443914);
    }
}