use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }
}

fn find_visible_count(point: &Point, size: usize, max_point: &Point, x_dir: i64, y_dir: i64, hash_map: &HashMap<Point, usize>) -> usize {
    if point.x == 0 || point.y == 0 || point.x == max_point.x || point.y == max_point.y {
        return 0;
    }

    let next_point = Point::new((point.x as i64 + x_dir) as usize, (point.y as i64 + y_dir) as usize);

    let height = hash_map.get(&next_point).unwrap();
    if *height >= size {
        return 1;
    }

    return find_visible_count(&next_point, size, max_point, x_dir, y_dir, hash_map) + 1;
}

fn count_visible (point: &Point, size: usize, max_point: &Point, hash_map: &HashMap<Point, usize>) -> usize {
    let east_visible_count = find_visible_count(point, size, max_point, 1, 0, hash_map);

    let west_visible_count = find_visible_count(point, size, max_point, -1, 0, hash_map);

    let north_visible_count = find_visible_count(point, size, max_point, 0, 1, hash_map);

    let south_visible_count = find_visible_count(point, size, max_point, 0, -1, hash_map);

    return east_visible_count * west_visible_count * north_visible_count * south_visible_count;
}

fn find_visible(point: &Point, size: usize, max_point: &Point, x_dir: i64, y_dir: i64, hash_map: &HashMap<Point, usize>) -> bool {
    if point.x == 0 || point.y == 0 || point.x == max_point.x || point.y == max_point.y {
        return true;
    }

    let next_point = Point::new((point.x as i64 + x_dir) as usize, (point.y as i64 + y_dir) as usize);

    let height = hash_map.get(&next_point).unwrap();
    if *height >= size {
        return false;
    }

    return find_visible(&next_point, size, max_point, x_dir, y_dir, hash_map);
}

fn is_visible(point: &Point, size: usize, max_point: &Point, hash_map: &HashMap<Point, usize>) -> bool {
    if point.x == 0 || point.y == 0 || point.x == max_point.x || point.y == max_point.y {
        return true;
    }

    let east_visible = find_visible(point, size, max_point, 1, 0, hash_map);
    if east_visible {
        return true;
    }

    let west_visible = find_visible(point, size, max_point, -1, 0, hash_map);
    if west_visible {
        return true;
    }
    let north_visible = find_visible(point, size, max_point, 0, 1, hash_map);
    if north_visible {
        return true;
    }
    return find_visible(point, size, max_point, 0, -1, hash_map);
}

fn puzzle1(input_array: &[&str]) -> usize {
    let mut point_map = HashMap::<Point, usize>::new();

    let max_x = input_array.get(0).unwrap().len() - 1;
    let max_y = input_array.len() - 1;
    let max_point = Point::new(max_x, max_y);

    for (row_index, row) in input_array.iter().enumerate() {
        for (column_index, value) in row.chars().enumerate() {
            let point = Point::new(column_index, row_index);
            point_map.insert(point, value.to_string().parse::<usize>().unwrap());
        }
    }

    let mut visible_count = 0;
    for (point, size) in &point_map {
        if is_visible(point, *size, &max_point, &point_map) {
            visible_count = visible_count + 1;
        }
    }

    return visible_count;
}

fn puzzle2(input_array: &[&str]) -> usize {
    let mut point_map = HashMap::<Point, usize>::new();

    let max_x = input_array.get(0).unwrap().len() - 1;
    let max_y = input_array.len() - 1;
    let max_point = Point::new(max_x, max_y);

    for (row_index, row) in input_array.iter().enumerate() {
        for (column_index, value) in row.chars().enumerate() {
            let point = Point::new(column_index, row_index);
            point_map.insert(point, value.to_string().parse::<usize>().unwrap());
        }
    }

    let mut max_visible_count = 0;
    for (point, size) in &point_map {
        let point_visible = count_visible(point, *size, &max_point, &point_map);
        if point_visible > max_visible_count {
            max_visible_count = point_visible;
        }

    }

    return max_visible_count;
}

#[cfg(test)]
mod tests {
    use crate::day8::puzzle1;
    use crate::day8::puzzle2;
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 5] = [
        "30373",
        "25512",
        "65332",
        "33549",
        "35390"];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, 21);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day8.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle1(&data[..]);
        assert_eq!(return_value, 1560);
    }

    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA);
        assert_eq!(return_value, 8);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day8.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data[..]);
        assert_eq!(return_value, 252000);
    }
}