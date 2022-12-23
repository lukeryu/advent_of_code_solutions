use num::{Num, Signed};

pub fn lcm_unsigned<T: Num + Clone + PartialOrd>(lhs: T, rhs: T) -> T {
    return lhs.clone() * rhs.clone() / gcd(lhs, rhs);
}

pub fn lcm3_unsigned<T: Num + Clone + PartialOrd>(a: T, b: T, c: T) -> T {
    return lcm_unsigned(a, lcm_unsigned(b, c));
}

pub fn lcm<T: Num + Clone + PartialOrd + Signed>(lhs: T, rhs: T) -> T {
    return num::abs(lhs.clone() * rhs.clone()) / gcd(lhs, rhs);
}

pub fn lcm3<T: Num + Clone + PartialOrd + Signed>(a: T, b: T, c: T) -> T {
    return lcm(a, lcm(b, c));
}

pub fn is_even<T: Num>(num: T) -> bool {
    let two = T::one().add(T::one());
    return num.rem(two).is_zero();
}

pub fn is_odd<T: Num>(num: T) -> bool {
    return !is_even(num);
}

pub fn gcd<T: Num + Clone + PartialOrd>(u: T, v: T) -> T {
    // simple cases (termination)
    if u == v {
        return u;
    }

    if u.is_zero() {
        return v;
    }

    if v.is_zero() {
        return u;
    }
    let two = T::one().add(T::one());

    // look for factors of 2
    if is_even(u.clone()) { // u is even
        if is_odd(v.clone()) {
            return gcd(u.clone() / two.clone(), v.clone());
        } else {
            return gcd(u.clone() / two.clone(), v.clone() / two.clone()) * two.clone();
        }
    }

    if is_even(v.clone()) { // u is odd, v is even
        return gcd(u.clone(), v.clone() / two.clone());
    }

    // reduce larger argument
    if u > v {
        return gcd((u.clone() - v.clone()) / two.clone(), v.clone());
    }
    return gcd((v.clone() - u.clone()) / two.clone(), u.clone());
}

#[cfg(test)]
mod tests {
    use crate::math::{gcd, is_even, is_odd, lcm, lcm_unsigned, lcm3_unsigned};

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12)
    }

    #[test]
    fn test_lcm_unsigned() {
        assert_eq!(lcm_unsigned(4, 6), 12);
        assert_eq!(lcm_unsigned(18, 28), 252);
    }

    #[test]
    fn test_lcm3_unsigned() {
        assert_eq!(lcm3_unsigned(18, 28, 44), 2772)
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(18, 28), 2);
        assert_eq!(gcd(252, 44), 4);
    }

    #[test]
    fn test_is_odd() {
        assert!(!is_odd(0));
        assert!(is_odd(1));
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(0));
        assert!(!is_even(1));
    }

}