use std::ops::Rem;
use std::collections::{BTreeSet, HashMap};

const UP: u16 = 1;
const DOWN: u16 = 2;
const LEFT: u16 = 4;
const RIGHT: u16 = 8;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
struct Grid {
    array: [bool; 25],
    index: i32,
}

fn to_bool(input: char) -> bool {
    return input == '#';
}

impl Grid {
    fn new(strings: Vec<String>) -> Option<Self> {
        let mut array = [false; 25];

        for string_index in 0..strings.len() {
            let string = strings.get(string_index)?;
            let indices = string.char_indices();
            for character_index in indices {
                array[string_index * 5 + character_index.0] = to_bool(character_index.1);
            }
        }

        return Some(Self {
            array,
            index: 0,
        });
    }

    fn new_empty(num: i32) -> Self {
        Self {
            array: [false; 25],
            index: num,
        }
    }

    fn next_iteration(self) -> Self {
        let mut new_array = self.array.clone();

        for index in 0..self.array.len() {
            let mut is_new_bug = self.array[index].clone();
            let count = self.adjacent_bug_count(index);
            if is_new_bug {
                if count != 1 {
                    is_new_bug = false;
                }
            } else {
                if count == 1 || count == 2 {
                    is_new_bug = true;
                }
            }
            new_array[index] = is_new_bug;
        }

        return Self {
            array: new_array,
            index: self.index,
        };
    }

    fn next_iteration_recursive(self, data_map: &HashMap<i32, Grid>) -> Self {
        let mut new_array = self.array.clone();

        for index in 0..self.array.len() {
            let mut is_new_bug = self.array[index].clone();
            let count = self.adjacent_bug_count_recursive(index, &data_map);
            if is_new_bug {
                if count != 1 {
                    is_new_bug = false;
                }
            } else {
                if count == 1 || count == 2 {
                    is_new_bug = true;
                }
            }
            new_array[index] = is_new_bug;
        }

        return Self {
            array: new_array,
            index: self.index,
        };
    }

    fn adjacent_bug_count_recursive(self, index: usize, data_map: &HashMap<i32, Grid>) -> u8 {
        if index == 12 {
            return 0;
        }

        let mut sum = 0;

        sum += self.up_count(index, &data_map);
        sum += self.left_count(index, &data_map);
        sum += self.right_count(index, &data_map);
        sum += self.down_count(index, &data_map);

        return sum;
    }

    fn up_count(self, index: usize, data_map: &HashMap<i32, Grid>) -> u8 {
        if index == 17 {
            return match data_map.get(&(self.index - 1)) {
                Some(parent_grid) => {
                    let mut sum = 0;
                    if parent_grid.array[20] { sum = sum + 1 }
                    if parent_grid.array[21] { sum = sum + 1 }
                    if parent_grid.array[22] { sum = sum + 1 }
                    if parent_grid.array[23] { sum = sum + 1 }
                    if parent_grid.array[24] { sum = sum + 1 }
                    return sum;
                }
                None => 0
            };
        }
        return if index > 4 {
            if self.array[index - 5] {
                1
            } else {
                0
            }
        } else {
            match data_map.get(&(self.index + 1)) {
                Some(parent_grid) => { if parent_grid.array[7] { 1 } else { 0 } }
                None => 0
            }
        }

        return 0;
    }
    fn left_count(self, index: usize, data_map: &HashMap<i32, Grid>) -> u8 {
        if index == 13 {
            return match data_map.get(&(self.index - 1)) {
                Some(parent_grid) => {
                    let mut sum = 0;
                    if parent_grid.array[4] { sum = sum + 1 }
                    if parent_grid.array[9] { sum = sum + 1 }
                    if parent_grid.array[14] { sum = sum + 1 }
                    if parent_grid.array[19] { sum = sum + 1 }
                    if parent_grid.array[24] { sum = sum + 1 }
                    return sum;
                }
                None => 0
            };
        }
        return if index % 5 != 0 {
            if self.array[index - 1] {
                1
            } else {
                0
            }
        } else {
            match data_map.get(&(self.index + 1)) {
                Some(parent_grid) => { if parent_grid.array[11] { 1 } else { 0 } }
                None => 0
            }
        }

        return 0;
    }
    fn right_count(self, index: usize, data_map: &HashMap<i32, Grid>) -> u8 {
        if index == 11 {
            return match data_map.get(&(self.index - 1)) {
                Some(parent_grid) => {
                    let mut sum = 0;
                    if parent_grid.array[0] { sum = sum + 1 }
                    if parent_grid.array[5] { sum = sum + 1 }
                    if parent_grid.array[10] { sum = sum + 1 }
                    if parent_grid.array[15] { sum = sum + 1 }
                    if parent_grid.array[20] { sum = sum + 1 }
                    return sum;
                }
                None => 0
            };
        }
        return if index % 5 != 4 {
            if self.array[index + 1] {
                1
            } else {
                0
            }
        } else {
            match data_map.get(&(self.index + 1)) {
                Some(parent_grid) => { if parent_grid.array[13] { 1 } else { 0 } }
                None => 0
            }
        }

        return 0;
    }
    fn down_count(self, index: usize, data_map: &HashMap<i32, Grid>) -> u8 {
        if index == 7 {
            return match data_map.get(&(self.index - 1)) {
                Some(parent_grid) => {
                    let mut sum = 0;
                    if parent_grid.array[0] { sum = sum + 1 }
                    if parent_grid.array[1] { sum = sum + 1 }
                    if parent_grid.array[2] { sum = sum + 1 }
                    if parent_grid.array[3] { sum = sum + 1 }
                    if parent_grid.array[4] { sum = sum + 1 }
                    return sum;
                }
                None => 0
            };
        }
        return if index < 20 {
            if self.array[index + 5] {
                1
            } else {
                0
            }
        } else {
            match data_map.get(&(self.index + 1)) {
                Some(parent_grid) => { if parent_grid.array[17] { 1 } else { 0 } }
                None => 0
            }
        }

        return 0;
    }


