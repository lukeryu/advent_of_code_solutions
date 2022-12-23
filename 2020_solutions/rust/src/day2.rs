extern crate regex;

use self::regex::Regex;

lazy_static! {
    static ref PASSWORD_FORMAT: Regex = Regex::new(r"(?P<minimumCount>\d+)-(?P<maximumCount>\d+) (?P<character>\w): (?P<password>\w+)").unwrap();
}

#[derive(Clone)]
struct Password {
    minimum_count: usize,
    maximum_count: usize,
    character: char,
    password: String
}

impl Password {
    fn is_valid(&self) -> bool {
        let matching_characters = self.password.chars()
            .filter(|password_char| *password_char == self.character)
            .count();

        return self.minimum_count <= matching_characters && matching_characters <= self.maximum_count;
    }

    fn is_valid_2(&self) -> bool {
        let val1 = self.password.chars().nth(self.minimum_count - 1).unwrap();
        let val2 = self.password.chars().nth(self.maximum_count - 1).unwrap();

        return (val1 == self.character) ^ (val2 == self.character);
    }

    fn from(string: &String) -> Self {
        return PASSWORD_FORMAT.captures(string)
            .map(|captures| -> Self {
                let minimum_count = captures.name("minimumCount").unwrap().as_str().parse::<usize>().unwrap();
                let maximum_count = captures.name("maximumCount").unwrap().as_str().parse::<usize>().unwrap();
                let character = captures.name("character").unwrap().as_str().chars().last().unwrap();
                let password = String::from(captures.name("password").unwrap().as_str());

                Self {
                    minimum_count,
                    maximum_count,
                    character,
                    password
                }
            }).unwrap();
    }
}

fn puzzle1(values: Vec<String>) -> usize {
    return values.iter()
        .map(Password::from)
        .filter(|pw | pw.is_valid())
        .count();
}

fn puzzle2(values: Vec<String>) -> usize {
    return values.iter()
        .map(Password::from)
        .filter(|pw | pw.is_valid_2())
        .count();
}

#[cfg(test)]
mod tests {
    use crate::utils::read_lines;
    use crate::day2::puzzle1;
    use crate::day2::puzzle2;

    #[test]
    fn test_puzzle_1() {
        let values = vec![String::from("1-3 a: abcde"), String::from("1-3 b: cdefg"), String::from("2-9 c: ccccccccc")];

        let result = puzzle1(values);

        assert_eq!(result, 2)
    }

    #[test]
    fn test_puzzle_1_2() {
        let lines = read_lines("data/Day2.txt");

        match lines {
            Ok(string_vec) => {
                let result = puzzle1(string_vec);

                assert_eq!(result, 528);
            }
            Err(err) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_puzzle_2() {
        let values = vec![String::from("1-3 a: abcde"), String::from("1-3 b: cdefg"), String::from("2-9 c: ccccccccc")];

        let result = puzzle2(values);

        assert_eq!(result, 1)
    }

    #[test]
    fn test_puzzle_2_2() {
        let lines = read_lines("data/Day2.txt");

        match lines {
            Ok(string_vec) => {

                let result = puzzle2(string_vec);

                assert_eq!(result, 497);
            }
            Err(err) => {
                assert!(false);
            }
        }
    }
}