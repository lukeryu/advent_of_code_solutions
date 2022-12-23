extern crate regex;

use std::fmt::{Display, Error, Formatter};

use self::regex::Regex;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Dimension {
    X,
    Y,
    Z,
}

///
///Represents a point in 3 dimensional space
///
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    ///
    /// Creates a new point
    ///
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        return Self {
            x,
            y,
            z,
        };
    }

    pub fn new_2d(x: i32, y: i32) -> Self {
        return Self::new(x, y, 0);
    }

    pub fn from_string(string: &String) -> Result<Self, ()> {
        return Self::from_str(string.as_str());
    }

    pub fn move_velocity(self, velocity: &Velocity) -> Self {
        return Self {
            x: self.x + velocity.x,
            y: self.y + velocity.y,
            z: self.z + velocity.z,
        };
    }

    pub fn distance_from_origin(&self) -> i32 {
        return self.x.abs() + self.y.abs() + self.z.abs();
    }
    pub fn shift_up(&self, d: i32) -> Self {
        return Self {
            x: self.x,
            y: self.y + d,
            z: self.z,
        };
    }
    pub fn shift_down(&self, d: i32) -> Self {
        return Self {
            x: self.x,
            y: self.y - d,
            z: self.z,
        };
    }
    pub fn shift_left(&self, d: i32) -> Self {
        return Self {
            x: self.x - d,
            y: self.y,
            z: self.z,
        };
    }
    pub fn shift_right(&self, d: i32) -> Self {
        return Self {
            x: self.x + d,
            y: self.y,
            z: self.z,
        };
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let point_format: Regex = Regex::new(r"<x=(?P<xdim>-?\d+), y=(?P<ydim>-?\d+), z=(?P<zdim>-?\d+)>").unwrap();

        return Ok(point_format.captures(string).map(|captures| -> Self {
            let x = captures.name("xdim").unwrap().as_str().parse::<i32>().unwrap();
            let y = captures.name("ydim").unwrap().as_str().parse::<i32>().unwrap();
            let z = captures.name("zdim").unwrap().as_str().parse::<i32>().unwrap();

            return Self {
                x,
                y,
                z,
            };
        }).unwrap());
    }
}


impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

pub const ORIGIN: Point = Point::new(0, 0, 0);
pub const ZERO_VELOCITY: Velocity = Velocity::new(0, 0, 0);

///
///Represents a velocity in 3 dimensional space
///
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Velocity {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        return Self {
            x,
            y,
            z,
        };
    }

    pub fn add(self, other_velocity: Self) -> Self {
        return Self::new(self.x + other_velocity.x, self.y + other_velocity.y, self.z + other_velocity.z);
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
        let point = Point::from_str("<x=-6, y=-8, z=-4>").unwrap();
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