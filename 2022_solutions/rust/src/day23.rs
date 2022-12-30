use std::collections::HashMap;
use crate::cartesian::{Direction, Point};

struct StatusQuerier {
    elf_north: Point<i64>,
    elf_south: Point<i64>,
    elf_west: Point<i64>,
    elf_east: Point<i64>,
    has_neighbor_north: bool,
    has_neighbor_south: bool,
    has_neighbor_west: bool,
    has_neighbor_east: bool,
    has_neighbor_nw: bool,
    has_neighbor_ne: bool,
    has_neighbor_sw: bool,
    has_neighbor_se: bool,

}

impl StatusQuerier {
    fn new(elf_location: &Point<i64>, elves: &HashMap<Point<i64>, usize>) -> Self {
        let elf_north = elf_location.up(1);
        let elf_south = elf_location.down(1);
        let elf_west = elf_location.left(1);
        let elf_east = elf_location.right(1);
        let elf_ne = elf_location.up(1).right(1);
        let elf_nw = elf_location.up(1).left(1);
        let elf_se = elf_location.down(1).right(1);
        let elf_sw = elf_location.down(1).left(1);

        let has_neighbor_north = elves.get(&elf_north).is_some();
        let has_neighbor_south = elves.get(&elf_south).is_some();
        let has_neighbor_east = elves.get(&elf_east).is_some();
        let has_neighbor_west = elves.get(&elf_west).is_some();
        let has_neighbor_ne = elves.get(&elf_ne).is_some();
        let has_neighbor_nw = elves.get(&elf_nw).is_some();
        let has_neighbor_se = elves.get(&elf_se).is_some();
        let has_neighbor_sw = elves.get(&elf_sw).is_some();

        Self {
            elf_north,
            elf_south,
            elf_west,
            elf_east,
            has_neighbor_north,
            has_neighbor_south,
            has_neighbor_east,
            has_neighbor_west,
            has_neighbor_ne,
            has_neighbor_nw,
            has_neighbor_se,
            has_neighbor_sw,
        }
    }

    fn has_adjoining_elf(&self) -> bool {
        return self.has_neighbor_north || self.has_neighbor_south || self.has_neighbor_east || self.has_neighbor_west
            || self.has_neighbor_nw || self.has_neighbor_ne || self.has_neighbor_sw || self.has_neighbor_se;
    }

    fn has_neighbors_in_direction(&self, dir: &Direction) -> bool {
        match dir {
            Direction::NORTH => self.has_neighbor_north || self.has_neighbor_nw || self.has_neighbor_ne,
            Direction::SOUTH => self.has_neighbor_south || self.has_neighbor_sw || self.has_neighbor_se,
            Direction::WEST => self.has_neighbor_west || self.has_neighbor_nw || self.has_neighbor_sw,
            Direction::EAST => self.has_neighbor_east || self.has_neighbor_ne || self.has_neighbor_se,
        }
    }

    pub fn get_next_in_direction(&self, p0: &Direction) -> Point<i64> {
        match p0 {
            Direction::NORTH => self.elf_north,
            Direction::SOUTH => self.elf_south,
            Direction::EAST => self.elf_east,
            Direction::WEST => self.elf_west
        }
    }
}

fn next_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::NORTH => Direction::SOUTH,
        Direction::SOUTH => Direction::WEST,
        Direction::WEST => Direction::EAST,
        Direction::EAST => Direction::NORTH,
    }
}

