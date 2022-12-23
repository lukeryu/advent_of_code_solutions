use std::fmt;

enum TurnDirection {
    RIGHT,
    LEFT,
}

enum CardinalDirection {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}


    fn fmt(f: &CardinalDirection) -> String {
        match f {
            CardinalDirection::NORTH => String::from("NORTH"),
            CardinalDirection::SOUTH => String::from("SOUTH"),
            CardinalDirection::EAST => String::from("EAST"),
            CardinalDirection::WEST => String::from("WEST"),
        }
    }


fn parseDirection(string: &str) -> (TurnDirection, i64) {
    let direction_char = string.chars()
        .nth(0)
        .unwrap();

    let turn_direction;
    if direction_char == 'R' {
        turn_direction = TurnDirection::RIGHT;
    } else {
        turn_direction = TurnDirection::LEFT;
    }
    
    let direction_char = string.chars().nth(1).unwrap().to_string().parse::<i64>().unwrap();

    return (turn_direction, direction_char)
}

fn turn(cardinal: &CardinalDirection, turn_dir: &TurnDirection) -> CardinalDirection {
    return match cardinal {
        CardinalDirection::NORTH => {
            match turn_dir {
                TurnDirection::LEFT => CardinalDirection::WEST,
                TurnDirection::RIGHT => CardinalDirection::EAST,
            }
        },
        CardinalDirection::SOUTH => {
            match turn_dir {
                TurnDirection::LEFT => CardinalDirection::EAST,
                TurnDirection::RIGHT => CardinalDirection::WEST,
            }
        },
        CardinalDirection::EAST => {
            match turn_dir {
                TurnDirection::LEFT => CardinalDirection::NORTH,
                TurnDirection::RIGHT => CardinalDirection::SOUTH,
            }
        },
        CardinalDirection::WEST => {
            match turn_dir {
                TurnDirection::LEFT => CardinalDirection::SOUTH,
                TurnDirection::RIGHT => CardinalDirection::NORTH,
            }
        },
        _ => panic!("What is this cardinal?")
    }
}

fn puzzle1(input_array: &[&str]) -> i64 {
    let input : Vec<&str> = input_array[0].split(",")
        .map(|string| string.trim())
        .collect();

        let mut current_cardinal = CardinalDirection::NORTH;
        let mut x_axis_amount:i64 = 0;
        let mut y_axis_amount: i64 = 0;

        for string in input {
            let (turnDir, amount) = parseDirection(string);
            current_cardinal = turn(&current_cardinal, &turnDir);
            match current_cardinal {
                CardinalDirection::NORTH => {
                    y_axis_amount += amount;
                },
                CardinalDirection::SOUTH => {
                    y_axis_amount -= amount;
                },
                CardinalDirection::EAST => {
                    x_axis_amount += amount;
                },
                CardinalDirection::WEST => {
                    x_axis_amount -= amount;
                },
                _ => panic!("What is this cardinal?")
            }
        }


    return x_axis_amount.abs() + y_axis_amount.abs();
}

fn puzzle2(input_array: &[&str]) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day1::puzzle1;
    use crate::day1::puzzle2;
    use crate::utils::read_file_strings;

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&["R2, L3"]);
        assert_eq!(return_value, 5);
    }

    #[test]
    fn test_puzzle1_2() {
        let return_value = puzzle1(&["R2, R2, R2"]);
        assert_eq!(return_value, 2);
    }

    #[test]
    fn test_puzzle1_3() {
        let return_value = puzzle1(&["R5, L5, R5, R3"]);
        assert_eq!(return_value, 12);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("data/Day1.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str()));

        let return_value = puzzle1(&data[..]);
        assert_eq!(return_value, 243);
    }

    #[test]
    #[ignore]
    fn test_puzzle2() {
        let return_value = puzzle2(&[]);
        assert_eq!(return_value, 100);
    }

    #[test]
    #[ignore]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day1.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str()));

        let return_value = puzzle2(&data[..]);
        assert_eq!(return_value, 100);
    }
}