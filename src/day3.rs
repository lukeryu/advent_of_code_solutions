use std::collections::hash_set::Intersection;
use std::collections::HashSet;
use std::fmt::{Display, Error, Formatter};
use std::io;
use std::thread::current;
use std::ops::Range;
use crate::cartesian::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Plane {
    HORIZONTAL,
    VERTICAL,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Segment {
    lhs: Point,
    rhs: Point,
    plane: Plane,
    start_number_of_steps: u32,
    direction: Direction,
}

fn calculate_signal_delay(lhssegment: &Segment, rhssegment: &Segment, point: &Point) -> u32 {
    return lhssegment.signal_delay_at(point) + rhssegment.signal_delay_at(point);
}

impl Segment {
    fn intersects_at(&self, segment: &Segment) -> Vec<Point> {
        if self.plane == segment.plane {
            return Vec::new();
        }
        if self.get_range().contains(&segment.get_line()) &&
            segment.get_range().contains(&self.get_line()) {
            let intersection_point;
            if self.plane == Plane::HORIZONTAL {
                intersection_point = Point::new2D(segment.get_line(), self.get_line());
            } else {
                intersection_point = Point::new2D(self.get_line(), segment.get_line());
            }
            if (!intersection_point.eq(&ORIGIN)) {
                return vec![intersection_point];
            }
        }

        return Vec::new();
    }

    fn signal_delay_at(&self, point: &Point) -> u32 {
        if self.plane == Plane::HORIZONTAL {
            let diff = i32::abs(self.start_point().x - point.x) as u32;
            return self.start_number_of_steps + diff;
        } else {
            let diff = i32::abs(self.start_point().y - point.y) as u32;
            return self.start_number_of_steps + diff;
        }
        return 0;
    }

    fn start_point(&self) -> Point {
        if (self.direction == Direction::LEFT) {
            return self.lhs;
        }
        return self.rhs;
    }

    fn signal_delay_intersects_at(&self, segment: &Segment) -> Vec<u32> {
        let points = self.intersects_at(segment);
        let mut signal_delays = Vec::new();

        for point in points.iter() {
            signal_delays.push(calculate_signal_delay(self, &segment, point));
        }

        return signal_delays;
    }

    fn get_range(&self) -> Range<i32> {
        if self.plane == Plane::HORIZONTAL {
            return Range {
                start: self.lhs.x,
                end: self.rhs.x,
            };
        } else {
            return Range {
                start: self.lhs.y,
                end: self.rhs.y,
            };
        }
    }

    fn get_line(&self) -> i32 {
        if self.plane == Plane::HORIZONTAL {
            return self.lhs.y;
        } else {
            return self.lhs.x;
        }
    }

    fn len(&self) -> u32 {
        return i32::abs(self.lhs.x - self.rhs.x) as u32 + i32::abs(self.lhs.y - self.rhs.y) as u32;
    }

    fn new(left_point: Point, right_point: Point, start_number_of_steps: u32) -> Segment {
        if left_point.x == right_point.x {
            if left_point.y < right_point.y {
                return Segment {
                    lhs: left_point,
                    rhs: right_point,
                    plane: Plane::VERTICAL,
                    start_number_of_steps: start_number_of_steps,
                    direction: Direction::LEFT,
                };
            } else {
                return Segment {
                    lhs: right_point,
                    rhs: left_point,
                    plane: Plane::VERTICAL,
                    start_number_of_steps: start_number_of_steps,
                    direction: Direction::RIGHT,
                };
            }
        } else {
            if left_point.x < right_point.x {
                return Segment {
                    lhs: left_point,
                    rhs: right_point,
                    plane: Plane::HORIZONTAL,
                    start_number_of_steps: start_number_of_steps,
                    direction: Direction::LEFT,
                };
            } else {
                return Segment {
                    lhs: right_point,
                    rhs: left_point,
                    plane: Plane::HORIZONTAL,
                    start_number_of_steps: start_number_of_steps,
                    direction: Direction::RIGHT,
                };
            }
        }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{},{}", self.lhs, self.rhs)
    }
}

fn get_direction(s: &String) -> Direction {
    let first_character_option: Option<char> = s.chars().next();
    match first_character_option {
        None => return Direction::UP,
        Some(first_character) => {
            match first_character {
                'U' => return Direction::UP,
                'D' => return Direction::DOWN,
                'L' => return Direction::LEFT,
                'R' => return Direction::RIGHT,
                _ => return Direction::UP
            }
        }
    }
}

fn get_steps(s: &String) -> i32 {
    let number_of_steps = &s[1..];
    return number_of_steps.parse::<i32>().unwrap();
}

fn get_segments(directions_vector: &Vec<String>) -> Vec<Segment> {
    let mut segments: Vec<Segment> = Vec::new();

    let mut current_point = ORIGIN;
    let mut start_number_of_steps = 0;

    for direction_string in directions_vector.iter() {
        println!("start_number_of_steps {}", start_number_of_steps);
        let dir = get_direction(&direction_string);
        let steps = get_steps(&direction_string);
        let next_point = match &dir {
            Direction::UP => current_point.shift_up(steps),
            Direction::DOWN => current_point.shift_down(steps),
            Direction::LEFT => current_point.shift_left(steps),
            Direction::RIGHT => current_point.shift_right(steps),
            _ => current_point.shift_up(steps),
        };

        let segment = Segment::new(current_point.clone(), next_point.clone(), start_number_of_steps);
        segments.push(segment);
        start_number_of_steps += segment.len();
        current_point = next_point;
    }

    return segments;
}

fn parse_string(string: &String) -> Vec<String> {
    return string.split(",").map(String::from).collect();
}


fn parse_index(input: &Vec<String>, index: usize) -> Vec<String> {
    return input.get(index)
        .map_or(vec![], parse_string);
}

fn puzzle_1(input: Vec<String>) -> i32 {
    let left_string: Vec<String> = parse_index(&input, 0);
    let left_segment_set = get_segments(&left_string);
    let right_string: Vec<String> = parse_index(&input, 1);
    let right_segment_set = get_segments(&right_string);

    let mut closest_point = Option::None;
    for lhs_segment in left_segment_set.iter() {
        for rhs_segment in right_segment_set.iter() {
            let points = lhs_segment.intersects_at(rhs_segment);

            for point in points.iter() {
                println!("rhs {}", point);
                match closest_point {
                    None => closest_point = Option::Some(point.clone()),
                    Some(intersection_point) => {
                        if point.distance_from_origin() < intersection_point.distance_from_origin() {
                            closest_point = Option::Some(point.clone())
                        }
                    }
                }
            }
        }
    }
    return closest_point.map_or(0, |point| point.distance_from_origin());
}

fn puzzle_2(input: Vec<String>) -> u32 {
    let left_string: Vec<String> = parse_index(&input, 0);
    let left_segment_set = get_segments(&left_string);
    let right_string: Vec<String> = parse_index(&input, 1);
    let right_segment_set = get_segments(&right_string);

    let mut closest_point = Option::None;
    for lhs_segment in left_segment_set.iter() {
        for rhs_segment in right_segment_set.iter() {
            let points = lhs_segment.signal_delay_intersects_at(rhs_segment);

            for point in points.iter() {
                println!("rhs {}", point);
                match closest_point {
                    None => closest_point = Option::Some(point.clone()),
                    Some(intersection_point) => {
                        if point < &intersection_point.clone() {
                            closest_point = Option::Some(point.clone())
                        }
                    }
                }
            }
        }
    }


    return closest_point.unwrap_or(0);
}

#[cfg(test)]
mod tests {
    use crate::day3::{puzzle_1, puzzle_2};
    use crate::utils;

    struct Puzzle1Test {
        test_data: Vec<String>,
        expected_result: i32,
    }

    #[test]
    fn test_puzzle_1() {
        let mut tests: Vec<Puzzle1Test> = Vec::new();
        tests.push(Puzzle1Test {
            test_data: vec![String::from("R8,U5,L5,D3"), String::from("U7,R6,D4,L4")],
            expected_result: 6,
        });
        tests.push(Puzzle1Test {
            test_data: vec![String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"), String::from("U62,R66,U55,R34,D71,R55,D58,R83")],
            expected_result: 159,
        });
        tests.push(Puzzle1Test {
            test_data: vec![String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"), String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")],
            expected_result: 135,
        });
        match utils::read_lines("data/Day3.txt") {
            Ok(lines) => {
                tests.push(Puzzle1Test {
                    test_data: lines,
                    expected_result: 865,
                });
                for test in tests {
                    let result = puzzle_1(test.test_data);
                    assert_eq!(result, test.expected_result);
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }

    struct Puzzle2Test {
        test_data: Vec<String>,
        expected_result: u32,
    }

    #[test]
    fn test_puzzle_2() {
        let mut tests: Vec<Puzzle2Test> = Vec::new();
        tests.push(Puzzle2Test {
            test_data: vec![String::from("R8,U5,L5,D3"), String::from("U7,R6,D4,L4")],
            expected_result: 30,
        });
        tests.push(Puzzle2Test {
            test_data: vec![String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"), String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")],
            expected_result: 410,
        });
        tests.push(Puzzle2Test {
            test_data: vec![String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"), String::from("U62,R66,U55,R34,D71,R55,D58,R83")],
            expected_result: 610,
        });
        match utils::read_lines("data/Day3.txt") {
            Ok(lines) => {
                tests.push(Puzzle2Test {
                    test_data: lines,
                    expected_result: 35038,
                });
                for test in tests {
                    let result = puzzle_2(test.test_data);
                    assert_eq!(result, test.expected_result);
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}