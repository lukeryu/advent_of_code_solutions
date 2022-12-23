use std::collections::{VecDeque, HashMap};

fn puzzle1(lines: Vec<String>) -> u32 {
    let mut numbers = lines.iter()
        .map(|line| (*line).parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    for turn_number in lines.len()..2020 {
        let last_number = numbers.iter().rev().next().unwrap();

        let previous_index = numbers.iter().rev()
            .skip(1)
            .position(|value| *value == *last_number);

        match previous_index {
            Some(index) => {
                numbers.push((turn_number - (numbers.len() - 1 - index)) as u32)
            },
            None => {
                numbers.push(0);
            }
        }

    }

    return numbers.iter().rev().next().unwrap().clone();
}

fn puzzle2(lines: Vec<String>) -> u32 {
    let mut numbers = lines.iter()
        .map(|line| (*line).parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut last_seen = HashMap::<u64,usize>::new();
    numbers.iter()
        .enumerate()
        .for_each(|(index, value)| {
            last_seen.entry(*value).or_insert(index);
        });

    let mut last_number = *(numbers.last().unwrap());
    for turn_number in lines.len()..30000000 {

        let number_index = last_number[turn_number];

        let previous_index = numbers.iter().rev()
            .skip(1)
            .position(|value| *value == last_number);

        let num = match previous_index {
            Some(index) => {
                (turn_number - (numbers.len() - 1 - index)) as u64
            },
            None => {
                0
            }
        };
        last_number = num;
        numbers.push(num);

    }

    return numbers.iter().rev().next().unwrap().clone();
}

#[cfg(test)]
mod tests {
    use crate::day15::{puzzle1, puzzle2};
    use crate::utils::read_lines;

    #[test]
    fn test_puzzle_1_1() {
        let tests = vec![String::from("0"),String::from("3"),String::from("6")];
        let result = puzzle1(tests);
        assert_eq!(result, 436);
    }

    #[test]
    fn test_puzzle_1_2() {
        let tests = vec![String::from("1"),String::from("3"),String::from("2")];
        let result = puzzle1(tests);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_puzzle_1_3() {
        let tests = vec![String::from("2"),String::from("1"),String::from("3")];
        let result = puzzle1(tests);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_puzzle_1_4() {
        let tests = vec![String::from("1"),String::from("2"),String::from("3")];
        let result = puzzle1(tests);
        assert_eq!(result, 27);
    }

    #[test]
    fn test_puzzle_1_5() {
        let tests = vec![String::from("2"),String::from("3"),String::from("1")];
        let result = puzzle1(tests);
        assert_eq!(result, 78);
    }

    #[test]
    fn test_puzzle_1_6() {
        let tests = vec![String::from("3"),String::from("2"),String::from("1")];
        let result = puzzle1(tests);
        assert_eq!(result, 438);
    }

    #[test]
    fn test_puzzle_1_7() {
        let tests = vec![String::from("3"),String::from("1"),String::from("2")];
        let result = puzzle1(tests);
        assert_eq!(result, 1836);
    }



    #[test]
    fn test_puzzle_1_8() {
        let tests = vec![
            String::from("2"),
            String::from("0"),
            String::from("1"),
            String::from("9"),
            String::from("5"),
            String::from("19")
        ];
        let result = puzzle1(tests);
        assert_eq!(result, 1009);
    }

    #[test]
    fn test_puzzle_2_1() {
        let tests = vec![String::from("0"),String::from("3"),String::from("6")];
        let result = puzzle2(tests);
        assert_eq!(result, 175594);
    }

    #[test]
    fn test_puzzle_2_2() {
        let tests = vec![String::from("1"),String::from("3"),String::from("2")];
        let result = puzzle2(tests);
        assert_eq!(result, 2578);
    }

    #[test]
    fn test_puzzle_2_3() {
        let tests = vec![String::from("2"),String::from("1"),String::from("3")];
        let result = puzzle2(tests);
        assert_eq!(result, 3544142);
    }

    #[test]
    fn test_puzzle_2_4() {
        let tests = vec![String::from("1"),String::from("2"),String::from("3")];
        let result = puzzle2(tests);
        assert_eq!(result, 261214);
    }

    #[test]
    fn test_puzzle_2_5() {
        let tests = vec![String::from("2"),String::from("3"),String::from("1")];
        let result = puzzle2(tests);
        assert_eq!(result, 6895259);
    }

    #[test]
    fn test_puzzle_2_6() {
        let tests = vec![String::from("3"),String::from("2"),String::from("1")];
        let result = puzzle2(tests);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_puzzle_2_7() {
        let tests = vec![String::from("3"),String::from("1"),String::from("2")];
        let result = puzzle2(tests);
        assert_eq!(result, 362);
    }

    #[test]
    fn test_puzzle_2_8() {
        let tests = vec![
            String::from("2"),
            String::from("0"),
            String::from("1"),
            String::from("9"),
            String::from("5"),
            String::from("19")
        ];
        let result = puzzle2(tests);
        assert_eq!(result, 0);
    }
}