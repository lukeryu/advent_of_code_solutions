use std::collections::VecDeque;

use regex::{Error, Regex};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
enum ShuffleType {
    DEAL_INTO,
    CUT,
    DEAL_WITH,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
struct Shuffle {
    shuffle_type: ShuffleType,
    index: i32,
}

impl Shuffle {
    fn new(string: String) -> Option<Self> {
        let deal_into_regex: Result<Regex, Error> = Regex::new("deal into new stack");

        if deal_into_regex.ok()?.is_match(string.as_str()) {
            return Some(Self {
                shuffle_type: ShuffleType::DEAL_INTO,
                index: 0,
            });
        }

        let cut_regex: Result<Regex, Error> = Regex::new(r"cut (?P<xdim>-?\d+)");
        match cut_regex.ok()?.captures(string.as_str()) {
            Some(captures) => {
                return Some(Self {
                    shuffle_type: ShuffleType::CUT,
                    index: captures.name("xdim").unwrap().as_str().parse::<i32>().unwrap(),
                });
            }
            None => {}
        }

        let deal_with_increment_regex: Result<Regex, Error> = Regex::new(r"deal with increment (?P<xdim>-?\d+)");
        match deal_with_increment_regex.ok()?.captures(string.as_str()) {
            Some(captures) => {
                return Some(Self {
                    shuffle_type: ShuffleType::DEAL_WITH,
                    index: captures.name("xdim").unwrap().as_str().parse::<i32>().unwrap(),
                });
            }
            None => {}
        }

        return None;
    }
    fn do_shuffle(self, card_deck: Vec<u32>) -> Vec<u32> {
        return match self.shuffle_type {
            ShuffleType::DEAL_INTO => {
                let mut new_vec = Vec::from(card_deck);
                new_vec.reverse();
                return new_vec;
            },
            ShuffleType::CUT => {

                let mut new_vec = Vec::with_capacity(card_deck.len());

                let usize_index = self.index.abs() as usize;
                if self.index > 0 {
                    let cut_cards = &card_deck[..usize_index];
                    let your_deck = &card_deck[usize_index..];

                    new_vec.extend_from_slice(your_deck);
                    new_vec.extend_from_slice(cut_cards);
                } else {
                    let cut_cards = &card_deck[..(card_deck.len() - usize_index)];
                    let your_deck = &card_deck[(card_deck.len() - usize_index)..];

                    new_vec.extend_from_slice(your_deck);
                    new_vec.extend_from_slice(cut_cards);
                }
                return new_vec;
            },
            ShuffleType::DEAL_WITH => {
                let mut new_vec = vec![0; card_deck.len()];

                let mut current_index = 0;
                let number_of_cards = card_deck.len();
                for card_value in card_deck {
                    new_vec[current_index] = card_value;

                    current_index = current_index + (self.index as usize);
                    if current_index >= number_of_cards {
                        current_index = current_index % number_of_cards;
                    }
                }

                return new_vec;
            }
            _ => {
                Vec::from(card_deck)
            }
        };
    }
}

fn puzzle1(shuffle_instructions: Vec<String>, number_of_cards: u32) -> Vec<u32> {
    let shuffles: Vec<Shuffle> = shuffle_instructions.into_iter()
        .map(Shuffle::new)
        .map(Option::unwrap)
        .collect();

    let mut card_deck = initialize_card_deck(number_of_cards);

    for shuffle in shuffles {
        card_deck = shuffle.do_shuffle(card_deck);
    }
    return card_deck;
}

fn initialize_card_deck(number_of_cards: u32) -> Vec<u32> {
    return (0..number_of_cards).collect();
}

#[cfg(test)]
mod tests {
    use crate::day22::puzzle1;
    use crate::utils::read_lines;

    struct Test1 {
        instructions: Vec<String>,
        expected: Vec<u32>,
    }

    #[test]
    fn test_puzzle1() {
        let mut tests = Vec::<Test1>::new();

        tests.push(Test1 {
            instructions: vec![String::from("deal with increment 7"),
                               String::from("deal into new stack"),
                               String::from("deal into new stack")],
            expected: vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
        });

        tests.push(Test1 {
            instructions: vec![String::from("cut 6"),
                               String::from("deal with increment 7"),
                               String::from("deal into new stack")],
            expected: vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6],
        });


        tests.push(Test1 {
            instructions: vec![String::from("deal with increment 7"),
                               String::from("deal with increment 9"),
                               String::from("cut -2")],
            expected: vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9],
        });

        tests.push(Test1 {
            instructions: vec![String::from("deal into new stack"),
                               String::from("cut -2"),
                               String::from("deal with increment 7"),
                               String::from("cut 8"),
                               String::from("cut -4"),
                               String::from("deal with increment 7"),
                               String::from("cut 3"),
                               String::from("deal with increment 9"),
                               String::from("deal with increment 3"),
                               String::from("cut -1")],
            expected: vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6],
        });

        for test in tests {
            assert_eq!(puzzle1(test.instructions, 10), test.expected)
        }

        let lines = read_lines("data/Day22.txt");

        let result = puzzle1(lines.unwrap(), 10007);

        let index = result.iter().position(|value| *value == 2019).unwrap();
        assert_eq!(2322, index)
    }
}