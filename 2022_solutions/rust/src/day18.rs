use std::collections::HashMap;
use std::ops::{Sub, SubAssign};
use std::ptr::hash;
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
            },
            _ => {}
        }

        match hashmap.get_mut(&point3.down(1)) {
            Some(value) => {
                value.sub_assign(1);
                count = count - 1;
            },
            _ => {}
        }

        match hashmap.get_mut(&point3.left(1)) {
            Some(value) => {
                value.sub_assign(1);
                count = count - 1;
            },
            _ => {}
        }

        match hashmap.get_mut(&point3.right(1)) {
            Some(value) => {
                value.sub_assign(1);
                count = count - 1;
            },
            _ => {}
        }

        match hashmap.get_mut(&point3.forward(1)) {
            Some(value) => {
                value.sub_assign(1);
                count = count - 1;
            },
            _ => {}
        }

        match hashmap.get_mut(&point3.back(1)) {
            Some(value) => {
                value.sub_assign(1);
                count = count - 1;
            },
            _ => {}
        }

        hashmap.insert(point3, count);
    }

    for (key, value) in &hashmap {
        println!("key=[x={}, y={}, z={}] value={}", key.x, key.y, key.z, value);
    }

    let value_sums = hashmap.values()
        .into_iter()
        .sum();

    return value_sums;
}

fn puzzle2(input_array: &[&str]) -> usize {
    return 0;
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
    #[ignore]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, 4370);
    }

    #[test]
    #[ignore]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day18.txt");
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
        let vec = read_file_strings("../data/Day18.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data);
        assert_eq!(return_value, 23641658401);
    }
}