    fn one_for_true(self, index: usize, direction: u16) -> u8 {
        let mut adjacent_index = index.clone();
        if (direction & UP) != 0 && index > 4 {
            adjacent_index = adjacent_index - 5;
        }
        if (direction & DOWN) != 0 && index < 20 {
            adjacent_index = adjacent_index + 5;
        }
        let remainder = index % 5;
        if (direction & LEFT) != 0 && remainder != 0 {
            adjacent_index = adjacent_index - 1;
        }
        if (direction & RIGHT) != 0 && remainder != 4 {
            adjacent_index = adjacent_index + 1;
        }

        if index != adjacent_index && self.array[adjacent_index] {
            return 1;
        }
        return 0;
    }

    fn adjacent_bug_count(self, index: usize) -> u8 {
        let mut sum = 0;
        sum += self.one_for_true(index, UP);
        sum += self.one_for_true(index, LEFT);
        sum += self.one_for_true(index, RIGHT);
        sum += self.one_for_true(index, DOWN);

        return sum;
    }

    fn calculate_biodiversity_rating(self) -> u32 {
        let base = 2u32;

        let mut sum = 0;
        for i in 0..self.array.len() {
            if self.array[i] {
                sum += base.pow(i as u32);
            }
        }
        return sum;
    }

    fn bug_count(self) -> u32 {
        return self.array.iter()
            .map(|value| -> u32 {
                return if value.clone() {
                    1u32
                } else {
                    0u32
                };
            })
            .sum();
    }

    fn is_empty(self) -> bool {
        return self.bug_count() == 0;
    }
}


fn puzzle1(strings: Vec<String>) -> Option<u32> {
    let mut grids = BTreeSet::new();
    let mut grid = Grid::new(strings)?;
    grids.insert(grid.clone());

    loop {
        let next_grid = grid.next_iteration();

        if grids.contains(&next_grid) {
            return Some(next_grid.calculate_biodiversity_rating());
        }
        grids.insert(next_grid.clone());
        grid = next_grid;
    }
}

fn run_iteration(depth_map: &HashMap<i32, Grid>) -> HashMap<i32, Grid> {
    let mut next_depth_map = HashMap::<i32, Grid>::new();
    for (key, value) in depth_map.iter() {
        next_depth_map.insert(key.clone(), value.next_iteration_recursive(&depth_map));
    }
    return next_depth_map;
}

fn get_total_bug_count(depth_map: HashMap<i32, Grid>) -> Option<u32> {
    let sum = depth_map.values()
        .map(|grid| -> u32 { grid.bug_count() })
        .sum();
    return Some(sum);
}

fn puzzle2(strings: Vec<String>, iterations: u32) -> Option<u32> {
    let mut depth_map = HashMap::<i32, Grid>::new();
    let mut grid = Grid::new(strings)?;
    depth_map.insert(0, grid);

    for num in 0..iterations {
        let i_num = (num as i32 + 1);
        depth_map.insert(i_num, Grid::new_empty(i_num));
        depth_map.insert(-1 * i_num, Grid::new_empty(-1 * i_num));
        depth_map = run_iteration(&depth_map);
    }

    return get_total_bug_count(depth_map);
}

#[cfg(test)]
mod tests {
    use crate::day24::{puzzle1, Grid, puzzle2};

    struct Test1 {
        data: Vec<String>,
        expected: u32,
    }

    #[test]
    fn test_puzzle1() {
        let mut tests = Vec::<Test1>::new();

        tests.push(Test1 {
            data: vec![String::from("....#"),
                       String::from("#..#."),
                       String::from("#..##"),
                       String::from("..#.."),
                       String::from("#....")],
            expected: 2129920,
        });

        tests.push(Test1 {
            data: vec![String::from("####."),
                       String::from(".###."),
                       String::from(".#..#"),
                       String::from("##.##"),
                       String::from("###..")],
            expected: 32511025,
        });

        for test in tests {
            assert_eq!(puzzle1(test.data), Some(test.expected));
        }
    }

    #[test]
    fn test_adjust() {
        let grid_option = Grid::new(vec![String::from("....#"),
                                         String::from("#..#."),
                                         String::from("#..##"),
                                         String::from("..#.."),
                                         String::from("#....")]);
        match grid_option {
            Some(grid) => {
                let count = grid.adjacent_bug_count(5);
                assert_eq!(count, 1);
            }
            None => assert!(false)
        }
    }

    struct Test2 {
        data: Vec<String>,
        iterations: u32,
        expected: u32,
    }

    #[test]
    fn test_puzzle2() {
        let mut tests = Vec::<Test2>::new();

        tests.push(Test2 {
            data: vec![String::from("....#"),
                       String::from("#..#."),
                       String::from("#..##"),
                       String::from("..#.."),
                       String::from("#....")],
            iterations: 10,
            expected: 99,
        });

        tests.push(Test2 {
            data: vec![String::from("####."),
                       String::from(".###."),
                       String::from(".#..#"),
                       String::from("##.##"),
                       String::from("###..")],
            iterations: 200,
            expected: 1932,
        });

        for test in tests {
            assert_eq!(puzzle2(test.data, test.iterations), Some(test.expected));
        }
    }
}