use std::str::Chars;

const SEARCH_WORD: &str = "XMAS";

fn should_count(
    row_index: i64,
    column_index: i64,
    indicies: &mut Chars,
    input_array: &[&str],
    row_inc: i64,
    col_inc: i64,
    num_of_rows: i64,
    num_of_columns: i64,
) -> bool {
    match indicies.next() {
        Some(char) => {
            if row_index < 0 {
                return false;
            }
            if column_index < 0 {
                return false;
            }
            if row_index >= num_of_rows {
                return false;
            }
            if column_index >= num_of_columns {
                return false;
            }
            match input_array[row_index as usize]
                .chars()
                .nth(column_index as usize)
            {
                Some(char2) => {
                    if char != char2 {
                        return false;
                    }
                    should_count(
                        row_index + row_inc,
                        column_index + col_inc,
                        indicies,
                        input_array,
                        row_inc,
                        col_inc,
                        num_of_rows,
                        num_of_columns,
                    )
                }
                None => false,
            }
        }
        None => true,
    }
}

fn puzzle1(input_array: &[&str]) -> usize {
    let num_of_rows = input_array.len() as i64;
    let num_of_columns = input_array[0].len() as i64;
    let mut count = 0;
    for (row_index, row) in input_array.iter().enumerate() {
        for (column_index, column) in row.chars().enumerate() {
            if column != 'X' {
                continue;
            }
            let row_index_i64 = row_index as i64;
            let column_index_i64 = column_index as i64;

            if should_count(
                row_index_i64,
                column_index_i64,
                &mut SEARCH_WORD.chars(),
                input_array,
                0,
                1,
                num_of_rows,
                num_of_columns,
            ) {
                count += 1;
            }
            if should_count(
                row_index_i64,
                column_index_i64,
                &mut SEARCH_WORD.chars(),
                input_array,
                0,
                -1,
                num_of_rows,
                num_of_columns,
            ) {
                count += 1;
            }
            if should_count(
                row_index_i64,
                column_index_i64,
                &mut SEARCH_WORD.chars(),
                input_array,
                1,
                1,
                num_of_rows,
                num_of_columns,
            ) {
                count += 1;
            }
            if should_count(
                row_index_i64,
                column_index_i64,
                &mut SEARCH_WORD.chars(),
                input_array,
                1,
                -1,
                num_of_rows,
                num_of_columns,
            ) {
                count += 1;
            }
            if should_count(
                row_index_i64,
                column_index_i64,
                &mut SEARCH_WORD.chars(),
                input_array,
                1,
                0,
                num_of_rows,
                num_of_columns,
            ) {
                count += 1;
            }
            if should_count(
                row_index_i64,
                column_index_i64,
                &mut SEARCH_WORD.chars(),
                input_array,
                -1,
                0,
                num_of_rows,
                num_of_columns,
            ) {
                count += 1;
            }
            if should_count(
                row_index_i64,
                column_index_i64,
                &mut SEARCH_WORD.chars(),
                input_array,
                -1,
                1,
                num_of_rows,
                num_of_columns,
            ) {
                count += 1;
            }
            if should_count(
                row_index_i64,
                column_index_i64,
                &mut SEARCH_WORD.chars(),
                input_array,
                -1,
                -1,
                num_of_rows,
                num_of_columns,
            ) {
                count += 1;
            }
        }
    }
    return count;
}

const MAS_STRING: &str = "MAS";
fn puzzle2(input_array: &[&str]) -> usize {
    let num_of_rows = input_array.len() as i64;
    let num_of_columns = input_array[0].len() as i64;
    let mut count = 0;
    for (row_index, row) in input_array.iter().enumerate() {
        for (column_index, column) in row.chars().enumerate() {
            if column != 'A' {
                continue;
            }
            let row_index_i64 = row_index as i64;
            let column_index_i64 = column_index as i64;

            if should_count(
                row_index_i64 + 1,
                column_index_i64 + 1,
                &mut MAS_STRING.chars(),
                input_array,
                -1,
                -1,
                num_of_rows,
                num_of_columns,
            ) && (should_count(
                row_index_i64 + 1,
                column_index_i64 - 1,
                &mut MAS_STRING.chars(),
                input_array,
                -1,
                1,
                num_of_rows,
                num_of_columns,
            ) || should_count(
                row_index_i64 - 1,
                column_index_i64 + 1,
                &mut MAS_STRING.chars(),
                input_array,
                1,
                -1,
                num_of_rows,
                num_of_columns,
            )) {
                count += 1;
            }

            if should_count(
                row_index_i64 - 1,
                column_index_i64 - 1,
                &mut MAS_STRING.chars(),
                input_array,
                1,
                1,
                num_of_rows,
                num_of_columns,
            ) && (should_count(
                row_index_i64 + 1,
                column_index_i64 - 1,
                &mut MAS_STRING.chars(),
                input_array,
                -1,
                1,
                num_of_rows,
                num_of_columns,
            ) || should_count(
                row_index_i64 - 1,
                column_index_i64 + 1,
                &mut MAS_STRING.chars(),
                input_array,
                1,
                -1,
                num_of_rows,
                num_of_columns,
            )) {
                count += 1;
            }
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use crate::day4::puzzle1;
    use crate::day4::puzzle2;
    use crate::utils::read_file_strings;

    const TEST_INPUT: [&str; 10] = [
        "MMMSXXMASM",
        "MSAMXMSMSA",
        "AMXSXMAAMM",
        "MSAMASMSMX",
        "XMASAMXAMM",
        "XXAMMXXAMA",
        "SMSMSASXSS",
        "SAXAMASAAA",
        "MAMMMXMMMM",
        "MXMXAXMASX",
    ];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_INPUT);
        assert_eq!(return_value, 18);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day4.txt");
        vec.iter().for_each(|string| data.push(string.as_str()));

        let return_value = puzzle1(&data[..]);
        assert_eq!(return_value, 2642);
    }

    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_INPUT);
        assert_eq!(return_value, 9);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day4.txt");
        vec.iter().for_each(|string| data.push(string.as_str()));

        let return_value = puzzle2(&data[..]);
        assert_eq!(return_value, 1974);
    }
}
