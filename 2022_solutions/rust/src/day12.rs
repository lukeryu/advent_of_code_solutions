use std::collections::HashSet;
use crate::cartesian::Point;

struct Map {
    map_info: Vec<char>,
    width: usize,
    height: usize,
}

fn add1_char(c: char) -> char {
    std::char::from_u32(c as u32 + 1).unwrap_or(c)
}

impl Map {
    fn new(input: &[&str]) -> Self {
        let mut map_info = Vec::new();
        let height = input.len();
        let width = input[0].len();

        for row in input {
            for column in row.chars() {
                if column == 'S' {
                    map_info.push('a');
                } else if column == 'E' {
                    map_info.push('z');
                } else {
                    map_info.push(column);
                }
            }
        }

        return Self {
            map_info,
            width,
            height,
        };
    }

    fn get_next_moves(&self, point: &Point<usize>) -> Vec<Point<usize>> {
        let mut vec = Vec::<Point<usize>>::new();

        let point_value = self.get_value_at(point);

        if point.x > 0 {
            let left_point = Point::new(point.x - 1, point.y);

            let left_point_value = self.get_value_at(&left_point);

            if left_point_value <= point_value + 1 {
                vec.push(left_point);
            }
        }
        if point.y > 0 {
            let up_point = Point { x: point.x, y: point.y - 1 };

            let up_point_value = self.get_value_at(&up_point);

            if up_point_value <= point_value + 1 {
                vec.push(up_point);
            }
        }
        if point.x < self.width - 1 {
            let right_point =  Point { x: point.x + 1, y: point.y };
            let right_point_value = self.get_value_at(&right_point);

            if right_point_value <= point_value + 1 {
                vec.push(right_point);
            }
        }
        if point.y < self.height - 1 {
            let down_point = Point { x: point.x, y: point.y + 1 };
            let down_point_value = self.get_value_at(&down_point);

            if down_point_value <= point_value + 1 {
                vec.push(down_point);
            }
        }

        return vec;
    }

    fn get_value_at(&self, point: &Point<usize>) -> i32 {
        let index = point.y * self.width + point.x;
        return self.map_info[index] as i32;
    }
}


fn parse_map(input_array: &[&str]) -> (Point<usize>, Point<usize>, Map) {
    let mut start = Point { x: 0, y: 0 };
    let mut end = Point { x: 0, y: 0 };

    for (column_index, string) in input_array.iter().enumerate() {
        for (row_index, character) in string.chars().enumerate() {
            if 'S' == character {
                start = Point { x: row_index, y: column_index };
            }

            if 'E' == character {
                end = Point { x: row_index, y: column_index };
            }
        }
    }

    return (start, end, Map::new(input_array));
}

fn make_a_move(current_point: &Point<usize>, end_point: &Point<usize>, map: &Map, visited_set: HashSet<Point<usize>>) -> usize {
    if current_point == end_point || map.get_value_at(current_point) == 'z' as i32 {
        return 1;
    }

    let next_moves = map.get_next_moves(&current_point);

    let mut max_find = usize::MAX;

    for a_move in next_moves {
        if visited_set.contains(&a_move) {
            continue;
        }

        let mut visit_clone = visited_set.clone();
        visit_clone.insert((*current_point).clone());
        let move_cost = make_a_move(&a_move, end_point, map, visit_clone);
        if move_cost < max_find {
            max_find = move_cost + 1;
        }
    }

    return max_find;
}

fn puzzle1(input_array: &[&str]) -> usize {
    let (start_point, end_point, map) = parse_map(input_array);

    let move_count = make_a_move(&start_point, &end_point, &map, HashSet::new());

    return move_count;
}

fn puzzle2(input_array: &[&str]) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day12::{puzzle1, puzzle2};
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 5] = [
        "Sabqponm",
        "abcryxxl",
        "accszExk",
        "acctuvwj",
        "abdefghi"];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, 31);
    }

    #[test]
    #[ignore]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day12.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle1(&data);
        assert_eq!(return_value, 102399);
    }


    #[test]
    #[ignore]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA);
        assert_eq!(return_value, 2713310158);
    }

    #[test]
    #[ignore]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day12.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data);
        assert_eq!(return_value, 23641658401);
    }
}