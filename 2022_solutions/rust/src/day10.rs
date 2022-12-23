use std::collections::LinkedList;

fn parse_instructions(input_array: &[&str]) -> LinkedList<i64> {
    let mut queue = LinkedList::new();

    for instruction in input_array {
        if *instruction == "noop" {
            queue.push_front(0);
        } else if instruction.starts_with("addx") {
            let mut fields = instruction.split_whitespace();
            let value = fields.nth(1).unwrap().parse::<i64>().unwrap();
            queue.push_front(0);
            queue.push_front(value);
        }
    }

    return queue;
}

fn puzzle1(input_array: &[&str], iterations: usize) -> i64 {
    let mut register_value = 1;

    let mut signal_strengths = Vec::new();

    let mut queue = parse_instructions(input_array);

    for index in 0..iterations {
        let cycle = (index + 1) as i64;

        let value = queue.pop_back();

        match value {
            Some(value) => register_value = register_value + value,
            None => continue
        }

        if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
            let signal = register_value * (cycle as i64);
            signal_strengths.push(signal);
        }
        // register_value = register_value + queue.pop_back().unwrap();
    }

    return signal_strengths.iter().sum();
}

fn puzzle2(input_array: &[&str], iterations: usize) -> i64 {
    let mut register_value = 1;

    let mut signal_strengths = ["."; 240];

    let mut queue = parse_instructions(input_array);

    for pixel_index in 0..iterations {

        let queue_value = queue.pop_back();

        let pixel_row = (pixel_index % 40)  as i64;

        if ((register_value) - pixel_row).abs() < 2 {
            signal_strengths[pixel_index] = "#";
        }

        register_value = register_value + queue_value.unwrap();
    }

    for row in 0..6 as usize {
        let slice = &signal_strengths[40 * row..40*(row + 1) - 1];
        println!("{:?}", slice);
    }


    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day10::puzzle1;
    use crate::day10::puzzle2;
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 3] = [
        "noop",
        "addx 3",
        "addx -5"];

    const TEST_DATA_2: [&str; 146] = ["addx 15",
        "addx -11",
        "addx 6",
        "addx -3",
        "addx 5",
        "addx -1",
        "addx -8",
        "addx 13",
        "addx 4",
        "noop",
        "addx -1",
        "addx 5",
        "addx -1",
        "addx 5",
        "addx -1",
        "addx 5",
        "addx -1",
        "addx 5",
        "addx -1",
        "addx -35",
        "addx 1",
        "addx 24",
        "addx -19",
        "addx 1",
        "addx 16",
        "addx -11",
        "noop",
        "noop",
        "addx 21",
        "addx -15",
        "noop",
        "noop",
        "addx -3",
        "addx 9",
        "addx 1",
        "addx -3",
        "addx 8",
        "addx 1",
        "addx 5",
        "noop",
        "noop",
        "noop",
        "noop",
        "noop",
        "addx -36",
        "noop",
        "addx 1",
        "addx 7",
        "noop",
        "noop",
        "noop",
        "addx 2",
        "addx 6",
        "noop",
        "noop",
        "noop",
        "noop",
        "noop",
        "addx 1",
        "noop",
        "noop",
        "addx 7",
        "addx 1",
        "noop",
        "addx -13",
        "addx 13",
        "addx 7",
        "noop",
        "addx 1",
        "addx -33",
        "noop",
        "noop",
        "noop",
        "addx 2",
        "noop",
        "noop",
        "noop",
        "addx 8",
        "noop",
        "addx -1",
        "addx 2",
        "addx 1",
        "noop",
        "addx 17",
        "addx -9",
        "addx 1",
        "addx 1",
        "addx -3",
        "addx 11",
        "noop",
        "noop",
        "addx 1",
        "noop",
        "addx 1",
        "noop",
        "noop",
        "addx -13",
        "addx -19",
        "addx 1",
        "addx 3",
        "addx 26",
        "addx -30",
        "addx 12",
        "addx -1",
        "addx 3",
        "addx 1",
        "noop",
        "noop",
        "noop",
        "addx -9",
        "addx 18",
        "addx 1",
        "addx 2",
        "noop",
        "noop",
        "addx 9",
        "noop",
        "noop",
        "noop",
        "addx -1",
        "addx 2",
        "addx -37",
        "addx 1",
        "addx 3",
        "noop",
        "addx 15",
        "addx -21",
        "addx 22",
        "addx -6",
        "addx 1",
        "noop",
        "addx 2",
        "addx 1",
        "noop",
        "addx -10",
        "noop",
        "noop",
        "addx 20",
        "addx 1",
        "addx 2",
        "addx 2",
        "addx -6",
        "addx -11",
        "noop",
        "noop",
        "noop"];

    #[test]
    #[ignore]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA_2, 220);
        assert_eq!(return_value, 13140);
    }

    #[test]
    fn test_puzzle1_asdf() {
        let return_value = puzzle1(&TEST_DATA, 5);
        assert_eq!(return_value, 0);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day10.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle1(&data[..], 220);
        assert_eq!(return_value, 15260);
    }


    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA_2, 240);
        assert_eq!(return_value, 0);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day10.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data[..], 240);
        assert_eq!(return_value, 0);
        //PGHFGLUG
    }
}