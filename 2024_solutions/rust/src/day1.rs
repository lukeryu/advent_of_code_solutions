use std::collections::HashMap;

fn parse_lists(input_array: &[&str]) -> (Vec<i32>, Vec<i32>) {
    let mut lhs = Vec::<i32>::new();
    let mut rhs = Vec::<i32>::new();

    for line in input_array {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace().collect::<Vec<&str>>();
        let lhs_value_string = parts.pop().unwrap();
        let rhs_value_string = parts.pop().unwrap();
        let lhs_val = lhs_value_string.parse().unwrap();
        let rhs_val = rhs_value_string.parse().unwrap();
        lhs.push(rhs_val);
        rhs.push(lhs_val);
    }

    return (lhs, rhs);
}

fn puzzle1(input_array: &[&str]) -> i32 {
    let lists = parse_lists(input_array);
    let mut lhs = lists.0;
    let mut rhs = lists.1;

    lhs.sort();
    rhs.sort();

    let mut sum = 0;

    for (index, lhs_value) in lhs.iter().enumerate() {
        let difference = lhs_value - rhs[index];

        sum = sum + difference.abs();
    }

    return sum;
}

fn puzzle2(input_array: &[&str]) -> i64 {
    let lists = parse_lists(input_array);
    let lhs = lists.0;
    let rhs = lists.1;

    let mut rhs_map = HashMap::<i32, i64>::new();

    for rhs_value in rhs {
        rhs_map.entry(rhs_value).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut product: i64 = 0;

    for lhs_value in lhs {
        let rhs_count = rhs_map.get(&lhs_value).unwrap_or(&0);
        let difference: i64 = lhs_value as i64 * rhs_count;

        product = product + difference.abs();
    }

    return product;
}

#[cfg(test)]
mod tests {
    use crate::day1::puzzle1;
    use crate::day1::puzzle2;
    use crate::utils::read_file_strings;

    #[test]
    fn test_puzzle1() {
        let values = &["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"];

        let return_value = puzzle1(values);
        assert_eq!(return_value, 11);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day1.txt");
        vec.iter().for_each(|string| data.push(string.as_str()));

        let return_value = puzzle1(&data[..]);
        assert_eq!(return_value, 1666427);
    }

    #[test]
    fn test_puzzle2() {
        let values = &["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"];

        let return_value = puzzle2(values);
        assert_eq!(return_value, 31);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day1.txt");
        vec.iter().for_each(|string| data.push(string.as_str()));

        let return_value = puzzle2(&data[..]);
        assert_eq!(return_value, 24316233);
    }
}
