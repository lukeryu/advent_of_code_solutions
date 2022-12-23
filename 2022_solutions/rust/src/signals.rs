use std::collections::HashSet;

pub fn start_of_packet_marker(input_string: &str) -> usize {
    for index in 0..input_string.len() - 3 {
        let count = input_string.chars()
            .skip(index)
            .take(4)
            .collect::<HashSet<char>>()
            .len();

        if count == 4 {
            return index + 4;
        }
    }

    return 0;
}

pub fn start_of_message_marker(input_string: &str) -> usize {
    for index in 0..input_string.len() - 13 {
        let count = input_string.chars()
            .skip(index)
            .take(14)
            .collect::<HashSet<char>>()
            .len();

        if count == 14 {
            return index + 14;
        }
    }

    return 0;
}