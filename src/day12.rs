use crate::cartesian::*;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Moon {
    location: Point,
    velocity: Velocity,
}

impl Moon {
    pub fn calculate_gravity_from(self, other_moon: &Moon) -> Velocity {
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

    fn new(location: Point, velocity: Velocity) -> Moon {
        return Moon { location, velocity };
    }
}

fn calculate_moon_energy(moon: &Moon) -> i32 {
    return moon.location.distance_from_origin() * moon.velocity.distance_from_origin();
}

fn puzzle1(strings: Vec<String>, iterations: usize) -> i32 {
    let mut moons: Vec<Moon> = strings.iter()
        .map(&Point::from_string)
        .map(|point| -> Moon {
            return Moon {
                location: point,
                velocity: ZERO_VELOCITY,
            };
        }).collect();

    for index in 0..iterations {
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


    return moons.iter()
        .map(calculate_moon_energy)
        .sum();
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
        expected: i32
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
            expected: 179
        });

        tests.push(Test1 {
            input: vec![String::from("<x=-8, y=-10, z=0>"),
                        String::from("<x=5, y=5, z=10>"),
                        String::from("<x=2, y=-7, z=3>"),
                        String::from("<x=9, y=-8, z=-3>")],
            iterations: 100,
            expected: 1940
        });
        let input = utils::read_lines("data/Day12.txt").unwrap();

        tests.push(Test1 {
            input: input,
            iterations: 1000,
            expected: 8742
        });
        for test in tests {
            let result = puzzle1(test.input, test.iterations);
            assert_eq!(result, test.expected)
        }

    }
}