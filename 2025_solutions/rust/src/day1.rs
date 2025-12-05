use anyhow::Result;

fn parse_line_amt(p0: String) -> i32 {
    let char = p0.chars().next().unwrap();
    let mut num = p0[1..].parse::<i32>().unwrap();
    num = num % 100;

    if char == 'L' {
        num = num * -1;
    }

    num
}

fn parse_line_amt2(p0: String) -> i32 {
    let char = p0.chars().next().unwrap();
    let mut num = p0[1..].parse::<i32>().unwrap();

    if char == 'L' {
        num = num * -1;
    }

    num
}
fn puzzle1(input: Vec<String>) -> usize {
    let mut value: i32 = 50;
    let mut occurence_zero = 0;

    for line in input {
        let line_amt = parse_line_amt(line);
        value = value + line_amt;
        if value < 0 {
            value = value + 100;
        }
        if value > 99 {
            value = value - 100;
        }
        if value == 0 {
            occurence_zero = occurence_zero + 1;
        }
    }

    return occurence_zero;
}

fn puzzle2(input: Vec<String>) -> usize {
    let mut value: i32 = 50;
    let mut occurence_zero: usize = 0;

    for line in input {
        let line_amt = parse_line_amt2(line);
        let line_thing = (line_amt.abs() / 100) as usize;
        let line_remainder = line_amt % 100;
        occurence_zero = occurence_zero + line_thing;
        value = value + line_remainder;
        if value < 0 {
            value = value + 100;
            occurence_zero = occurence_zero + 1;
        }
        if value > 99 {
            value = value - 100;
            occurence_zero = occurence_zero + 1;
        }
    }

    return occurence_zero;
}

fn part2(input: Vec<String>) -> Result<u64> {
    let mut acc = 50_u32 + (u32::MAX / 2).next_multiple_of(100);
    let mut count = 0_u64;
    for l in input {
        let bytes = l.as_bytes();
        let num: u32 = unsafe { str::from_utf8_unchecked(&bytes[1..]) }.parse()?;
        let old_acc = acc;
        match bytes[0] {
            b'R' => {
                acc += num;
                count +=
                    u32::abs_diff(u32::div_euclid(old_acc, 100), u32::div_euclid(acc, 100)) as u64;
            }
            b'L' => {
                acc -= num;
                count += u32::abs_diff(u32::div_ceil(old_acc, 100), u32::div_ceil(acc, 100)) as u64;
            }
            _ => unreachable!(),
        }
    }
    Ok(count)
}

mod tests {
    use super::*;
    use crate::utils::read_file_strings;
    #[test]
    fn test_puzzle1() {
        let input: Vec<String> = vec![
            String::from("L68"),
            String::from("L30"),
            String::from("R48"),
            String::from("L5"),
            String::from("R60"),
            String::from("L55"),
            String::from("L1"),
            String::from("L99"),
            String::from("R14"),
            String::from("L82"),
        ];

        assert_eq!(puzzle1(input), 3);
    }

    #[test]
    fn test_puzzle1_data() {
        let input: Vec<String> = read_file_strings("../data/Day1.txt");
        let filter_input = input.into_iter()
            .filter(|line| !line.is_empty())
            .collect::<Vec<String>>();

        assert_eq!(puzzle1(filter_input), 1081);
    }

    #[test]
    fn test_puzzle2() {
        let input: Vec<String> = vec![
            String::from("L68"),
            String::from("L30"),
            String::from("R48"),
            String::from("L5"),
            String::from("R60"),
            String::from("L55"),
            String::from("L1"),
            String::from("L99"),
            String::from("R14"),
            String::from("L82"),
        ];

        assert_eq!(puzzle2(input), 6);
    }

    #[test]
    fn test_puzzle2_data() {
        let input: Vec<String> = read_file_strings("../data/Day1.txt");
        let filter_input = input.into_iter()
            .filter(|line| !line.is_empty())
            .collect::<Vec<String>>();
        
        assert_eq!(part2(filter_input).unwrap(), 6689);
    }
}
