use crate::day2::RPS::{PAPER, ROCK, SCISSORS};

#[derive(PartialEq, Eq, Clone, Copy)]
enum RPS {
    ROCK,
    PAPER,
    SCISSORS,
}

struct RPSRecord {
    lose_to: RPS,
    win_against: RPS,
    rps: RPS,
    code: &'static str,
    score: u64,
}

const RECORD_VALUES: [RPSRecord; 6] = [
    RPSRecord {
        code: "A",
        rps: ROCK,
        win_against: SCISSORS,
        lose_to: PAPER,
        score: 1,
    },
    RPSRecord {
        code: "X",
        rps: ROCK,
        win_against: SCISSORS,
        lose_to: PAPER,
        score: 1,
    },
    RPSRecord {
        code: "B",
        rps: PAPER,
        win_against: ROCK,
        lose_to: SCISSORS,
        score: 2,
    },
    RPSRecord {
        code: "Y",
        rps: PAPER,
        win_against: ROCK,
        lose_to: SCISSORS,
        score: 2,
    },
    RPSRecord {
        code: "C",
        rps: SCISSORS,
        win_against: PAPER,
        lose_to: ROCK,
        score: 3,
    },
    RPSRecord {
        code: "Z",
        rps: SCISSORS,
        win_against: PAPER,
        lose_to: ROCK,
        score: 3,
    },
];

fn get_rps(string: String) -> Option<&'static RPSRecord> {
    return RECORD_VALUES.iter()
        .find(|&record| record.code == string);
}

fn get_stategic_rps(opponent_rps: &RPSRecord, code_string: String) -> Option<&'static RPSRecord> {

    if code_string == "X" {
        //LOSE
        return RECORD_VALUES.iter().find(|&record| record.lose_to == opponent_rps.rps);
    }
    if code_string == "Y" {
        //DRAW
        return RECORD_VALUES.iter().find(|&record| record.rps == opponent_rps.rps);
    }
    if code_string == "Z" {
        //WIN
        return RECORD_VALUES.iter().find(|&record| record.win_against == opponent_rps.rps);
    }
    return None;
}

fn puzzle1(data: &[String]) -> u64 {
    let mut sum = 0;
    for datum in data {
        let mut strings = datum.trim().split(" ");
        let opp_string = strings.next().unwrap();
        let my_string = strings.next().unwrap();
        let opponent_rps = get_rps(String::from(opp_string)).unwrap();
        let my_rps = get_rps(String::from(my_string)).unwrap();
        sum += my_rps.score;
        if my_rps.win_against == opponent_rps.rps {
            sum += 6;
        } else if my_rps.rps == opponent_rps.rps {
            sum += 3;
        }
    }
    return sum;
}

fn puzzle2(data: &[String]) -> u64 {
    let mut sum = 0;
    for datum in data {
        let mut strings = datum.trim().split(" ");
        let opp_string = strings.next().unwrap();
        let my_string = strings.next().unwrap();
        let opponent_rps = get_rps(String::from(opp_string)).unwrap();
        let my_rps = get_stategic_rps(opponent_rps, String::from(my_string)).unwrap();
        sum += my_rps.score;
        if my_rps.win_against == opponent_rps.rps {
            sum += 6;
        } else if my_rps.rps == opponent_rps.rps {
            sum += 3;
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use crate::day2::puzzle1;
    use crate::day2::puzzle2;
    use crate::utils::read_file_strings;

    #[test]
    fn test_puzzle1() {
        let test_data = vec![String::from("A Y"),
                             String::from("B X"),
                             String::from("C Z")];

        let return_value = puzzle1(&test_data);
        assert_eq!(return_value, 15);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let data = read_file_strings("../data/Day2.txt");

        let return_value = puzzle1(&data);
        assert_eq!(return_value, 10941);
    }

    #[test]
    fn test_puzzle2() {
        let data = vec![String::from("A Y"),
                        String::from("B X"),
                        String::from("C Z")];

        let return_value = puzzle2(&data);
        assert_eq!(return_value, 12);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let data = read_file_strings("../data/Day2.txt");

        let return_value = puzzle2(&data);
        assert_eq!(return_value, 13071);
    }
}