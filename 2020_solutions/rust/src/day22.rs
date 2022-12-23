use std::collections::VecDeque;
use crate::day22::PlayerWinner::PLAYER2;
use std::thread::current;

enum PlayerWinner {
    PLAYER1,
    PLAYER2
}

struct Puzzle2Result {
    player: PlayerWinner,
    count: usize,
}

#[derive(Eq, PartialEq)]
struct GameState {
    player1_state: VecDeque<usize>,
    player2_state: VecDeque<usize>,
}

impl GameState {
    fn new(player1_state: VecDeque<usize>,
           player2_state: VecDeque<usize>) -> Self {
        Self{
            player1_state,
            player2_state
        }
    }
}

fn count_score(player_hand: &VecDeque<usize>) -> usize {
    return player_hand.iter().rev()
        .enumerate()
        .map(|(index, value)| {
            let indexusize = (index as usize) + 1;
            let value_val = *value;

            return indexusize * value_val;
        })
        .sum();
}

fn puzzle1(player1: &mut VecDeque<usize>, player2: &mut VecDeque<usize>) -> usize {
    while (!player1.is_empty() && !player2.is_empty()) {
        let player1_value = player1.pop_front().unwrap();
        let player2_value = player2.pop_front().unwrap();

        if (player1_value > player2_value) {
            player1.push_back(player1_value);
            player1.push_back(player2_value);
        } else {
            player2.push_back(player2_value);
            player2.push_back(player1_value);
        }
    }

    if (player1.is_empty()) {
        return count_score(player2);
    } else {
        return count_score(player1);
    }
}

