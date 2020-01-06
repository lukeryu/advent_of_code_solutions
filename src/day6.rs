use std::collections::HashMap;

struct Orbit {
    orbited: String,
    orbiting: String
}

impl Orbit {
    fn new(orbited: String, orbiting: String) -> Orbit {
        return Orbit {
            orbited: orbited,
            orbiting: orbiting
        };
    }
}

struct CelestialBody {
    name: String,
    parents: Vec<CelestialBody>,
    children: Vec<CelestialBody>
}

impl CelestialBody {
    fn addChild(mut self, mut childBody: CelestialBody) {
        self.children.push(childBody);
        childBody.parents.push(self);
    }

    fn new(name: String) -> CelestialBody {
        return CelestialBody {
            name: name,
            parents: Vec::new(),
            children: Vec::new()
        }
    }
}

fn getOrCreate(mut celestialBodyMap: HashMap<String, CelestialBody>, bodyName: String) -> CelestialBody {
    let body = CelestialBody::new(bodyName);
    return celestialBodyMap.entry(bodyName)
        .get_or_insert(body);
}

fn puzzle1(input: Vec<String>) -> i32 {
//    let orbits = input.iter()
//        .map(|s| -> Orbit {
//            let split = s.split("(").collect();
//            return Orbit::new(
//                orbited: split[0],
//                orbiting: split[1]
//            );
//        }).collect();
//
//    let mut celestialBodyMap: HashMap<String, CelestialBody> = HashMap::new();

//    for orbit in orbits.iter() {
//        let mut orbitedBody = getOrCreate(celestialBodyMap, orbit.orbited);
//        let mut orbitingBody = getOrCreate(celestialBodyMap, orbit.orbiting);
//        orbitedBody.addChild(orbitingBody);
//    }

    return 0;
}

fn puzzle2(input: Vec<String>) -> i32 {}

#[cfg(test)]
mod tests {
    use crate::utils;
    use crate::day6::{puzzle1, puzzle2};
    struct Puzzle1Test {
        test_data: Vec<String>,
        expected_result: i32,
    }

    #[test]
    fn test_puzzle_1() {
        let mut tests: Vec<Puzzle1Test> = Vec::new();
        tests.push(Puzzle1Test {
            test_data: vec![String::from("COM)B"),
                            String::from("B)C"),
                            String::from("C)D"),
                            String::from("D)E"),
                            String::from("E)F"),
                            String::from("B)G"),
                            String::from("G)H"),
                            String::from("D)I"),
                            String::from("E)J"),
                            String::from("J)K"),
                            String::from("K)L")],
            expected_result: 42,
        });
        match utils::read_lines("data/Day6.txt") {
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
        tests.push(Puzzle2Test {
            test_data: vec![String::from("1969")],
            expected_result: 966,
        });
        tests.push(Puzzle2Test {
            test_data: vec![String::from("100756")],
            expected_result: 50346,
        });
        match utils::read_lines("data/Day1.txt") {
            Ok(lines) => {
                tests.push(Puzzle2Test {
                    test_data: lines,
                    expected_result: 4687331,
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