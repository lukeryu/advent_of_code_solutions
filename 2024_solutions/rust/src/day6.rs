use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::Add;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position2d {
    x: i64,
    y: i64,
}

impl Add for Position2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl fmt::Display for Position2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

const LEFT: Position2d = Position2d { x: -1, y: 0 };
const RIGHT: Position2d = Position2d { x: 1, y: 0 };
const UP: Position2d = Position2d { x: 0, y: -1 };
const DOWN: Position2d = Position2d { x: 0, y: 1 };

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Vector {
    position: Position2d,
    direction: Position2d,
}

fn trace_steps(input_array: &[&str], starting_positions: Position2d) -> Vec<Vector> {
    let mut current = starting_positions.clone();
    let mut positions = Vec::<Vector>::new();
    let mut current_direction = UP;
    let max_row = input_array.len() as i64;
    let max_col = input_array[0].len() as i64;

    while is_in_bounds(&current, max_row, max_col) {
        println!("{}", current);
        positions.push(Vector {
            position: current.clone(),
            direction: current_direction.clone(),
        });

        let next = current.clone() + current_direction.clone();
        let row = input_array.iter().nth(next.y as usize);
        match row {
            Some(row) => match row.chars().nth(next.x as usize) {
                Some(char) => match char {
                    '#' => {
                        println!("{}", char);
                        current_direction = find_next_direction(current_direction);
                    }
                    _ => {
                        current = next;
                        println!("{}", char);
                    }
                },
                None => panic!("asdfasdf"),
            },
            _ => {
                if !is_in_bounds(&next, max_row, max_col) {
                    break;
                }
            }
        }
    }

    positions
}

fn puzzle1(input_array: &[&str]) -> usize {
    let current = find_start(input_array);
    let position_count = trace_steps(&input_array, current);

    position_count
        .iter()
        .map(|vector| vector.position)
        .collect::<HashSet<_>>()
        .len()
}

fn find_next_direction(p0: Position2d) -> Position2d {
    match p0 {
        LEFT => UP,
        RIGHT => DOWN,
        UP => RIGHT,
        DOWN => LEFT,
        _ => panic!("invalid direction"),
    }
}

fn is_in_bounds(p0: &Position2d, max_row: i64, max_column: i64) -> bool {
    p0.x >= 0 && p0.y >= 0 && p0.x < max_row && p0.y < max_column
}

fn find_start(input_array: &[&str]) -> Position2d {
    for (row_index, row) in input_array.iter().enumerate() {
        for (col_index, col) in row.chars().enumerate() {
            if col == '^' {
                return Position2d {
                    x: col_index as i64,
                    y: row_index as i64,
                };
            }
        }
    }

    panic!("Cannot Find Start")
}

fn puzzle2(input_array: &[&str]) -> usize {
    let current = find_start(input_array);
    let position_count = trace_steps(&input_array, current);

    let mut hash_map = HashMap::<Position2d, HashSet<Position2d>>::new();

    position_count.iter().for_each(|vector| {
        hash_map
            .entry(vector.position)
            .and_modify(|v| {
                v.insert(vector.direction);
            })
            .or_insert(new_hashset(vector.direction));
    });

    hash_map.values().filter(&has_right_turn).count()
}

fn has_right_turn(directions : &&HashSet<Position2d>) -> bool {
    if directions.contains(&RIGHT) && directions.contains(&DOWN) {
        return true;
    }

    if directions.contains(&LEFT) && directions.contains(&DOWN) {
        return true;
    }

    if directions.contains(&RIGHT) && directions.contains(&UP) {
        return true;
    }

    if directions.contains(&LEFT) && directions.contains(&UP) {
        return true;
    }

    false
}

fn new_hashset(p0: Position2d) -> HashSet<Position2d> {
    let mut hash_set = HashSet::<Position2d>::new();
    hash_set.insert(p0);
    return hash_set;
}

#[cfg(test)]
mod tests {
    use crate::day6::*;
    use crate::utils::read_file_strings;

    const EXAMPLE_INPUT: [&str; 10] = [
        "....#.....",
        ".........#",
        "..........",
        "..#.......",
        ".......#..",
        "..........",
        ".#..^.....",
        "........#.",
        "#.........",
        "......#...",
    ];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&EXAMPLE_INPUT);
        assert_eq!(return_value, 41);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day6.txt");
        vec.iter().for_each(|string| data.push(string.as_str()));

        let return_value = puzzle1(&data[..]);
        assert_eq!(return_value, 4778);
    }

    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&EXAMPLE_INPUT);
        assert_eq!(return_value, 6);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day6.txt");
        vec.iter().for_each(|string| data.push(string.as_str()));

        let return_value = puzzle2(&data[..]);
        assert_eq!(return_value, 6004);
    }
}
