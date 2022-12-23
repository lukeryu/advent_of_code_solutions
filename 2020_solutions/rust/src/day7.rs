extern crate regex;

use self::regex::Regex;

lazy_static! {
    static ref PASSWORD_FORMAT: Regex = Regex::new(r"(?P<color>\d+) bags contain (?P<color>\d \w) (, \d \w)\.").unwrap();
}


fn puzzle1(input: Vec<String>) -> u32 {
    return 0;
}


#[cfg(test)]
mod tests {
    use crate::day7::puzzle1;

    #[test]
    fn test_puzzle_1() {
        let rows: Vec<&str> = vec!["light red bags contain 1 bright white bag, 2 muted yellow bags.",
                        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
                        "bright white bags contain 1 shiny gold bag.",
                        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
                        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
                        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
                        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
                        "faded blue bags contain no other bags.",
                        "dotted black bags contain no other bags."];
        let stringRows = rows.iter()
            .map(|row| String::from(*row))
            .collect();
        let result = puzzle1(stringRows);

        assert_eq!(result, 4);
    }
}