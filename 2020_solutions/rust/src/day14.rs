use std::collections::BTreeMap;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn puzzle1(inputs: Vec<String>) -> u64 {
    let mut memory = BTreeMap::<u64, u64>::new();
    let mut or_mask: u64 = 0;
    let mut and_mask: u64 = 0;
    let rx_mask = Regex::new(r"mask = ([X10]{36})").unwrap();
    let rx_mem = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    for line in inputs {
        for captures in rx_mask.captures(&line) {
            let mask_string = &captures[1];
            let mut two_power = 36;
            let mut off_bits: u64 = 0;
            let mut on_bits: u64 = 0;
            for mask_char in mask_string.chars() {
                two_power -= 1;
                let bit_position: u64 = 1 << two_power;
                if mask_char == '0' {
                    off_bits |= bit_position
                }
                if mask_char == '1' {
                    on_bits |= bit_position
                }
            }
            or_mask = on_bits;
            and_mask = !off_bits;
            // println!("{} {:b} {:b}", mask_string, or_mask, and_mask);
        }
        for captures in rx_mem.captures(&line) {
            let addr: u64 = captures[1].parse().unwrap();
            let unmasked_val: u64 = captures[2].parse().unwrap();
            let masked_val: u64 = {
                let bits_off_val: u64 = unmasked_val & and_mask;
                let bits_on_val: u64 = bits_off_val | or_mask;
                // println!("{} <- val: {}, off: {}, on: {}", addr, unmasked_val, bits_off_val, bits_on_val);
                bits_on_val
            };
            memory.insert(addr, masked_val);
        }
    }

    memory.iter()
        .for_each(|(key, value)| println!("mem[{}] = {}", *key, *value));

    println!("Size: {}", memory.len());

    // println!("Memory: {:?}", memory);
    return memory.values().fold(0, |acc, v| acc + *v);
    // println!("Sum of all values: {}", sum)
}

fn puzzle2(inputs: Vec<String>) -> u64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day14::{puzzle1, puzzle2};
    use crate::utils::read_lines;

    #[test]
    fn test_puzzle_1_1() {
        let vec = vec![String::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
                       String::from("mem[8] = 11"),
                       String::from("mem[7] = 101"),
                       String::from("mem[8] = 0")];

        let result = puzzle1(vec);
        assert_eq!(result, 165);
    }

    #[test]
    fn test_puzzle_1_2() {
        let strs = read_lines("data/Day14.txt").unwrap();
        let result = puzzle1(strs);
        assert_eq!(result, 9967721333886);
    }

    // #[test]
    // fn test_puzzle_2_1() {
    //     let result = puzzle2(String::from("7,13,x,x,59,x,31,19"));
    //     assert_eq!(result, 1068781);
    //
    //     let result2 = puzzle2(String::from("17,x,13,19"));
    //     assert_eq!(result2, 3417);
    //
    //     let result3 = puzzle2(String::from("67,7,59,61"));
    //     assert_eq!(result3, 754018);
    //
    //     let result4 = puzzle2(String::from("67,x,7,59,61"));
    //     assert_eq!(result4, 779210);
    //
    //     let result5 = puzzle2(String::from("67,7,x,59,61"));
    //     assert_eq!(result5, 1261476);
    //
    //     let result6 = puzzle2(String::from("1789,37,47,1889"));
    //     assert_eq!(result6, 1202161486);
    // }
    //
    // #[test]
    // fn test_puzzle_2_2() {
    //     let result = puzzle2(String::from("23,x,x,x,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,509,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,401,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,19"));
    //     assert_eq!(result, 225850756401039);
    // }
}