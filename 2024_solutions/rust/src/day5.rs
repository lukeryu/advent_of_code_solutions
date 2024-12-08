
fn parse_input(input_array: &[&str]) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let mut instructions = Vec::new();
    let mut updates = Vec::new();

    for input_line in input_array {
        if input_line.contains('|') {
            let instruction = input_line
                .split("|")
                .map(|token| token.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            instructions.push(instruction);
        } else if input_line.contains(",") {
            let update = input_line
                .split(",")
                .map(|token| token.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            updates.push(update);
        }
    }

    (instructions, updates)
}

fn matches_all_instructions(instructions: &Vec<Vec<i64>>, update: &Vec<i64>) -> bool {
    for instruction in instructions {
        match update.iter().position(|&page| page == instruction[0]) {
            Some(first_update_index) => {
                match update.iter().position(|&page| page == instruction[1]) {
                    Some(second_update_index) => {
                        if first_update_index > second_update_index {
                            return false;
                        }
                    }
                    _ => continue,
                }
            }
            _ => continue,
        }
    }
    true
}
fn puzzle1(input_array: &[&str]) -> i64 {
    let (instructions, updates) = parse_input(input_array);

    let mut sum = 0;
    for update in updates {
        if matches_all_instructions(&instructions, &update) {
            let middle = update.len() / 2;
            let midde_value = update[middle];
            sum = sum + midde_value;
        }
    }

    sum
}

fn fix_update(instructions: &Vec<Vec<i64>>, update: &Vec<i64>) -> Vec<i64> {
    let mut fixed_update = update.clone();

    for instruction in instructions {
        match fixed_update.iter().position(|&page| page == instruction[0]) {
            Some(first_update_index) => {
                match fixed_update.iter().position(|&page| page == instruction[1]) {
                    Some(second_update_index) => {
                        if first_update_index > second_update_index {
                           fixed_update.remove(first_update_index);
                           fixed_update.insert(second_update_index, instruction[0]);
                            return fix_update(instructions, &fixed_update);
                        }
                    }
                    _ => continue,
                }
            }
            _ => continue,
        }
    }

    fixed_update
}


fn puzzle2(input_array: &[&str]) -> i64 {
    let (instructions, updates) = parse_input(input_array);

    let mut sum = 0;
    for update in updates {
        if !matches_all_instructions(&instructions, &update) {
            let fixed_update = fix_update(&instructions, &update);
            let middle = fixed_update.len() / 2;
            let midde_value = fixed_update[middle];
            sum = sum + midde_value;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::day5::*;
    use crate::utils::read_file_strings;

    const EXAMPLE_INPUT: [&str; 28] = [
        "47|53",
        "97|13",
        "97|61",
        "97|47",
        "75|29",
        "61|13",
        "75|53",
        "29|13",
        "97|29",
        "53|29",
        "61|53",
        "97|53",
        "61|29",
        "47|13",
        "75|47",
        "97|75",
        "47|61",
        "75|61",
        "47|29",
        "75|13",
        "53|13",
        "",
        "75,47,61,53,29",
        "97,61,53,29,13",
        "75,29,13",
        "75,97,47,61,53",
        "61,13,29",
        "97,13,75,29,47",
    ];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&EXAMPLE_INPUT);
        assert_eq!(return_value, 143);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day5.txt");
        vec.iter().for_each(|string| data.push(string.as_str()));

        let return_value = puzzle1(&data[..]);
        assert_eq!(return_value, 4774);
    }

    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&EXAMPLE_INPUT);
        assert_eq!(return_value, 123);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day5.txt");
        vec.iter().for_each(|string| data.push(string.as_str()));

        let return_value = puzzle2(&data[..]);
        assert_eq!(return_value, 6004);
    }
}
