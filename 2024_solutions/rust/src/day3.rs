use regex::Regex;

fn puzzle1(input_array: &[&str]) -> usize {
    let mut sum = 0;
    let regex: Regex = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();

    for input in input_array {
        let captures = regex.captures_iter(input);

        for capture in captures {
            let first = capture["first"].parse::<usize>();
            let second = capture["second"].parse::<usize>();

            if first.is_ok() && second.is_ok() {
                let first_num = first.unwrap();
                let second_num = second.unwrap();
                sum = sum + (first_num * second_num);
            }
        }
    }

    return sum;
}

fn puzzle2(input_array: &[&str]) -> usize {
    let mut sum = 0;
    let mut is_enabled = true;
    let regex: Regex =
        Regex::new(r"(?<mul>mul)\((?<first>\d+),(?<second>\d+)\)|(?<do>do\(\))|(?<dont>don't\(\))")
            .unwrap();

    for input in input_array {
        let captures = regex.captures_iter(input);

        for capture in captures {
            let mul = capture.name("mul").is_some();
            let do_str = capture.name("do").is_some();
            let dont = capture.name("dont").is_some();

            if do_str {
                is_enabled = true;
            }

            if dont {
                is_enabled = false;
            }

            if mul {
                let first = capture["first"].parse::<usize>();
                let second = capture["second"].parse::<usize>();
                if first.is_ok() && second.is_ok() && is_enabled {
                    let first_num = first.unwrap();
                    let second_num = second.unwrap();
                    sum = sum + (first_num * second_num);
                }
            }
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use crate::day3::puzzle1;
    use crate::day3::puzzle2;
    use crate::utils::read_file_strings;

    const TEST_INPUT: [&str; 1] =
        ["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_INPUT);
        assert_eq!(return_value, 161);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day3.txt");
        vec.iter().for_each(|string| data.push(string.as_str()));

        let return_value = puzzle1(&data[..]);
        assert_eq!(return_value, 180233229); // 625 too high
    }

    #[test]
    fn test_puzzle2() {
        let input = ["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"];
        let return_value = puzzle2(&input);
        assert_eq!(return_value, 48);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day3.txt");
        vec.iter().for_each(|string| data.push(string.as_str()));

        let return_value = puzzle2(&data[..]);
        assert_eq!(return_value, 95411583);
    }
}
