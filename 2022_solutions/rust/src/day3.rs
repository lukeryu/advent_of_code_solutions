use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref PRIORITY: HashMap<&'static str, u64> = {
        let mut hash_map = HashMap::<&'static str, u64>::with_capacity(52);
        hash_map.insert("a", 1);
        hash_map.insert("b", 2u64);
        hash_map.insert("c", 3u64);
        hash_map.insert("d", 4u64);
        hash_map.insert("e", 5u64);
        hash_map.insert("f", 6u64);
        hash_map.insert("g", 7u64);
        hash_map.insert("h", 8u64);
        hash_map.insert("i", 9u64);
        hash_map.insert("j", 10u64);
        hash_map.insert("k", 11u64);
        hash_map.insert("l", 12u64);
        hash_map.insert("m", 13u64);
        hash_map.insert("n", 14u64);
        hash_map.insert("o", 15u64);
        hash_map.insert("p", 16u64);
        hash_map.insert("q", 17u64);
        hash_map.insert("r", 18u64);
        hash_map.insert("s", 19u64);
        hash_map.insert("t", 20u64);
        hash_map.insert("u", 21u64);
        hash_map.insert("v", 22u64);
        hash_map.insert("w", 23u64);
        hash_map.insert("x", 24u64);
        hash_map.insert("y", 25u64);
        hash_map.insert("z", 26u64);
        hash_map.insert("A", 27u64);
        hash_map.insert("B", 28u64);
        hash_map.insert("C", 29u64);
        hash_map.insert("D", 30u64);
        hash_map.insert("E", 31u64);
        hash_map.insert("F", 32u64);
        hash_map.insert("G", 33u64);
        hash_map.insert("H", 34u64);
        hash_map.insert("I", 35u64);
        hash_map.insert("J", 36u64);
        hash_map.insert("K", 37u64);
        hash_map.insert("L", 38u64);
        hash_map.insert("M", 39u64);
        hash_map.insert("N", 40u64);
        hash_map.insert("O", 41u64);
        hash_map.insert("P", 42u64);
        hash_map.insert("Q", 43u64);
        hash_map.insert("R", 44u64);
        hash_map.insert("S", 45u64);
        hash_map.insert("T", 46u64);
        hash_map.insert("U", 47u64);
        hash_map.insert("V", 48u64);
        hash_map.insert("W", 49u64);
        hash_map.insert("X", 50u64);
        hash_map.insert("Y", 51u64);
        hash_map.insert("Z", 52u64);
        return hash_map;
    };
}

fn get_priority_cost(datum: &String) -> u64 {
    let half_length = datum.len() / 2;
    let first_compartment = &datum[..half_length];
    let second_compartment = &datum[half_length..];

    for datum_element_char in first_compartment.chars() {
        let datum_element = datum_element_char.to_string();
        if second_compartment.contains(&datum_element) {
            let priority_element = PRIORITY.get(datum_element.as_str()).unwrap();
            return priority_element.clone();
        }
    }
    return 0;
}

fn get_badge_const(data: &[String]) -> u64 {
    let first_pack = data.get(0).unwrap();
    let second_pack = data.get(1).unwrap();
    let third_pack = data.get(2).unwrap();

    for datum_element_char in first_pack.chars() {
        let datum_element = datum_element_char.to_string();
        if second_pack.contains(&datum_element) && third_pack.contains(&datum_element) {
            let priority_element = PRIORITY.get(datum_element.as_str()).unwrap();
            return priority_element.clone();
        }
    }
    return 0;
}

fn puzzle1(data: &Vec<String>) -> u64 {
    let mut sum = 0;

    for datum in data {
        sum += get_priority_cost(&datum);
    }

    return sum;
}

fn puzzle2(data: &Vec<String>) -> u64 {
    let mut sum = 0;

    for index in (0..data.len()).step_by(3) {
        let slice = &data[index..index + 3];
        sum += get_badge_const(slice);
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use crate::day3::puzzle1;
    use crate::day3::puzzle2;
    use crate::utils::read_file_strings;

    const TEST_DATA: [&'static str; 6] = ["vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw"];

    #[test]
    fn test_puzzle1() {
        let data = TEST_DATA.into_iter()
            .map(String::from)
            .collect();

        let return_value = puzzle1(&data);
        assert_eq!(return_value, 157);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let data = read_file_strings("../data/Day3.txt");

        let return_value = puzzle1(&data);
        assert_eq!(return_value, 8185);
    }

    #[test]
    fn test_puzzle2() {
        let data = TEST_DATA.into_iter()
            .map(String::from)
            .collect();

        let return_value = puzzle2(&data);
        assert_eq!(return_value, 70);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let data = read_file_strings("../data/Day3.txt");

        let return_value = puzzle2(&data);
        assert_eq!(return_value, 2817);
    }
}