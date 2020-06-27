use std::char::from_u32;

fn puzzle1(input: String, width: u32, height: u32) -> u32 {
    let layer_size = (width * height) as usize;
    let values: Vec<u32> = input.chars()
        .map(to_u32)
        .collect();

    let mut layers = Vec::<Vec<u32>>::with_capacity(values.len() / layer_size);

    let mut current_vector = Vec::<u32>::with_capacity(layer_size);
    for index in 0..values.len() {
        if (index % layer_size == 0 && !current_vector.is_empty()) {
            layers.push(current_vector);
            current_vector = Vec::<u32>::with_capacity(layer_size);
        }

        current_vector.push(values.get(index).unwrap().clone())
    }

    if (!current_vector.is_empty()) {
        layers.push(current_vector);
        current_vector = Vec::<u32>::with_capacity(layer_size);
    }

    let mut min_layer_index = usize::MAX;
    let mut min_zero_count = usize::MAX;

    for layer_index in 0..layers.len() {
        let layer = layers.get(layer_index).unwrap();
        let layer_zero_count = layer.iter()
            .filter(|value| value == &&0)
            .count();

        if (layer_zero_count < min_zero_count) {
            min_layer_index = layer_index;
            min_zero_count = layer_zero_count;
        }
    }

    let min_layer = layers.get(min_layer_index).unwrap();

    let layer_one_count = min_layer.iter()
        .filter(|value| value == &&1)
        .count();

    let layer_two_count = min_layer.iter()
        .filter(|value| value == &&2)
        .count();

    return (layer_one_count * layer_two_count) as u32;
}

fn to_u32(c: char) -> u32 {
    return c.to_digit(10).unwrap();
}

fn to_usize(c: char) -> usize {
    return c.to_digit(10).unwrap() as usize;
}

fn puzzle2(input: String, width: usize, height: usize) -> String {
    let layer_size = width * height;
    let values: Vec<u32> = input.chars()
        .map(to_u32)
        .collect();

    let mut layers = Vec::<Vec<u32>>::with_capacity(values.len() / layer_size);

    let mut current_vector = Vec::<u32>::with_capacity(layer_size);
    for index in 0..values.len() {
        if (index % layer_size == 0 && !current_vector.is_empty()) {
            layers.push(current_vector);
            current_vector = Vec::<u32>::with_capacity(layer_size);
        }

        current_vector.push(values.get(index).unwrap().clone())
    }

    if (!current_vector.is_empty()) {
        layers.push(current_vector);
        current_vector = Vec::<u32>::with_capacity(layer_size);
    }

    let mut rendered_image = Vec::<u32>::with_capacity(layer_size);

    for vertical_index in 0..height {
        for horizontal_index in 0..width {
            let mut color = 2;
            let pixel_index = horizontal_index + vertical_index * width;
            for layer_index in layers.iter() {
                if (color == 2) {
                    let current_layer = layer_index.get(pixel_index).unwrap();
                    color = current_layer.clone();
                }
            }
            rendered_image.push(color);
        }
    }

    println!("{:?}", rendered_image);
    return rendered_image.iter()
        .fold(String::from(""), |acc, c| -> String { format!("{}{}", acc, c.to_string()) });
}

#[cfg(test)]
mod tests {
    use crate::utils;
    use crate::day8::{puzzle1, puzzle2};

    struct Puzzle1Test {
        test_data: String,
        width: u32,
        height: u32,
        expected_result: u32,
    }

    #[test]
    fn test_puzzle_1() {
        let mut tests: Vec<Puzzle1Test> = Vec::new();
        tests.push(Puzzle1Test {
            test_data: String::from("123456789012"),
            width: 3,
            height: 2,
            expected_result: 1,
        });
        match utils::read_lines("data/Day8.txt") {
            Ok(lines) => {
                tests.push(Puzzle1Test {
                    test_data: lines.get(0).unwrap().clone(),
                    width: 25,
                    height: 6,
                    expected_result: 2480,
                });
                for test in tests {
                    let result = puzzle1(test.test_data, test.width, test.height);
                    assert_eq!(result, test.expected_result);
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }

    struct Puzzle2Test {
        test_data: String,
        width: usize,
        height: usize,
        expected_result: String,
    }

    #[test]
    fn test_puzzle_2() {
        let mut tests: Vec<Puzzle2Test> = Vec::new();
        tests.push(Puzzle2Test {
            test_data: String::from("0222112222120000"),
            width: 2,
            height: 2,
            expected_result: String::from("0110"),
        });
        match utils::read_lines("data/Day8.txt") {
            Ok(lines) => {
                tests.push(Puzzle2Test {
                    test_data: lines.get(0).unwrap().clone(),
                    width: 25,
                    height: 6,
                    expected_result: String::from("111101000111100100001001000010100011001010000100100010001010111001000011110010000010010010100001001010000001001001010000100101111000100111001111010010"),
                });
                for test in tests {
                    let result = puzzle2(test.test_data, test.width, test.height);
                    assert_eq!(result, test.expected_result);
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}