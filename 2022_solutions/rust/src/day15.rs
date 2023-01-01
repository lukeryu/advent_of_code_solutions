use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use std::ops::Range;
use lazy_static::lazy_static;
use regex::Regex;
use crate::cartesian::{Point};

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
}

struct Sensor {
    beacon: Point<i64>,
    sensor: Point<i64>,
    radius: i64,
}

impl Sensor {
    fn new(beacon: Point<i64>, sensor: Point<i64>) -> Self {
        Self {
            beacon,
            sensor,
            radius: sensor.distance_between(beacon)
        }
    }
}

fn parse_sensors(input_array: &[&str]) -> Vec<Sensor> {
    let mut vec = Vec::<Sensor>::new();

    for input_string in input_array {
        let captures = REGEX.captures(*input_string).unwrap();

        let sensor_x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let sensor_y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let beacon_x = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let beacon_y = captures.get(4).unwrap().as_str().parse::<i64>().unwrap();

        let sensor_point = Point::new(sensor_x, sensor_y);
        let beacon_point = Point::new(beacon_x, beacon_y);


        let sensor = Sensor::new(beacon_point, sensor_point);

        vec.push(sensor);
    }

    return vec;
}

fn solve(
    beacons: &[Sensor],
    target_y: i64,
    max_x: Option<i64>,
) -> (Vec<Range<i64>>, i64) {

    let mut regions: Vec<Range<i64>> = vec![];
    let mut extra: HashSet<Point<i64>> = HashSet::new();
    for beacon in beacons.iter() {
        let x = beacon.sensor.x;
        let from_target =
            beacon.radius - (beacon.sensor.y.abs_diff(target_y) as i64);
        if from_target >= 0 {
            let width = from_target;
            let mut min = x - width;
            let mut max = x + width + 1;
            if let Some(max_x) = max_x {
                min = 0.max(min.min(max_x));
                max = (max_x).min(max);
            }
            let range = (min)..(max);
            if beacon.sensor.y == target_y && range.contains(&beacon.sensor.x) {
                extra.insert(beacon.sensor);
            }
            if beacon.sensor.y == target_y && range.contains(&beacon.beacon.x) {
                extra.insert(beacon.beacon);
            }
            regions.push(range);
        }
    }
    return (regions, extra.len() as i64)
}

fn reduce(mut regions: Vec<Range<i64>>) -> VecDeque<Range<i64>> {
    let mut stack: VecDeque<Range<i64>> = VecDeque::new();
    regions.sort_by(|a, b| a.start.cmp(&b.start));
    let rng = regions.remove(0);
    stack.push_front(rng);
    for i in 0..regions.len() {
        let Some(top) = stack.pop_front() else  {break;};
        let Some(next) = regions.get(i) else {break;};

        if top.end < next.start {
            stack.push_back(top);
            stack.push_front(next.clone());
        } else if top.end < next.end {
            stack.push_front((top.start)..(next.end));
        } else {
            stack.push_front(top);
        }
    }
    return stack;
}

fn puzzle1(input_array: &[&str], at_row: i64) -> usize {
    let parsed_sensors = parse_sensors(input_array);

    let mut min_x = 0;
    let mut max_x = 0;

    for sensor in parsed_sensors.iter() {
        let xminus = sensor.sensor.x - sensor.radius;
        let xplus = sensor.sensor.x + sensor.radius;

        min_x = min(min_x, xminus);
        max_x = max(max_x, xplus);
    }

    let mut count = 0;
    for column in min_x..=max_x {
        let point = Point::new(column, at_row);
        let can_be_sensed = parsed_sensors.iter()
            .any(|sensor| sensor.sensor.distance_between(point) <= sensor.radius);

        if can_be_sensed {
            count = count + 1;
        }
    }

    return count - 1;
}

fn puzzle2(input_array: &[&str], row_size: i64) -> i64 {
    let parsed_sensors = parse_sensors(input_array);

    for y in 0..row_size {
        let (regions, _) = solve(&parsed_sensors, y, Some(row_size));
        let stack = reduce(regions);
        if stack.len() > 1 {
            return ((stack[1].end) * 4000000) + y;
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day15::{puzzle1, puzzle2};
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 14] = [
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3"];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA, 10);
        assert_eq!(return_value, 26);
    }

    #[test]
    #[ignore]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day15.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle1(&data, 2000000);
        assert_eq!(return_value, 5878678);
    }


    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA, 20);
        assert_eq!(return_value, 56000011);
    }

    #[test]
    #[ignore]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day15.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data, 4000000);
        assert_eq!(return_value, 23641658401);
    }
}