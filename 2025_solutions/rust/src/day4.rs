use anyhow::Result;
use std::ops::IndexMut;

struct Point {
    row_index: usize,
    column_index: usize,
}

impl Point {
    fn new(row_index: usize, column_index: usize) -> Self {
        Self {
            row_index,
            column_index,
        }
    }
}
fn puzzle1(input: Vec<String>) -> Result<usize> {
    let mut count = 0;

    let row_count = input.len();
    let column_count = input[0].len();

    for (row_index, row) in input.iter().enumerate() {
        for (col_index, character) in row.chars().enumerate() {
            if character == '@' {
                if is_forkliftable(row_index, col_index, row_count, column_count, &input) {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

fn is_forkliftable(
    row_index: usize,
    column_index: usize,
    row_count: usize,
    column_count: usize,
    input: &Vec<String>,
) -> bool {
    let mut count = 0;

    if row_index > 0 {
        count = count + is_roll_of_paper(row_index - 1, column_index, &input);
        if (column_index + 1) < column_count {
            count = count + is_roll_of_paper(row_index - 1, column_index + 1, &input);
        }
        if column_index > 0 {
            count = count + is_roll_of_paper(row_index - 1, column_index - 1, &input);
        }
    }
    if (column_index + 1) < column_count {
        count = count + is_roll_of_paper(row_index, column_index + 1, &input);
    }
    if column_index > 0 {
        count = count + is_roll_of_paper(row_index, column_index - 1, &input);
    }
    if (row_index + 1) < row_count {
        count = count + is_roll_of_paper(row_index + 1, column_index, &input);
        if (column_index + 1) < column_count {
            count = count + is_roll_of_paper(row_index + 1, column_index + 1, &input);
        }
        if column_index > 0 {
            count = count + is_roll_of_paper(row_index + 1, column_index - 1, &input);
        }
    }

    return count < 4;
}

fn is_roll_of_paper(row_index: usize, column_index: usize, input: &Vec<String>) -> usize {
    let row = input.get(row_index).unwrap();
    let value = row.chars().nth(column_index).unwrap();
    if value == '@' { 1 } else { 0 }
}

fn puzzle2(input: Vec<String>) -> Result<usize> {
    let row_count = input.len();
    let column_count = input[0].len();

    let mut editable_input = input.clone();

    let mut total_removed_count = 0;
    let mut last_round_count = usize::MAX;
    while last_round_count > 0 {
        let mut position_set = Vec::<Point>::new();
        for (row_index, row) in editable_input.iter().enumerate() {
            for (col_index, character) in row.chars().enumerate() {
                if character == '@' {
                    if is_forkliftable(
                        row_index,
                        col_index,
                        row_count,
                        column_count,
                        &editable_input,
                    ) {
                        position_set.push(Point::new(row_index, col_index));
                    }
                }
            }
        }

        position_set.iter().for_each(|point| {
            remove_roll(point.row_index, point.column_index, &mut editable_input);
        });

        last_round_count = position_set.len();
        total_removed_count += position_set.len();
    }

    Ok(total_removed_count)
}

fn remove_roll(row_index: usize, col_index: usize, input: &mut Vec<String>) {
    let row = input.index_mut(row_index);
    row.replace_range(col_index..(col_index + 1), ".");
}

mod tests {
    use super::*;
    use crate::utils::read_file_strings;
    #[test]
    fn test_puzzle1() {
        let input: Vec<String> = vec![
            String::from("..@@.@@@@."),
            String::from("@@@.@.@.@@"),
            String::from("@@@@@.@.@@"),
            String::from("@.@@@@..@."),
            String::from("@@.@@@@.@@"),
            String::from(".@@@@@@@.@"),
            String::from(".@.@.@.@@@"),
            String::from("@.@@@.@@@@"),
            String::from(".@@@@@@@@."),
            String::from("@.@.@@@.@."),
        ];

        assert_eq!(puzzle1(input).unwrap(), 13);
    }

    #[test]
    fn test_puzzle1_data() {
        let input: Vec<String> = read_file_strings("../data/Day4.txt");
        let filter_input = input
            .into_iter()
            .filter(|line| !line.is_empty())
            .collect::<Vec<String>>();

        assert_eq!(puzzle1(filter_input).unwrap(), 1602);
    }

    #[test]
    fn test_puzzle2() {
        let input: Vec<String> = vec![
            String::from("..@@.@@@@."),
            String::from("@@@.@.@.@@"),
            String::from("@@@@@.@.@@"),
            String::from("@.@@@@..@."),
            String::from("@@.@@@@.@@"),
            String::from(".@@@@@@@.@"),
            String::from(".@.@.@.@@@"),
            String::from("@.@@@.@@@@"),
            String::from(".@@@@@@@@."),
            String::from("@.@.@@@.@."),
        ];

        assert_eq!(puzzle2(input).unwrap(), 43);
    }

    #[test]
    fn test_puzzle2_data() {
        let input: Vec<String> = read_file_strings("../data/Day4.txt");
        let filter_input = input
            .into_iter()
            .filter(|line| !line.is_empty())
            .collect::<Vec<String>>();

        assert_eq!(puzzle2(filter_input).unwrap(), 37432260594);
    }

    #[test]
    fn test_remove_roll() {
        let mut vec = vec![
            String::from("@@@"),
            String::from("@@@"),
            String::from("@@@"),
        ];

        remove_roll(1, 1, &mut vec);

        assert_eq!(vec, vec![
            String::from("@@@"),
            String::from("@.@"),
            String::from("@@@"),
        ]);
    }
}
