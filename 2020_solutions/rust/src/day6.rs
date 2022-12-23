use std::collections::HashSet;

fn parse_passports(input: Vec<String>) -> Vec::<Vec<String>> {
    let mut passports = Vec::<Vec<String>>::new();
    let mut values = Vec::<String>::new();

    for string in input {
        if (string.len() == 0) {
            passports.push(values);
            values = Vec::<String>::new();
        } else {
            values.push(string);
        }
    }

    if (!values.is_empty()) {
        passports.push(values);
    }

    return passports;
}

fn count_unique_caracters(survey: &Vec<String>) -> usize {
    let mut char_set = HashSet::<char>::new();

    for string in survey {
        for characters in string.chars() {
            char_set.insert(characters);
        }
    }

    return char_set.len();
}

fn puzzle1(input: Vec<String>) -> usize {
    let passports = parse_passports(input);

    return passports.iter()
        .map(|pp| count_unique_caracters(pp))
        .sum();
}

fn count_characters_in_all_surveys(survey: &Vec<String>) -> usize {
    let mut asdf: Vec<char> = survey.get(0).unwrap().chars().collect();
    for s in survey.iter().skip(1) {
        asdf.retain(|charac| s.contains(*charac));
    }
    return asdf.len();
}


fn puzzle2(input: Vec<String>) -> usize {
    let passports = parse_passports(input);

    return passports.iter()
        .map(|passport| count_characters_in_all_surveys(passport))
        .sum();
}

#[cfg(test)]
mod tests {
    use crate::day6::puzzle1;
    use crate::day6::puzzle2;
    use crate::utils::read_lines;

    #[test]
    fn test_puzzle_1() {
        let strs = vec!["abc",
                        "",
                        "a",
                        "b",
                        "c",
                        "",
                        "ab",
                        "ac",
                        "",
                        "a",
                        "a",
                        "a",
                        "a",
                        "",
                        "b",];

        let strings = strs.into_iter()
            .map(String::from)
            .collect();

        let result = puzzle1(strings);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_puzzle_1_2() {
        let strs = read_lines("data/Day6.txt").unwrap();

        let result = puzzle1(strs);
        assert_eq!(result, 6534);
    }

    #[test]
    fn test_puzzle_2() {
        let strs = vec!["abc",
                        "",
                        "a",
                        "b",
                        "c",
                        "",
                        "ab",
                        "ac",
                        "",
                        "a",
                        "a",
                        "a",
                        "a",
                        "",
                        "b",];

        let strings = strs.into_iter()
            .map(String::from)
            .collect();

        let result = puzzle2(strings);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_puzzle_2_2() {
        let strs = read_lines("data/Day6.txt").unwrap();

        let result = puzzle2(strs);
        assert_eq!(result, 3402);
    }
}