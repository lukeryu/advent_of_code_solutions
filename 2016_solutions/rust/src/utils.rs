use std::fs;
use std::io::Read;

pub fn read_file_strings(string: &str) -> Vec<String> {
    let mut data = String::new();
    let mut file = fs::File::open(string).unwrap();
    file.read_to_string(&mut data).unwrap();
    let file_lines: Vec<String> = data.split("\n")
        .map(String::from)
        .collect();

    return file_lines;
}

pub fn is_between<T: Ord>(lower_bound: T, upper_bound: T, value: T) -> bool {
    return value >= lower_bound && value <= upper_bound;
}

pub struct Range<T: Ord> {
    lhs: T,
    rhs: T,
}

impl <T: Ord> Range<T> {
    pub fn new(lhs: T, rhs: T) -> Self {
        if lhs <= rhs {
            return Self { lhs, rhs };
        } else {
            return Self { lhs: rhs, rhs: lhs };
        }
    }

    pub fn encompasses(&self, other_range: &Range<T>) -> bool {
        return self.lhs <= other_range.lhs && self.rhs >= other_range.rhs;
    }

    pub fn overlaps(&self, other_range: &Range<T>) -> bool {
        return is_between(&self.lhs, &self.rhs, &other_range.lhs)
            || is_between(&self.lhs, &self.rhs, &other_range.rhs)
            || other_range.encompasses(&self);
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::Range;

    #[test]
    fn test_range_encompasses() {
        let range1 = Range::new(1, 10);
        let range2 = Range::new(5, 6);
        let range3 = Range::new(20, 30);

        assert!(range1.encompasses(&range2));
        assert!(!range2.encompasses(&range1));
        assert!(!range1.encompasses(&range3));
        assert!(!range2.encompasses(&range3));
    }

    #[test]
    fn test_range_overlaps() {
        let range1 = Range::new(1, 10);
        let range2 = Range::new(5, 6);
        let range3 = Range::new(20, 30);
        let range4 = Range::new(5, 15);

        assert!(range1.overlaps(&range1));
        assert!(range1.overlaps(&range2));
        assert!(!range1.overlaps(&range3));
        assert!(range1.overlaps(&range4));

        assert!(range2.overlaps(&range1));
        assert!(range2.overlaps(&range2));
        assert!(!range2.overlaps(&range3));
        assert!(range2.overlaps(&range4));

        assert!(!range3.overlaps(&range1));
        assert!(!range3.overlaps(&range2));
        assert!(range3.overlaps(&range3));
        assert!(!range3.overlaps(&range4));

        assert!(range4.overlaps(&range1));
        assert!(range4.overlaps(&range2));
        assert!(!range4.overlaps(&range3));
        assert!(range4.overlaps(&range4));
    }

}
