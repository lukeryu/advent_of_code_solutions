fn recurse(element: &Vec<usize>, include_concat: bool) -> Vec<usize> {
    let mut results = Vec::new();

    if element.is_empty() {
        return results;
    }

    let last = element.last();
    match last {
        Some(item) => {
            let sublist = element[..element.len() - 1]
                .iter()
                .map(|element| element.clone())
                .collect::<Vec<usize>>();
            let result_items = recurse(&sublist, include_concat);

            if result_items.is_empty() {
                results.push(*item);
            } else {
                for result_item in result_items {
                    results.push(result_item + item);
                    results.push(result_item * item);
                    if include_concat {
                        let mut result_string = result_item.to_string();
                        let item_string = item.to_string();

                        result_string.push_str(&item_string);

                        let concat_number = result_string.parse::<usize>().unwrap();
                        results.push(concat_number);
                    }
                }
            }
        }
        None => (),
    }

    results
}

fn puzzle1(input_array: &[&str]) -> usize {
    let (totals, elements) = parse_input(input_array);

    let mut sum = 0;
    for (index, element) in elements.iter().enumerate() {
        let results = recurse(element, false);
        match totals.iter().nth(index) {
            Some(total) => {
                if results.contains(total) {
                    sum = sum + *total;
                }
            }
            None => (),
        }
    }

    sum
}

fn parse_input(input_array: &[&str]) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut totals = Vec::<usize>::with_capacity(input_array.len());
    let mut elements = Vec::<Vec<usize>>::with_capacity(input_array.len());

    for input in input_array {
        let mut split_result = input.split(':').collect::<Vec<&str>>();

        let element_result = split_result.pop();
        match element_result {
            Some(val) => elements.push(
                val.split_whitespace()
                    .filter(|string| !string.is_empty())
                    .map(|str| str.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            ),
            None => elements.push(Vec::new()),
        }

        let total_result = split_result.pop();
        match total_result {
            Some(val) => totals.push(val.parse::<usize>().unwrap()),
            None => totals.push(0),
        }
    }

    (totals, elements)
}

fn puzzle2(input_array: &[&str]) -> usize {
    let (totals, elements) = parse_input(input_array);

    let mut sum = 0;
    for (index, element) in elements.iter().enumerate() {
        let results = recurse(element, true);
        match totals.iter().nth(index) {
            Some(total) => {
                if results.contains(total) {
                    sum = sum + *total;
                }
            }
            None => (),
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::day7::*;
    use crate::utils::read_file_strings;

    const EXAMPLE_INPUT: [&str; 9] = [
        "190: 10 19",
        "3267: 81 40 27",
        "83: 17 5",
        "156: 15 6",
        "7290: 6 8 6 15",
        "161011: 16 10 13",
        "192: 17 8 14",
        "21037: 9 7 18 13",
        "292: 11 6 16 20",
    ];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&EXAMPLE_INPUT);
        assert_eq!(return_value, 3749);
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
