extern crate regex;

use std::fmt::{Display, Error, Formatter};

use self::regex::Regex;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32, z: i32) -> Point {
        return Point {
            x,
            y,
            z,
        };
    }

    pub fn new2D(x: i32, y: i32) -> Point {
        return Point::new(x, y, 0);
    }

    pub fn from_string(string: &String) -> Point {
        return Point::from_str(string.as_str());
    }

    pub fn from_str(string: &str) -> Point {
        let point_format: Regex = Regex::new(r"<x=(?P<xdim>-?\d+), y=(?P<ydim>-?\d+), z=(?P<zdim>-?\d+)>").unwrap();

        return point_format.captures(string).map(|captures| -> Point {
            let x = captures.name("xdim").unwrap().as_str().parse::<i32>().unwrap();
            let y = captures.name("ydim").unwrap().as_str().parse::<i32>().unwrap();
            let z = captures.name("zdim").unwrap().as_str().parse::<i32>().unwrap();

            return Point {
                x,
                y,
                z,
            };
        }).unwrap();
    }

    pub fn move_velocity(self, velocity: &Velocity) -> Point {
        return Point {
            x: self.x + velocity.x,
            y: self.y + velocity.y,
            z: self.z + velocity.z,
        };
    }

    pub fn distance_from_origin(&self) -> i32 {
        return self.x.abs() + self.y.abs() + self.z.abs();
    }
    pub fn shift_up(&self, d: i32) -> Point {
        return Point {
            x: self.x,
            y: self.y + d,
            z: self.z,
        };
    }
    pub fn shift_down(&self, d: i32) -> Point {
        return Point {
            x: self.x,
            y: self.y - d,
            z: self.z,
        };
    }
    pub fn shift_left(&self, d: i32) -> Point {
        return Point {
            x: self.x - d,
            y: self.y,
            z: self.z,
        };
    }
    pub fn shift_right(&self, d: i32) -> Point {
        return Point {
            x: self.x + d,
            y: self.y,
            z: self.z,
        };
    }
}


impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

pub const ORIGIN: Point = Point::new(0, 0, 0);
pub const ZERO_VELOCITY: Velocity = Velocity::new(0, 0, 0);

#[derive(Hash, Eq, PartialEq,Debug, Copy, Clone)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Velocity {
    pub const fn new(x: i32, y: i32, z: i32) -> Velocity {
        return Velocity {
            x,
            y,
            z,
        };
    }

    pub fn add(self, other_velocity: Velocity) -> Velocity {
        return Velocity::new(self.x + other_velocity.x, self.y + other_velocity.y, self.z + other_velocity.z);
    }

    pub fn distance_from_origin(&self) -> i32 {
        return self.x.abs() + self.y.abs() + self.z.abs();
    }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::*;

    #[test]
    fn test_from_str() {
        let point = Point::from_str("<x=-6, y=-8, z=-4>");
        assert_eq!(point.x, -6);
        assert_eq!(point.y, -8);
        assert_eq!(point.z, -4);
    }

    #[test]
    fn test_move_velocity() {
        let point = Point::new(1, 2, 3);
        let velocity = Velocity::new(4, 5, 6);
        let moved_point = point.move_velocity(&velocity);
        assert_eq!(moved_point.x, 5);
        assert_eq!(moved_point.y, 7);
        assert_eq!(moved_point.z, 9);
    }
}