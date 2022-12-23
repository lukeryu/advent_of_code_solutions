use std::cmp::max;
use std::collections::{HashSet};
use std::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x_pos: i32,
    y_pos: i32,
}

impl Point {
    fn new(x_pos: i32, y_pos: i32) -> Self {
        return Self {
            x_pos,
            y_pos,
        };
    }

    fn distance(&self, other_point: &Point) -> i32 {
        let x_dis = (self.x_pos - other_point.x_pos).abs();
        let y_dis = (self.y_pos - other_point.y_pos).abs();
        return max(x_dis, y_dis);
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x_pos={}, y_pos={})", self.x_pos, self.y_pos)
    }
}

fn move_all_points(direction: &str, point_trail: &Vec<Point>) -> Vec<Point> {
    let mut array = vec![Point::new(0, 0); point_trail.len()];
    if direction == "U" {
        array[0] = Point::new(point_trail[0].x_pos, point_trail[0].y_pos + 1);
    } else if direction == "D" {
        array[0] = Point::new(point_trail[0].x_pos, point_trail[0].y_pos - 1);
    } else if direction == "L" {
        array[0] = Point::new(point_trail[0].x_pos - 1, point_trail[0].y_pos);
    } else if direction == "R" {
        array[0] = Point::new(point_trail[0].x_pos + 1, point_trail[0].y_pos);
    } else {
        panic!("Unknown Direction");
    }

    for index in 1..array.len() as usize {
        if array[index - 1].distance(&point_trail[index]) > 1 {
            array[index] = Point::new(
                point_trail[index].x_pos + (array[index - 1].x_pos - point_trail[index].x_pos).signum(),
                point_trail[index].y_pos + (array[index - 1].y_pos - point_trail[index].y_pos).signum(),
            );
        } else {
            array[index] = point_trail[index].clone();
        }
    }
    return array;
}

fn puzzle1(input_array: &[&str]) -> usize {
    let mut point_set = HashSet::<Point>::new();

    const LENGTH: usize = 2;
    let mut point_trail = vec![Point::new(0, 0); LENGTH];

    point_set.insert(point_trail[1]);

    for input_string in input_array {
        let mut fields = input_string.split_whitespace();
        let direction = fields.next().unwrap();
        let num_of_steps = fields.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..num_of_steps {
            let results = move_all_points(direction, &point_trail);
            point_trail = results;

            point_set.insert(point_trail[1]);
        }
    }

    return point_set.len();
}

fn puzzle2(input_array: &[&str]) -> usize {
    let mut point_set = HashSet::<Point>::new();

    const LENGTH: usize = 10;

    let mut point_trail = vec![Point::new(0, 0); LENGTH];

    point_set.insert(Point::new(0, 0));

    for input_string in input_array {
        let mut fields = input_string.split_whitespace();
        let direction = fields.next().unwrap();
        let num_of_steps = fields.next().unwrap().parse::<i32>().unwrap();

        for _step_num in 0..num_of_steps {
            let updated_points = move_all_points(direction, &point_trail);
            point_trail = updated_points;

            point_set.insert(point_trail[9]);

        }
    }
    return point_set.len();
}

#[cfg(test)]
mod tests {
    use crate::day9::{move_all_points, Point, puzzle1};
    use crate::day9::puzzle2;
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 8] = [
        "R 4",
        "U 4",
        "L 3",
        "D 1",
        "R 4",
        "D 1",
        "L 5",
        "R 2"];

    const TEST_DATA_2: [&str; 8] = [
        "R 5",
        "U 8",
        "L 8",
        "D 3",
        "R 17",
        "D 10",
        "L 25",
        "U 20"];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, 13);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day9.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle1(&data[..]);
        assert_eq!(return_value, 5735);
    }

    #[test]
    fn test_move_all_points() {
        let input = vec![
            Point::new(4, 1),
            Point::new(3, 0),
            Point::new(2, 0),
            Point::new(1, 0),
            Point::new(0, 0),
            Point::new(0, 0),
            Point::new(0, 0),
            Point::new(0, 0),
            Point::new(0, 0),
            Point::new(0, 0),
        ];
        let expected_output = vec![
            Point::new(4, 2),
            Point::new(4, 1),
            Point::new(3, 1),
            Point::new(2, 1),
            Point::new(1, 1),
            Point::new(0, 0),
            Point::new(0, 0),
            Point::new(0, 0),
            Point::new(0, 0),
            Point::new(0, 0),
        ];
        let output = move_all_points("U", &input);

        assert_eq!(output, expected_output);
    }


    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA);
        assert_eq!(return_value, 1);
    }

    #[test]
    fn test_puzzle2_more() {
        let return_value = puzzle2(&TEST_DATA_2);
        assert_eq!(return_value, 36);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day9.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data[..]);
        assert_eq!(return_value, 2478);
    }
}