fn puzzle2(player1: &mut VecDeque<usize>, player2: &mut VecDeque<usize>, game_number: u32) -> Puzzle2Result {
    println!("=== Game {} ===", game_number);
    let mut round = 1;
    let mut previous_states = Vec::<GameState>::new();

    let mut current_state = GameState::new(player1.clone(), player2.clone());

    while (!player1.is_empty() && !player2.is_empty()
        && !previous_states.iter().any(|state| *state == current_state)) {
        println!("-- Round {} (Game {}) --", round, game_number);
        println!("Player 1's Deck: {:?}", player1);
        println!("Player 2's Deck: {:?}", player2);

        previous_states.push(current_state);
        let player1_value = player1.pop_front().unwrap();
        let player2_value = player2.pop_front().unwrap();

        println!("Player 1 plays: {:?}", player1_value);
        println!("Player 2 plays: {:?}", player2_value);

        let winner;
        if (player1_value <= player1.len() && player2_value <= player2.len()) {
            println!("Playing a sub-game to determine the winner...");
            let mut player1_clone = player1.clone();
            let mut player2_cline = player2.clone();
            winner = puzzle2(&mut player1_clone, &mut player2_cline, game_number + 1).player;
        } else if (player1_value > player2_value) {
            winner = PlayerWinner::PLAYER1;
        } else {
            winner = PlayerWinner::PLAYER2;
        }
        match winner {
            PlayerWinner::PLAYER1 => {
                println!("Player 1 wins round {} of game {}!", round, game_number);
                player1.push_back(player1_value);
                player1.push_back(player2_value);
            },
            PlayerWinner::PLAYER2 => {
                println!("Player 2 wins round {} of game {}!", round, game_number);
                player2.push_back(player2_value);
                player2.push_back(player1_value);
            }
        }
        current_state = GameState::new(player1.clone(), player2.clone());
        round += 1;
    }

    println!("...anyway, back to game {}.", game_number);
    if (player1.is_empty()) {
        return Puzzle2Result {
            player: PlayerWinner::PLAYER2,
            count: count_score(player2)
        };
    } else {
        return Puzzle2Result {
            player: PlayerWinner::PLAYER1,
            count: count_score(player1)
        };
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use crate::day22::{puzzle1, puzzle2};

    #[test]
    fn test_puzzle1() {
        let mut player1 = VecDeque::<usize>::new();
        player1.push_back(9);
        player1.push_back(2);
        player1.push_back(6);
        player1.push_back(3);
        player1.push_back(1);

        let mut player2 = VecDeque::<usize>::new();
        player2.push_back(5);
        player2.push_back(8);
        player2.push_back(4);
        player2.push_back(7);
        player2.push_back(10);

        let result = puzzle1(&mut player1, &mut player2);
        assert_eq!(result, 306);
    }

    #[test]
    fn test_puzzle1_run() {
        let mut player1 = VecDeque::<usize>::new();
        player1.push_back(18);
        player1.push_back(50);
        player1.push_back(9);
        player1.push_back(4);
        player1.push_back(25);
        player1.push_back(37);
        player1.push_back(39);
        player1.push_back(40);
        player1.push_back(29);
        player1.push_back(6);
        player1.push_back(41);
        player1.push_back(28);
        player1.push_back(3);
        player1.push_back(11);
        player1.push_back(31);
        player1.push_back(8);
        player1.push_back(1);
        player1.push_back(38);
        player1.push_back(33);
        player1.push_back(30);
        player1.push_back(42);
        player1.push_back(15);
        player1.push_back(26);
        player1.push_back(36);
        player1.push_back(43);

        let mut player2 = VecDeque::<usize>::new();
        player2.push_back(32);
        player2.push_back(44);
        player2.push_back(19);
        player2.push_back(47);
        player2.push_back(12);
        player2.push_back(48);
        player2.push_back(14);
        player2.push_back(2);
        player2.push_back(13);
        player2.push_back(10);
        player2.push_back(35);
        player2.push_back(45);
        player2.push_back(34);
        player2.push_back(7);
        player2.push_back(5);
        player2.push_back(17);
        player2.push_back(46);
        player2.push_back(21);
        player2.push_back(24);
        player2.push_back(49);
        player2.push_back(16);
        player2.push_back(22);
        player2.push_back(20);
        player2.push_back(27);
        player2.push_back(23);

        let result = puzzle1(&mut player1, &mut player2);
        assert_eq!(result, 33561);
    }

    #[test]
    fn test_puzzle2() {
        let mut player1 = VecDeque::<usize>::new();
        player1.push_back(9);
        player1.push_back(2);
        player1.push_back(6);
        player1.push_back(3);
        player1.push_back(1);

        let mut player2 = VecDeque::<usize>::new();
        player2.push_back(5);
        player2.push_back(8);
        player2.push_back(4);
        player2.push_back(7);
        player2.push_back(10);

        let result = puzzle2(&mut player1, &mut player2, 1);
        assert_eq!(result.count, 291);
    }

    #[test]
    fn test_puzzle2_run() {
        let mut player1 = VecDeque::<usize>::new();
        player1.push_back(18);
        player1.push_back(50);
        player1.push_back(9);
        player1.push_back(4);
        player1.push_back(25);
        player1.push_back(37);
        player1.push_back(39);
        player1.push_back(40);
        player1.push_back(29);
        player1.push_back(6);
        player1.push_back(41);
        player1.push_back(28);
        player1.push_back(3);
        player1.push_back(11);
        player1.push_back(31);
        player1.push_back(8);
        player1.push_back(1);
        player1.push_back(38);
        player1.push_back(33);
        player1.push_back(30);
        player1.push_back(42);
        player1.push_back(15);
        player1.push_back(26);
        player1.push_back(36);
        player1.push_back(43);

        let mut player2 = VecDeque::<usize>::new();
        player2.push_back(32);
        player2.push_back(44);
        player2.push_back(19);
        player2.push_back(47);
        player2.push_back(12);
        player2.push_back(48);
        player2.push_back(14);
        player2.push_back(2);
        player2.push_back(13);
        player2.push_back(10);
        player2.push_back(35);
        player2.push_back(45);
        player2.push_back(34);
        player2.push_back(7);
        player2.push_back(5);
        player2.push_back(17);
        player2.push_back(46);
        player2.push_back(21);
        player2.push_back(24);
        player2.push_back(49);
        player2.push_back(16);
        player2.push_back(22);
        player2.push_back(20);
        player2.push_back(27);
        player2.push_back(23);

        let result = puzzle2(&mut player1, &mut player2, 1);
        assert_eq!(result.count, 33561);
    }
}