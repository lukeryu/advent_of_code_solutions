use std::collections::hash_set::Intersection;
use std::collections::HashSet;
use std::thread::current;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self) -> i32 {
        return self.x.abs() + self.y.abs();
    }
    fn up(&self) -> Point {
        return Point {
            x: self.x,
            y: self.y + 1,
        };
    }
    fn down(&self) -> Point {
        return Point {
            x: self.x,
            y: self.y - 1,
        };
    }
    fn left(&self) -> Point {
        return Point {
            x: self.x - 1,
            y: self.y,
        };
    }
    fn right(&self) -> Point {
        return Point {
            x: self.x + 1,
            y: self.y,
        };
    }
}

fn getDirection(s: &String) -> Direction {
    let firstCharacterOption: Option<char> = s.chars().next();
    match firstCharacterOption {
        None => return Direction::UP,
        Some(firstCharacter) => {
            match firstCharacter {
                'U' => return Direction::UP,
                'D' => return Direction::DOWN,
                'L' => return Direction::LEFT,
                'R' => return Direction::RIGHT,
                _ => return Direction::UP
            }
        }
    }
}

fn getSteps(s: &String) -> i32 {
    let number_of_steps = &s[1..];
    return number_of_steps.parse::<i32>().unwrap();
}

fn getPointSet(stringVec: &Vec<String>) -> HashSet<Point> {
    let mut set: HashSet<Point> = HashSet::new();

    let mut current_point = Point { x: 0, y: 0 };

    for currentThing in stringVec {
        let dir: Direction = getDirection(&currentThing);
        let mut steps = getSteps(&currentThing);
        for i in 1..steps {
            let next_point = match &dir {
                UP => current_point.up(),
                DOWN => current_point.down(),
                LEFT => current_point.left(),
                RIGHT => current_point.right(),
                _ => current_point.up()
            };

            set.insert(next_point.clone());
            current_point = next_point;
        }
    }


    return set;
}

fn parseString(string: &String) -> Vec<String> {
    return string.split(",").map(String::from).collect();
}


fn parseIndex(input: &Vec<String>, index: usize) -> Vec<String> {
    return input.get(index)
        .map_or(vec![], parseString);
}

fn puzzle1(input: Vec<String>) -> i32 {
    let leftString: Vec<String> = parseIndex(&input, 0);
    let leftPointSet = getPointSet(&leftString);
    let rightString: Vec<String> = parseIndex(&input, 1);
    let rightPointSet = getPointSet(&rightString);

    println!("left String {}", &leftString.join(","));
    println!("right String {}", &rightString.join(","));
    println!("left size {}", &leftPointSet.len());
    println!("right size {}", &rightPointSet.len());

    let intersection = leftPointSet.intersection(&rightPointSet);

    let mut closestPoint = Option::None;

    for intersect in intersection {
        match closestPoint {
            None => closestPoint = Option::Some(intersect),
            Some(point) => if intersect.distance() < point.distance() {
                closestPoint = Option::Some(intersect)
            }
        }
    }

    return closestPoint.map_or(0, |point| point.distance());
}

fn puzzle2(input: Vec<String>) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day3::{puzzle1, puzzle2};
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
                    expected_result: 0,
                });
                for test in tests {
                    let result = puzzle1(test.test_data);
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
        expected_result: i32,
    }

    #[test]
    fn test_puzzle_2() {
        let mut tests: Vec<Puzzle2Test> = Vec::new();
        tests.push(Puzzle2Test {
            test_data: vec![String::from("14")],
            expected_result: 2,
        });
        match utils::read_lines("data/Day1.txt") {
            Ok(lines) => {
                tests.push(Puzzle2Test {
                    test_data: lines,
                    expected_result: 0,
                });
                for test in tests {
                    let result = puzzle2(test.test_data);
                    assert_eq!(result, test.expected_result);
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}