use std::ops::{Add, Mul, Rem};

pub fn multiply_by<T: Mul>(value: T) -> &'static dyn Fn(&T) -> T {
    return &(|input: &T| -> T {
        return input.clone() * value;
    });
}

pub fn add_to<T: Add>(value: T) -> &'static dyn Fn(&T) -> T {
    return &(|input: &T| -> T {
        return input.clone() + value;
    });
}

pub fn square<T: Mul>() -> &'static dyn Fn(&T) -> T {
    return &(|input: &T| -> T {
        return input.clone() * input.clone();
    });
}

pub fn test_value<T: Rem, R>(test: T, if_true: R, if_false: R) -> &'static dyn Fn(&T) -> R {
    return &(|item_priority| -> T {
        if item_priority.clone() % test == 0 {
            return if_true;
        } else {
            return if_false;
        }
    });
}