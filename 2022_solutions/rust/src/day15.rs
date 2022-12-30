use lazy_static::lazy_static;
use regex::Regex;
use crate::cartesian::{Grid, Point};

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
}

struct Sensor {
    beacon: Point<i64>,
    sensor: Point<i64>,
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


        let sensor = Sensor {
            sensor: sensor_point,
            beacon: beacon_point,
        };

        vec.push(sensor);
    }

    return vec;
}

fn puzzle1(input_array: &[&str], at_row: i64) -> usize {
    let parsed_sensors = parse_sensors(input_array);

    println!("sensors: {}", parsed_sensors.len());

    let mut grid = Grid::<i64, bool>::new();

    for sensor in parsed_sensors {
        let distance = sensor.sensor.distance_between(sensor.beacon);

        for x_delta in 0..distance {
            for y_delta in 0..(distance - x_delta) {
                let point1 = Point::new(sensor.sensor.x + x_delta, sensor.sensor.y + y_delta);
                let point2 = Point::new(sensor.sensor.x + x_delta, sensor.sensor.y - y_delta);
                let point3 = Point::new(sensor.sensor.x - x_delta, sensor.sensor.y + y_delta);
                let point4 = Point::new(sensor.sensor.x - x_delta, sensor.sensor.y - y_delta);

                grid.set_value_at(point1, true);
                grid.set_value_at(point2, true);
                grid.set_value_at(point3, true);
                grid.set_value_at(point4, true);
            }
        }
    }

    println!("len {}", grid.len());

    return grid.matches_row_and_value(at_row, true);
}

fn puzzle2(input_array: &[&str]) -> usize {
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
    #[ignore]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA, 10);
        assert_eq!(return_value, 13);
    }

    #[test]
    #[ignore]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day13.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle1(&data, 2000000);
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
        let vec = read_file_strings("../data/Day13.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data);
        assert_eq!(return_value, 23641658401);
    }
}