use lazy_static::lazy_static;
use crate::utils::Range;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
}

fn parse_ranges(input_string: &str) -> (Range<u64>, Range<u64>) {
    let tag = REGEX.captures((input_string).trim()).unwrap();

    let lhs_1 = tag.get(1).unwrap().as_str().parse::<u64>().unwrap();
    let rhs_1 = tag.get(2).unwrap().as_str().parse::<u64>().unwrap();
    let lhs_2 = tag.get(3).unwrap().as_str().parse::<u64>().unwrap();
    let rhs_2 = tag.get(4).unwrap().as_str().parse::<u64>().unwrap();
    let range1 = Range::new(lhs_1, rhs_1);
    let range2 = Range::new(lhs_2, rhs_2);

    return (range1, range2);
}

fn puzzle1(input_array: &[&str]) -> usize {
    return input_array.iter()
        .map(|string| parse_ranges(*string))
        .filter(|(range1, range2)| -> bool {
            return range1.encompasses(&range2) || range2.encompasses(&range1);
        })
        .count();
}

fn puzzle2(input_array: &[&str]) -> usize {
    return input_array.iter()
        .map(|string| parse_ranges(*string))
        .filter(|(range1, range2)| -> bool {
            return range1.overlaps(&range2) || range2.overlaps(&range1);
        })
        .count();
}

#[cfg(test)]
mod tests {
    use crate::day4::puzzle1;
    use crate::day4::puzzle2;
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 6] = ["2-4,6-8",
        "2-3,4-5",
        "5-7,7-9",
        "2-8,3-7",
        "6-6,4-6",
        "2-6,4-8"];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, 2);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day4.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str()));

        let return_value = puzzle1(&data[..]);
        assert_eq!(return_value, 530);
    }

    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA);
        assert_eq!(return_value, 4);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day4.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str()));

        let return_value = puzzle2(&data[..]);
        assert_eq!(return_value, 903);
    }
}