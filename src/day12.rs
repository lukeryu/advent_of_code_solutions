use crate::cartesian::*;
use crate::math::*;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct DimensionHolder {
    position: i32,
    velocity: i32,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Moon {
    location: Point,
    velocity: Velocity,
}

impl Moon {
    pub fn calculate_gravity_from(self, other_moon: &Self) -> Velocity {
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;

        if (self.location.x < other_moon.location.x) {
            x = 1;
        } else if (self.location.x > other_moon.location.x) {
            x = -1;
        }

        if (self.location.y < other_moon.location.y) {
            y = 1;
        } else if (self.location.y > other_moon.location.y) {
            y = -1;
        }

        if (self.location.z < other_moon.location.z) {
            z = 1;
        } else if (self.location.z > other_moon.location.z) {
            z = -1;
        }

        return Velocity::new(x, y, z);
    }

    fn new(location: Point, velocity: Velocity) -> Self {
        return Self { location, velocity };
    }

    fn get_holder_for_dimension(self, dimension: Dimension) -> DimensionHolder {
        if (dimension == Dimension::X) {
            return DimensionHolder {
                position: self.location.x,
                velocity: self.velocity.x,
            };
        }
        if (dimension == Dimension::Y) {
            return DimensionHolder {
                position: self.location.y,
                velocity: self.velocity.y,
            };
        }
        return DimensionHolder {
            position: self.location.z,
            velocity: self.velocity.z,
        };
    }

    fn get_velocity_for_dimension(self, dimension: Dimension) -> i32 {
        if (dimension == Dimension::X) {
            return self.velocity.x;
        }
        if (dimension == Dimension::Y) {
            return self.velocity.y;
        }
        return self.velocity.z;
    }
}

fn calculate_moon_energy(moon: &Moon) -> i32 {
    return moon.location.distance_from_origin() * moon.velocity.distance_from_origin();
}

fn parse_moons(strings: Vec<String>) -> Vec<Moon> {
    return strings.iter()
        .map(&Point::from_string)
        .map(|point| -> Moon {
            return Moon {
                location: point,
                velocity: ZERO_VELOCITY,
            };
        }).collect();
}

fn update_moons(moons: &mut Vec<Moon>) {
    for moon_index in 0..moons.len() {
        let moon = moons.get(moon_index).unwrap();
        let mut new_velocity = moon.velocity;
        for other_moon_index in 0..moons.len() {
            if (other_moon_index != moon_index) {
                let other_moon = moons.get(other_moon_index).unwrap();
                let velocity = moon.calculate_gravity_from(&other_moon);
                new_velocity = new_velocity.add(velocity);
            }
        }

        moons.get_mut(moon_index).unwrap().velocity = new_velocity;
    }

    for moon in moons.iter_mut() {
        moon.location = moon.location.move_velocity(&moon.velocity);
    }
}

fn puzzle1(strings: Vec<String>, iterations: usize) -> i32 {
    let mut moons = parse_moons(strings);

    for index in 0..iterations {
        update_moons(&mut moons);
    }

    return moons.iter()
        .map(calculate_moon_energy)
        .sum();
}

fn get_current_dimension(moons: &Vec<Moon>, dimension: &Dimension) -> Vec<DimensionHolder> {
    return moons.iter()
        .map(|moon| -> DimensionHolder {
            return moon.get_holder_for_dimension(dimension.clone());
        })
        .collect();
}

fn get_velocities_for_dimension(moons: &Vec<Moon>, dimension: &Dimension) -> Vec<i32> {
    return moons.iter()
        .map(|moon| -> i32 {
            return moon.get_velocity_for_dimension(dimension.clone());
        })
        .collect();
}

fn get_repeat_count(mut moons: Vec<Moon>, dimension: Dimension) -> u64 {
    let mut moon_dimension_position = Vec::<Vec<DimensionHolder>>::new();
    moon_dimension_position.push(get_current_dimension(&moons, &dimension));

    loop {
        update_moons(&mut moons);

        let dimensions = get_current_dimension(&moons, &dimension);
        if (moon_dimension_position.contains(&dimensions)) {
            return (moon_dimension_position.len() * 2) as u64;
        }

        moon_dimension_position.push(dimensions);
    }
}

fn get_repeat_count2(mut moons: Vec<Moon>, dimension: Dimension) -> u64 {
    let final_velocities = get_current_dimension(&moons, &dimension);
    let mut steps = 0u64;
    loop {
        update_moons(&mut moons);

        steps += 1;

        let velocities = get_current_dimension(&moons, &dimension);
        if velocities == final_velocities {
            return steps;
        }

    }
}

fn puzzle2(strings: Vec<String>) -> u64 {
    let mut moons = parse_moons(strings);

    let x_count = get_repeat_count2(moons.clone(), Dimension::X);
    let y_count = get_repeat_count2(moons.clone(), Dimension::Y);
    let z_count = get_repeat_count2(moons.clone(), Dimension::Z);

    //x=36, y=56, z=88
    println!("x={}, y={}, z={}", x_count, y_count, z_count);

    return lcm3_unsigned(x_count, y_count, z_count);
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{ZERO_VELOCITY, Point};
    use crate::day12::*;
    use crate::utils;

    #[test]
    fn moon_calculate_gravity_from() {
        let ganimete = Moon::new(
            Point::new(3, 3, 3),
            ZERO_VELOCITY,
        );

        let callisto = Moon::new(
            Point::new(5, 5, 5),
            ZERO_VELOCITY,
        );

        let asdf1 = ganimete.calculate_gravity_from(&callisto);
        assert_eq!(asdf1.x, 1);
        assert_eq!(asdf1.y, 1);
        assert_eq!(asdf1.z, 1);

        let asdf2 = callisto.calculate_gravity_from(&ganimete);
        assert_eq!(asdf2.x, -1);
        assert_eq!(asdf2.y, -1);
        assert_eq!(asdf2.z, -1);

        let asdf3 = ganimete.calculate_gravity_from(&ganimete);
        assert_eq!(asdf3.x, 0);
        assert_eq!(asdf3.y, 0);
        assert_eq!(asdf3.z, 0);
    }

    struct Test1 {
        input: Vec<String>,
        iterations: usize,
        expected: i32,
    }

    #[test]
    fn test_puzzle1() {
        let mut tests = Vec::new();
        tests.push(Test1 {
            input: vec![String::from("<x=-1, y=0, z=2>"),
                        String::from("<x=2, y=-10, z=-7>"),
                        String::from("<x=4, y=-8, z=8>"),
                        String::from("<x=3, y=5, z=-1>")],
            iterations: 10,
            expected: 179,
        });

        tests.push(Test1 {
            input: vec![String::from("<x=-8, y=-10, z=0>"),
                        String::from("<x=5, y=5, z=10>"),
                        String::from("<x=2, y=-7, z=3>"),
                        String::from("<x=9, y=-8, z=-3>")],
            iterations: 100,
            expected: 1940,
        });
        let input = utils::read_lines("data/Day12.txt").unwrap();

        tests.push(Test1 {
            input: input,
            iterations: 1000,
            expected: 8742,
        });
        for test in tests {
            let result = puzzle1(test.input, test.iterations);
            assert_eq!(result, test.expected)
        }
    }

    struct Test2 {
        input: Vec<String>,
        expected: u64,
    }

    #[test]
    fn test_puzzle2() {
        let mut tests = Vec::<Test2>::new();
        tests.push(Test2 {
            input: vec![String::from("<x=-1, y=0, z=2>"),
                        String::from("<x=2, y=-10, z=-7>"),
                        String::from("<x=4, y=-8, z=8>"),
                        String::from("<x=3, y=5, z=-1>")],
            expected: 2772,
        });

        tests.push(Test2 {
            input: vec![String::from("<x=-8, y=-10, z=0>"),
                        String::from("<x=5, y=5, z=10>"),
                        String::from("<x=2, y=-7, z=3>"),
                        String::from("<x=9, y=-8, z=-3>")],
            expected: 4686774924
        });

        let input = utils::read_lines("data/Day12.txt").unwrap();

        tests.push(Test2 {
            input,
            expected: 325433763467176,
        });

        for test in tests {
            let result = puzzle2(test.input);
            assert_eq!(result, test.expected)
        }
    }
}