fn get_seat_number(string: &String) -> u16 {
    let mut seat_number = 0;

    string.chars()
        .take(7)
        .for_each(|char| {
            seat_number = seat_number << 1;
            if char == 'B' {
                seat_number = seat_number | 1;
            }
        });

    string.chars()
        .skip(7)
        .for_each(|char| {
            seat_number = seat_number << 1;
            if char == 'R' {
                seat_number = seat_number | 1;
            }
        });

    return seat_number;
}

fn puzzle1(input: Vec<String>) -> u16 {
    return input.iter()
        .map(get_seat_number)
        .max()
        .unwrap();
}

fn puzzle2(input: Vec<String>) -> u16 {
    let seat_numbers: Vec<u16> = input.iter()
        .map(get_seat_number)
        .collect();

    for value in 1..1024u16 {
        if (!seat_numbers.iter().any(|i| *i == value)) {
            if(seat_numbers.iter().any(|i| *i == value + 1)
             && seat_numbers.iter().any(|i| *i == value - 1)) {
                return value;
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day5::{puzzle1, puzzle2};
    use crate::utils::read_lines;

    #[test]
    fn test_puzzle_1() {
        let input = vec![String::from("FBFBBFFRLR")];

        let result = puzzle1(input);
        assert_eq!(result, 357);

        let input2 = vec![String::from("BFFFBBFRRR")];

        let result2 = puzzle1(input2);
        assert_eq!(result2, 567);

        let input3 = vec![String::from("FFFBBBFRRR")];

        let result3 = puzzle1(input3);
        assert_eq!(result3, 119);

        let input4 = vec![String::from("BBFFBBFRLL")];

        let result4 = puzzle1(input4);
        assert_eq!(result4, 820);
    }

    #[test]
    fn test_puzzle_1_2() {
        let input = read_lines("data/Day5.txt").unwrap();
        let result = puzzle1(input);
        assert_eq!(result, 874);
    }

    #[test]
    fn test_puzzle_2_2() {
        let input = read_lines("data/Day5.txt").unwrap();
        let result = puzzle2(input);
        assert_eq!(result, 594);
    }
}