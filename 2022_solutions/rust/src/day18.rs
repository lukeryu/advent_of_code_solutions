use std::cmp::{max, Ordering};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ops::{Range, SubAssign};

use crate::cartesian::Point3;

fn puzzle1(input_array: &[&str]) -> usize {
    let mut hashmap = HashMap::<Point3<i64>, usize>::new();

    for input_string in input_array {
        let mut split = (*input_string).split(",");
        let x = split.next().unwrap().parse::<i64>().unwrap();
        let y = split.next().unwrap().parse::<i64>().unwrap();
        let z = split.next().unwrap().parse::<i64>().unwrap();

        let point3 = Point3::new(x, y, z);

        let mut count = 6;

        match hashmap.get_mut(&point3.up(1)) {
            Some(value) => {
                value.sub_assign(1);
                count = count - 1;
            }
            _ => {}
        }

        match hashmap.get_mut(&point3.down(1)) {
            Some(value) => {
                value.sub_assign(1);
                count = count - 1;
            }
            _ => {}
        }

        match hashmap.get_mut(&point3.left(1)) {
            Some(value) => {
                value.sub_assign(1);
                count = count - 1;
            }
            _ => {}
        }

        match hashmap.get_mut(&point3.right(1)) {
            Some(value) => {
                value.sub_assign(1);
                count = count - 1;
            }
            _ => {}
        }

        match hashmap.get_mut(&point3.forward(1)) {
            Some(value) => {
                value.sub_assign(1);
                count = count - 1;
            }
            _ => {}
        }

        match hashmap.get_mut(&point3.back(1)) {
            Some(value) => {
                value.sub_assign(1);
                count = count - 1;
            }
            _ => {}
        }

        hashmap.insert(point3, count);
    }

    let value_sums = hashmap.values()
        .into_iter()
        .sum();

    return value_sums;
}

fn is_edge_face(movement_function: fn(&Point3<i64>, i64) -> Point3<i64>,
                value_function: fn(&Point3<i64>) -> i64,
                current_point: Point3<i64>,
                valid_range: Range<i64>,
                all_points: &BTreeSet<Point3<i64>>) -> bool {
    let next_point = movement_function(&current_point, 1);
    let exists = all_points.contains(&next_point);
    if exists {
        return false;
    }

    let value = value_function(&next_point);
    if !valid_range.contains(&value) {
        return true;
    }

    return is_edge_face(movement_function, value_function, next_point, valid_range, all_points);
}

fn puzzle2(input_array: &[&str]) -> usize {
    let mut hashmap = BTreeMap::<Point3<i64>, usize>::new();

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    for input_string in input_array {
        let mut split = (*input_string).split(",");
        let x = split.next().unwrap().parse::<i64>().unwrap();
        let y = split.next().unwrap().parse::<i64>().unwrap();
        let z = split.next().unwrap().parse::<i64>().unwrap();

        max_x = max(max_x, x);
        max_y = max(max_y, y);
        max_z = max(max_z, z);

        let point3 = Point3::new(x, y, z);

        hashmap.insert(point3, 6);
    }

    let keys: BTreeSet<Point3<i64>> = hashmap.keys()
        .map(|r| r.clone())
        .collect();

    for (key, value) in hashmap.iter_mut() {
        let a = is_edge_face(Point3::up, Point3::get_y, key.clone(), 0..max_y+1, &keys);
        let b = is_edge_face(Point3::down, Point3::get_y, key.clone(), 0..max_y+1, &keys);
        let c = is_edge_face(Point3::left, Point3::get_x, key.clone(), 0..max_x+1, &keys);
        let d = is_edge_face(Point3::right, Point3::get_x, key.clone(), 0..max_x+1, &keys);
        let e = is_edge_face(Point3::forward, Point3::get_z, key.clone(), 0..max_z+1, &keys);
        let f = is_edge_face(Point3::back, Point3::get_z, key.clone(), 0..max_z+1, &keys);

        if !a {
            value.sub_assign(1);
        }

        if !b {
            value.sub_assign(1);
        }
        if !c {
            value.sub_assign(1);
        }
        if !d {
            value.sub_assign(1);
        }
        if !e {
            value.sub_assign(1);
        }
        if !f {
            value.sub_assign(1);
        }
    }

    let value_sums = hashmap.values()
        .into_iter()
        .sum();

    return value_sums;
}

#[cfg(test)]
mod tests {
    use crate::day18::{puzzle1, puzzle2};
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 13] = [
        "2,2,2",
        "1,2,2",
        "3,2,2",
        "2,1,2",
        "2,3,2",
        "2,2,1",
        "2,2,3",
        "2,2,4",
        "2,2,6",
        "1,2,5",
        "3,2,5",
        "2,1,5",
        "2,3,5"];

    #[test]
    fn test_puzzle0() {
        let data: [&str; 2] = [
            "1,1,1",
            "2,1,1"];

        let return_value = puzzle1(&data);
        assert_eq!(return_value, 10);
    }

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, 64);
    }

    #[test]
    #[ignore]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day18.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle1(&data);
        assert_eq!(return_value, 4370);
    }


    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA);
        assert_eq!(return_value, 58);
    }

    #[test]
    #[ignore]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day18.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data);
        assert_eq!(return_value, 23641658401);
    }
}