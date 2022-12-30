use std::cmp::Ordering;
use std::collections::{HashMap};
use std::hash::Hash;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point<T: Ord + Sub<Output=T> + Add<Output=T>> {
    pub x: T,
    pub y: T,
}

impl<T: Ord + Sub<Output=T> + Add<Output=T> + Copy + Hash> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn distance_between(self, other_point: Point<T>) -> T {
        let mut x_dis: T;
        if self.x < other_point.x {
            x_dis = other_point.x - self.x;
        } else {
            x_dis = self.x - other_point.x;
        }
        let mut y_dis: T;
        if self.y < other_point.y {
            y_dis = other_point.y - self.y;
        } else {
            y_dis = self.y - other_point.y;
        }

        return x_dis + y_dis;
    }

    pub fn up(&self, value: T) -> Self {
        Self {
            x: self.x,
            y: self.y + value,
        }
    }

    pub fn down(&self, value: T) -> Self {
        Self {
            x: self.x,
            y: self.y - value,
        }
    }

    pub fn left(&self, value: T) -> Self {
        Self {
            x: self.x - value,
            y: self.y,
        }
    }

    pub fn right(&self, value: T) -> Self {
        Self {
            x: self.x + value,
            y: self.y,
        }
    }
}

impl<T: Ord + Sub<Output=T> + Add<Output=T>> PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let order = self.x.cmp(&other.x);
        if order == Ordering::Equal {
            return Some(self.y.cmp(&other.y));
        }
        return Some(order);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point3<T: Ord + Sub<Output=T> + Add<Output=T> + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Ord + Sub<Output=T> + Add<Output=T> + Copy> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn distance_between(self, other_point: Point3<T>) -> T {
        let mut x_dis: T;
        if self.x < other_point.x {
            x_dis = other_point.x - self.x;
        } else {
            x_dis = self.x - other_point.x;
        }
        let mut y_dis: T;
        if self.y < other_point.y {
            y_dis = other_point.y - self.y;
        } else {
            y_dis = self.y - other_point.y;
        }

        let mut z_dis: T;
        if self.y < other_point.y {
            z_dis = other_point.z - self.z;
        } else {
            z_dis = self.z - other_point.z;
        }

        return x_dis + y_dis + z_dis;
    }

    pub fn up(&self, value: T) -> Self {
        Self {
            x: self.x,
            y: self.y + value,
            z: self.z,
        }
    }

    pub fn down(&self, value: T) -> Self {
        Self {
            x: self.x,
            y: self.y - value,
            z: self.z,
        }
    }

    pub fn left(&self, value: T) -> Self {
        Self {
            x: self.x - value,
            y: self.y,
            z: self.z,
        }
    }

    pub fn right(&self, value: T) -> Self {
        Self {
            x: self.x + value,
            y: self.y,
            z: self.z,
        }
    }

    pub fn forward(&self, value: T) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + value,
        }
    }

    pub fn back(&self, value: T) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z - value,
        }
    }
}

pub enum Direction {
    NORTH,
    SOUTH,
    WEST,
    EAST
}

impl<T: Ord + Sub<Output=T> + Add<Output=T> + Copy> PartialOrd for Point3<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut order = self.x.cmp(&other.x);
        if order == Ordering::Equal {
            order = self.y.cmp(&other.y);
        }
        if order == Ordering::Equal {
            order = self.z.cmp(&other.z);
        }
        return Some(order);
    }
}


pub struct Grid<T: Ord + Sub<Output=T> + Add<Output=T> + Hash, V: Eq> {
    map: HashMap<Point<T>, V>,
}

impl<T: Ord + Sub<Output=T> + Add<Output=T> + Hash, V: Eq> Grid<T, V> {
    pub fn new() -> Self {
        return Self {
            map: HashMap::new()
        };
    }

    pub fn set_value_at(&mut self, point: Point<T>, value: V) {
        self.map.insert(point, value);
    }

    pub fn matches_row_and_value(&self, row: T, value: V) -> usize {
        let mut count = 0;
        for (key, map_val) in self.map.iter() {
            if (*key).y == row {
                if *map_val == value {
                    count = count + 1;
                }
            }
        }
        return count;
    }

    pub fn len(&self) -> usize {
        return self.map.len();
    }
}