#[derive(Eq, PartialEq)]
enum PassportFieldType {
    byr,
    iyr,
    eyr,
    hgt,
    hcl,
    ecl,
    pid,
    cid,
}

impl PassportFieldType {
    fn from(string: &str) -> Option<Self> {
        return match string {
            "byr" => Option::Some(PassportFieldType::byr),
            "iyr" => Option::Some(PassportFieldType::iyr),
            "eyr" => Option::Some(PassportFieldType::eyr),
            "hgt" => Option::Some(PassportFieldType::hgt),
            "hcl" => Option::Some(PassportFieldType::hcl),
            "ecl" => Option::Some(PassportFieldType::ecl),
            "pid" => Option::Some(PassportFieldType::pid),
            "cid" => Option::Some(PassportFieldType::cid),
            _ => Option::None
        };
    }
}


struct PassportFieldValue {
    field_type: PassportFieldType,
    field_value: String,
}

impl PassportFieldValue {
    fn new(string: &str) -> Self {
        let mut halfs = string.split(":");
        let field_type = halfs.next().unwrap();
        let field_value = halfs.next().unwrap();

        return Self {
            field_type: PassportFieldType::from(field_type).unwrap(),
            field_value: String::from(field_value),
        };
    }

    fn is_valid(&self) -> bool {
        return match self.field_type {
            PassportFieldType::byr => true,
            PassportFieldType::iyr => true,
            PassportFieldType::eyr => true,
            PassportFieldType::hgt => true,
            // PassportFieldType::hcl => self.field_value.matches("#[\\da-f]{9}").collect().len() > 0,
            // PassportFieldType::ecl => self.field_value.matches("amb|blu|brn|gry|grn|hzl|oth").collect().len() > 0,
            // PassportFieldType::pid => self.field_value.matches("\\d{9}").collect().len() > 0,
            PassportFieldType::cid => true,
            _ => true
        };
    }
}

struct Passport {
    field_values: Vec<PassportFieldValue>
}

impl Passport {
    fn has_valid_fields(&self) -> bool {
        let len = self.field_values.len();
        if (len == 8) {
            return true;
        }
        if (len == 7 && !self.has_cid()) {
            return true;
        }
        return false;
    }

    fn has_cid(&self) -> bool {
        return self.field_values.iter()
            .any(|value| (&value).field_type == PassportFieldType::cid);
    }

    fn new(input: Vec<String>) -> Self {
        let mut field_values = Vec::<PassportFieldValue>::new();

        for string in input {
            let splits = string.split(" ");
            for split in splits {
                field_values.push(PassportFieldValue::new(split));
            }
        }

        return Self {
            field_values: field_values
        };
    }

    fn is_valid(&self) -> bool {
        return self.has_valid_fields()
            && self.field_values.iter()
            .all(PassportFieldValue::is_valid);
    }
}

fn parse_passports(input: Vec<String>) -> Vec<Passport> {
    let mut passports = Vec::<Passport>::new();
    let mut values = Vec::<String>::new();

    for string in input {
        if (string.len() == 0) {
            passports.push(Passport::new(values));
            values = Vec::<String>::new();
        } else {
            values.push(string);
        }
    }

    if (!values.is_empty()) {
        passports.push(Passport::new(values));
    }

    return passports;
}


fn puzzle1(input: Vec<String>) -> usize {
    let passports = parse_passports(input);

    return passports.iter()
        .filter(|passport| passport.has_valid_fields())
        .count();
}

fn puzzle2(input: Vec<String>) -> usize {
    let passports = parse_passports(input);

    return passports.iter()
        .filter(|passport| passport.is_valid())
        .count();
}

#[cfg(test)]
mod tests {
    use crate::day4::puzzle1;
    use crate::utils::read_lines;

    #[test]
    fn test_puzzle_1() {
        let strs = vec!["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
                        "byr:1937 iyr:2017 cid:147 hgt:183cm",
                        "",
                        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
                        "hcl:#cfa07d byr:1929",
                        "",
                        "hcl:#ae17e1 iyr:2013",
                        "eyr:2024",
                        "ecl:brn pid:760753108 byr:1931",
                        "hgt:179cm",
                        "",
                        "hcl:#cfa07d eyr:2025 pid:166559648",
                        "iyr:2011 ecl:brn hgt:59in"];

        let strings = strs.into_iter()
            .map(String::from)
            .collect();

        let result = puzzle1(strings);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_puzzle_1_2() {
        let strs = read_lines("data/Day4.txt").unwrap();

        let result = puzzle1(strs);
        assert_eq!(result, 222);
    }

    #[test]
    fn test_puzzle_2_valid() {
        let strs = vec!["pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
                        "hcl:#623a2f",
                        "",
                        "eyr:2029 ecl:blu cid:129 byr:1989",
                        "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
                        "",
                        "hcl:#888785",
                        "hgt:164cm byr:2001 iyr:2015 cid:88",
                        "pid:545766238 ecl:hzl",
                        "eyr:2022",
                        "",
                        "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"];

        let strings = strs.into_iter()
            .map(String::from)
            .collect();

        let result = puzzle1(strings);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_puzzle_2_invalid() {
        let strs = vec!["eyr:1972 cid:100",
                        "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
                        "",
                        "iyr:2019",
                        "hcl:#602927 eyr:1967 hgt:170cm",
                        "ecl:grn pid:012533040 byr:1946",
                        "",
                        "hcl:dab227 iyr:2012",
                        "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
                        "",
                        "hgt:59cm ecl:zzz",
                        "eyr:2038 hcl:74454a iyr:2023",
                        "pid:3556412378 byr:2007"];

        let strings = strs.into_iter()
            .map(String::from)
            .collect();

        let result = puzzle1(strings);
        assert_eq!(result, 0);
    }
}