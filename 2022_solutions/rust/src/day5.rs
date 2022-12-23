use std::collections::{LinkedList};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
}

fn initialize_stacks(input_array: &[&str], blank_index: usize) -> Vec<LinkedList<String>> {
    let mut stacks = Vec::<LinkedList<String>>::with_capacity(10);
    stacks.insert(0, LinkedList::new());
    let source = input_array.get(blank_index - 1).unwrap();
    for (index, stack_header) in source.chars().enumerate() {
        if stack_header.to_string().trim().is_empty() {
            continue;
        }

        let stack_header_num = stack_header.to_string().parse::<usize>().unwrap();
        let mut current_stack = LinkedList::<String>::new();

        for stack_level_index in (0..blank_index - 1).rev() {
            let current_level = input_array.get(stack_level_index).unwrap();
            let current_value = (*current_level).chars().nth(index);
            if current_value.is_some() {
                let unwrapped_value = current_value.unwrap();
                if unwrapped_value.is_alphanumeric() {
                    current_stack.push_front(String::from(unwrapped_value));
                }
            }
        }

        stacks.insert(stack_header_num, current_stack);
    }

    return stacks;
}

fn puzzle1_move_items(quantity: u64, source_stack_index: usize, target_stack_index: usize,  stacks: &mut Vec<LinkedList<String>>) {
    let mut temp_stack = LinkedList::<String>::new();

    {
        let source_stack = stacks.get_mut(source_stack_index).unwrap();
        for _ in 0..quantity {
            let value = source_stack.pop_front().unwrap();
            temp_stack.push_front(value);
        }
    }

    {
        let target_stack = stacks.get_mut(target_stack_index).unwrap();
        for _quant in 0..quantity {
            let value = temp_stack.pop_back().unwrap();
            target_stack.push_front(value);
        }
    }
}

fn puzzle2_move_items(quantity: u64, source_stack_index: usize, target_stack_index: usize,  stacks: &mut Vec<LinkedList<String>>) {
    let mut temp_stack = LinkedList::<String>::new();

    {
        let source_stack = stacks.get_mut(source_stack_index).unwrap();
        for _ in 0..quantity {
            let value = source_stack.pop_front().unwrap();
            temp_stack.push_front(value);
        }
    }

    {
        let target_stack = stacks.get_mut(target_stack_index).unwrap();
        for _quant in 0..quantity {
            let value = temp_stack.pop_front().unwrap();
            target_stack.push_front(value);
        }
    }
}

fn do_the_puzzle(input_array: &[&str], mover: &dyn Fn(u64, usize, usize, &mut Vec<LinkedList<String>>)) -> String {
    let mut blank_index = input_array.len();

    for index in 0..input_array.len() {
        if input_array.get(index).unwrap().trim().len() == 0 {
            blank_index = index;
        }
    }

    let mut stacks = initialize_stacks(input_array, blank_index);
    for index in (blank_index + 1)..input_array.len() {
        let input_string = input_array.get(index).unwrap();
        let tag = REGEX.captures((*input_string).trim()).unwrap();

        let quantity = tag.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let source_stack_index = tag.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let target_stack_index = tag.get(3).unwrap().as_str().parse::<usize>().unwrap();

        mover(quantity, source_stack_index, target_stack_index, &mut stacks);
    }

    let mut result = String::new();

    for stack in stacks.iter_mut() {
        if !stack.is_empty() {
            let top_element = stack.pop_front().unwrap();
            top_element.chars().for_each(|char| result.push(char));
        }
    }

    return result;
}

fn puzzle1(input_array: &[&str]) -> String {
    return do_the_puzzle(input_array, &puzzle1_move_items);
}

fn puzzle2(input_array: &[&str]) -> String {
    return do_the_puzzle(input_array, &puzzle2_move_items);
}

#[cfg(test)]
mod tests {
    use crate::day5::puzzle1;
    use crate::day5::puzzle2;
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 9] = [
        "    [D]    ",
        "[N] [C]    ",
        "[Z] [M] [P]",
        " 1   2   3 ",
        "",
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2"];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, "CMZ");
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day5.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str()));

        let return_value = puzzle1(&data[..]);
        assert_eq!(return_value, "TLFGBZHCN");
    }

    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA);
        assert_eq!(return_value, "MCD");
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day5.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str()));

        let return_value = puzzle2(&data[..]);
        assert_eq!(return_value, "QRQFHFWCL");
    }
}