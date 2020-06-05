use std::collections::HashMap;

struct Orbit {
    orbited: String,
    orbiting: String,
}

impl Orbit {
    fn new(orbited: String, orbiting: String) -> Orbit {
        return Orbit {
            orbited,
            orbiting,
        };
    }
}

struct CelestialBody {
    name: String,
    parents: Vec<String>,
    children: Vec<String>,
}

impl CelestialBody {
    fn addChild(&mut self, childBody: String) {
        self.children.push(childBody);
    }

    fn addParent(&mut self, parentBody: String) {
        self.parents.push(parentBody);
    }

    fn new(name: String) -> CelestialBody {
        return CelestialBody {
            name,
            parents: Vec::new(),
            children: Vec::new(),
        };
    }

    fn orbit_count(&self, map: &HashMap<String, CelestialBody>) -> u32 {
        if self.parents.is_empty() {
            return 0;
        }

        let x: u32 = self.parents.iter()
            .map(|name| -> u32 {
                map.get(name).map_or(0, |body| body.orbit_count(map))
            })
            .sum();
        return x + 1u32;
    }

    fn parents_and_children(&self) -> Vec<String> {
        let mut vector = Vec::new();
        vector.extend(self.parents.clone());
        vector.extend(self.children.clone());

        return vector;
    }

    fn find_santa(&self, map: &HashMap<String, CelestialBody>, visited: Vec<String>) -> u32 {
        if visited.contains(&self.name) {
            return 0;
        }
        if (self.name == String::from("SAN")){
            return (visited.len() - 2) as u32;
        }
        for associate_name in self.parents_and_children().iter() {

            let mut asdf: Vec<String> = Vec::new();
            asdf.extend(visited.clone());
            asdf.push(self.name.clone());

            let result = match map.get(associate_name) {
                Some(body) => body.find_santa(map, asdf),
                None => 0
            };

            if (result > 0) {
                return result;
            }
        }
        return 0;
    }
}

fn make_orbit_graph(input: Vec<String>) -> HashMap<String, CelestialBody> {
    let orbits: Vec<Orbit> = input.iter()
        .map(|inputString| -> Orbit {
            let split: Vec<&str> = inputString.split(")")
                .collect();
            let orbited = String::from(split[0]);
            let orbiting = String::from(split[1]);
            return Orbit::new(
                orbited,
                orbiting,
            );
        }).collect();

    let mut celestial_body_map: HashMap<String, CelestialBody> = HashMap::new();

    for orbit in orbits.iter() {
        let mut orbited_body = celestial_body_map.entry(orbit.orbited.clone())
            .or_insert(CelestialBody::new(orbit.orbited.clone()));
        orbited_body.addChild(orbit.orbiting.clone());
    }

    for orbit in orbits.iter() {
        let mut orbiting_body = celestial_body_map.entry(orbit.orbiting.clone())
            .or_insert(CelestialBody::new(orbit.orbiting.clone()));
        orbiting_body.addParent(orbit.orbited.clone());
    }

    return celestial_body_map;
}

fn puzzle1(input: Vec<String>) -> u32 {
    let celestial_body_map = make_orbit_graph(input);

    return celestial_body_map.values()
        .map(|body| body.orbit_count(&celestial_body_map))
        .sum();
}

fn puzzle2(input: Vec<String>) -> u32 {
    let celestial_body_map = make_orbit_graph(input);

    let origin_body = celestial_body_map.get("YOU");

    return match origin_body {
        Some(body) => body.find_santa(&celestial_body_map, Vec::new()),
        None => 0
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;
    use crate::day6::{puzzle1, puzzle2};

    struct Puzzle1Test {
        test_data: Vec<String>,
        expected_result: u32,
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
                    expected_result: 322508,
                });
                for test in tests {
                    println!("{}", "test");
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
        expected_result: u32,
    }

    #[test]
    fn test_puzzle_2() {
        let mut tests: Vec<Puzzle2Test> = Vec::new();
        tests.push(Puzzle2Test {
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
                                String::from("K)L"),
                                String::from("K)YOU"),
                                String::from("I)SAN")],
                expected_result: 4,
        });

        match utils::read_lines("data/Day6.txt") {
            Ok(lines) => {
                tests.push(Puzzle2Test {
                    test_data: lines,
                    expected_result: 496,
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