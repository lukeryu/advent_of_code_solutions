use std::cmp::{max, min};
use std::collections::HashSet;
use crate::cartesian::Point;


pub fn points_between(self_point: &Point<i128>, other_point: &Point<i128>) -> HashSet<Point<i128>> {
    let mut set = HashSet::new();

    if self_point.x == other_point.x {
        let min = min(self_point.y, other_point.y);
        let max = max(self_point.y, other_point.y);

        for y_value in min..=max {
            set.insert(Point::new(self_point.x, y_value));
        }
    } else {
        let min = min(self_point.x, other_point.x);
        let max = max(self_point.x, other_point.x);

        for x_value in min..=max {
            set.insert(Point::new(x_value, self_point.y));
        }
    }


    return set;
}

fn parse_input(input_array: &[&str]) -> HashSet<Point<i128>> {
    let mut point_set = HashSet::<Point<i128>>::new();

    for input_string in input_array {
        let mut previous: Option<Point<i128>> = None;
        for (index, token) in input_string.split_whitespace().enumerate() {
            if index % 2 == 0 {
                let mut stuff = token.split(",");
                let x_string = stuff.next().unwrap();
                let y_string = stuff.next().unwrap();
                let x_value = x_string.parse::<i128>().unwrap();
                let y_value = y_string.parse::<i128>().unwrap();
                let point = Point::new(x_value, y_value);

                match previous {
                    Some(prev_point) => {
                        let between_set = points_between(&point, &prev_point);
                        point_set.extend(between_set);
                    }
                    _ => {}
                }

                previous = Some(point);
            }
        }
    }

    return point_set;
}

fn drop_down(point: Point<i128>, point_set: &mut HashSet<Point<i128>>, max_depth: i128) -> bool {
    if point.y > max_depth {
        return true;
    }
    let down = point.up(1);
    if point_set.contains(&down) {
        let down_left = down.left(1);
        if point_set.contains(&down_left) {
            let down_right = down.right(1);
            if point_set.contains(&down_right) {
                point_set.insert(point);
                return false;
            } else {
                return drop_down(down_right, point_set, max_depth);
            }
        } else {
            return drop_down(down_left, point_set, max_depth);
        }
    } else {
        return drop_down(down, point_set, max_depth);
    }
}

fn drop_down_2(point: Point<i128>, point_set: &mut HashSet<Point<i128>>, max_depth: i128) -> Point<i128> {
    if point.y == (max_depth + 1) {
        point_set.insert(point);
        return point;
    }
    let down = point.up(1);
    if point_set.contains(&down) {
        let down_left = down.left(1);
        if point_set.contains(&down_left) {
            let down_right = down.right(1);
            if point_set.contains(&down_right) {
                point_set.insert(point);
                return point;
            } else {
                return drop_down_2(down_right, point_set, max_depth);
            }
        } else {
            return drop_down_2(down_left, point_set, max_depth);
        }
    } else {
        return drop_down_2(down, point_set, max_depth);
    }
}

fn puzzle1(input_array: &[&str]) -> usize {
    let mut sand_count = 0;
    let mut set = parse_input(input_array);

    let max_depth = set.iter()
        .map(|point| point.y)
        .max().unwrap();

    loop {
        let mut point = Point::new(500, 0);
        let asdf = drop_down(point, &mut set, max_depth);
        if asdf {
            break;
        }
        sand_count = sand_count + 1;
    }

    return sand_count;
}

fn puzzle2(input_array: &[&str]) -> usize {
    let mut sand_count = 0;
    let mut set = parse_input(input_array);

    let max_depth = set.iter()
        .map(|point| point.y)
        .max().unwrap();

    loop {
        sand_count = sand_count + 1;
        let mut point = Point::new(500, 0);
        let asdf = drop_down_2(point, &mut set, max_depth);
        if asdf == Point::new(500, 0) {
            break;
        }
    }

    return sand_count;
}


#[cfg(test)]
mod tests {
    use crate::day14::{puzzle1, puzzle2};
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 2] = [
        "498,4 -> 498,6 -> 496,6",
        "503,4 -> 502,4 -> 502,9 -> 494,9"];


    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, 24);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day14.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle1(&data);
        assert_eq!(return_value, 1513);
    }

    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA);
        assert_eq!(return_value, 93);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day14.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data);
        assert_eq!(return_value, 22646);
    }
}