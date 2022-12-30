use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX :Regex = Regex::new(r"^\[\]$").unwrap();
}


fn are_in_the_right_order(lhs: &str, rhs: &str) -> bool {


    return true;
}

fn puzzle1(input_array: &[&str]) -> usize {
    let mut vec = Vec::<usize>::new();
    let mut packet_count = 0usize;

    let mut input_iter = input_array.iter();
    loop {
        packet_count = packet_count + 1;
        let lhs_option = input_iter.next();

        if lhs_option == None {
            break;
        }

        let lhs= *lhs_option.unwrap();
        let rhs = *input_iter.next().unwrap();
        let _blank_line = input_iter.next();


        if are_in_the_right_order(lhs, rhs) {
            vec.push(packet_count);
        }
    }

    return vec.iter().sum();
}

fn puzzle2(input_array: &[&str]) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day13::{puzzle1, puzzle2};
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 23] = [
        "[1,1,3,1,1]",
        "[1,1,5,1,1]",
        "",
        "[[1],[2,3,4]]",
        "[[1],4]",
        "",
        "[9]",
        "[[8,7,6]]",
        "",
        "[[4,4],4,4]",
        "[[4,4],4,4,4]",
        "",
        "[7,7,7,7]",
        "[7,7,7]",
        "",
        "[]",
        "[3]",
        "",
        "[[[]]]",
        "[[]]",
        "",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]"];

    #[test]
    #[ignore]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, 13);
    }

    #[test]
    #[ignore]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day13.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle1(&data);
        assert_eq!(return_value, 102399);
    }


    #[test]
    #[ignore]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA);
        assert_eq!(return_value, 2713310158);
    }

    #[test]
    #[ignore]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day13.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data);
        assert_eq!(return_value, 23641658401);
    }
}