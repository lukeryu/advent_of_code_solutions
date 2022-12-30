fn from_snafu(input_string: &str) -> u128 {
    return 0;
}

fn to_snafu(decimal: u128) -> String {
    return String::new();
}

fn puzzle1(input_array: &[&str]) -> String {
    let sum = input_array.iter()
        .map(|snfu| from_snafu(*snfu))
        .sum();

    return to_snafu(sum);
}


#[cfg(test)]
mod tests {
    use crate::day25::{from_snafu, puzzle1, to_snafu};
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 13] = [
        "1=-0-2",
        "12111",
        "2=0=",
        "21",
        "2=01",
        "111",
        "20012",
        "112",
        "1=-1=",
        "1-12",
        "12",
        "1=",
        "122"];

    const TRANSLATIONS: [(u128, &str); 28] = [
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
        (1747, "1=-0-2"),
        (906, "12111"),
        (198, "2=0="),
        (11, "21"),
        (201, "2=01"),
        (31, "111"),
        (1257, "20012"),
        (32, "112"),
        (353, "1=-1="),
        (107, "1-12"),
        (7, "12"),
        (3, "1="),
        (37, "122")
    ];


    #[test]
    fn test_from_snafu() {
        for (decimal, snafu) in TRANSLATIONS {
            let decimal_result = from_snafu(snafu);
            assert_eq!(decimal, decimal_result)
        }

    }

    #[test]
    fn test_to_snafu() {
        for (decimal, snafu) in TRANSLATIONS {
            let snafu_result = to_snafu(decimal);
            assert_eq!(snafu, snafu_result.as_str())
        }

    }


    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, "2=-1=0");
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day25.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle1(&data);
        assert_eq!(return_value, "3788");
    }

}