fn iterate_elves(elves: &mut HashMap<Point<i64>, usize>, initial_direction: &Direction) -> usize {
    let mut move_count = 0;
    let mut future_state = HashMap::<Point<i64>, Vec<usize>>::new();

    for (elf_location, elf_name) in (*elves).iter() {
        let querier = StatusQuerier::new(elf_location, elves);


        if querier.has_adjoining_elf() {
            if !querier.has_neighbors_in_direction(initial_direction) {
                let next = querier.get_next_in_direction(initial_direction);
                match future_state.get_mut(&next) {
                    Some(vector) => { vector.push(elf_name.clone()) }
                    None => { future_state.insert(next, vec![elf_name.clone()]); }
                }
            } else {
                let direction_2 = next_direction(initial_direction);
                if !querier.has_neighbors_in_direction(&direction_2) {
                    let next2 = querier.get_next_in_direction(&direction_2);
                    match future_state.get_mut(&next2) {
                        Some(vector) => { vector.push(elf_name.clone()) }
                        None => { future_state.insert(next2, vec![elf_name.clone()]); }
                    }
                } else {
                    let direction_3 = next_direction(&direction_2);
                    if !querier.has_neighbors_in_direction(&direction_3) {
                        let next3 = querier.get_next_in_direction(&direction_3);
                        match future_state.get_mut(&next3) {
                            Some(vector) => { vector.push(elf_name.clone()) }
                            None => { future_state.insert(next3, vec![elf_name.clone()]); }
                        }
                    } else {
                        let direction_4 = next_direction(&direction_3);
                        if !querier.has_neighbors_in_direction(&direction_4) {
                            let next4 = querier.get_next_in_direction(&direction_4);
                            match future_state.get_mut(&next4) {
                                Some(vector) => { vector.push(elf_name.clone()) }
                                None => { future_state.insert(next4, vec![elf_name.clone()]); }
                            }
                        }
                    }
                }
            }
        }
    }

    for (point, proposed_elves) in future_state {
        if proposed_elves.len() == 1 {
            move_count = move_count + 1;

            let elf_name = proposed_elves.get(0).unwrap();

            elves.retain(|key, val| *val != *elf_name);
            elves.insert(point, *elf_name);
        }
    }

    return move_count;
}

fn puzzle1(input_array: &[&str]) -> usize {
    let mut elves = HashMap::<Point<i64>, usize>::new();
    let mut elf_count = 0;
    for (row_index, input_string) in input_array.iter().rev().enumerate() {
        for (column_index, character) in input_string.chars().enumerate() {
            if character == '#' {
                elf_count = elf_count + 1;
                let point = Point::new(column_index as i64, row_index as i64);
                elves.insert(point, elf_count);
            }
        }
    }

    let mut initial_direction = Direction::NORTH;
    let mut movement_count = iterate_elves(&mut elves, &initial_direction);
    initial_direction = next_direction(&initial_direction);

    // while movement_count > 0 {
    for _ in 0..10 {
        movement_count = iterate_elves(&mut elves, &initial_direction);
        initial_direction = next_direction(&initial_direction);
    }

    let max_x = elves.iter().map(|(elf, index)| elf.x).max().unwrap();
    let min_x = elves.iter().map(|(elf, index)| elf.x).min().unwrap();
    let max_y = elves.iter().map(|(elf, index)| elf.y).max().unwrap();
    let min_y = elves.iter().map(|(elf, index)| elf.y).min().unwrap();

    let x_delta = (max_x - min_x) + 1;
    let y_delta = (max_y - min_y) + 1;
    let area = (x_delta * y_delta) as usize;


    if area >= elf_count {
        return area - elf_count;
    }
    return 0;
}

fn puzzle2(input_array: &[&str]) -> usize {
    let mut elves = HashMap::<Point<i64>, usize>::new();
    let mut elf_count = 0;
    for (row_index, input_string) in input_array.iter().rev().enumerate() {
        for (column_index, character) in input_string.chars().enumerate() {
            if character == '#' {
                elf_count = elf_count + 1;
                let point = Point::new(column_index as i64, row_index as i64);
                elves.insert(point, elf_count);
            }
        }
    }

    let mut rounds = 0;
    let mut initial_direction = Direction::NORTH;
    let mut movement_count = iterate_elves(&mut elves, &initial_direction);
    initial_direction = next_direction(&initial_direction);
    rounds = rounds + 1;

    while movement_count > 0 {
    // for _ in 0..10 {
        movement_count = iterate_elves(&mut elves, &initial_direction);
        initial_direction = next_direction(&initial_direction);
        rounds = rounds + 1;
    }

    return rounds;
}

#[cfg(test)]
mod tests {
    use crate::day23::{puzzle1, puzzle2};
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 6] = [
        ".....",
        "..##.",
        "..#..",
        ".....",
        "..##.",
        "....."];

    const TEST_DATA_2: [&str; 12] = [
        "..............",
        "..............",
        ".......#......",
        ".....###.#....",
        "...#...#.#....",
        "....#...##....",
        "...#.###......",
        "...##.#.##....",
        "....#..#......",
        "..............",
        "..............",
        ".............."
    ];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, 25);
    }

    #[test]
    fn test_puzzle1_a() {
        let return_value = puzzle1(&TEST_DATA_2);
        assert_eq!(return_value, 110);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day23.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle1(&data);
        assert_eq!(return_value, 3788);
    }


    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA_2);
        assert_eq!(return_value, 20);
    }

    #[test]
    #[ignore]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day23.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data);
        assert_eq!(return_value, 921);
    }
}