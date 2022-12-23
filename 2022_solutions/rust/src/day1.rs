use std::collections::HashMap;

fn parse_elf_packs(data: &[&str]) -> HashMap::<u64, Vec<u64>> {
    let mut elf_count = 1;
    let mut current_array = Vec::<u64>::new();
    let mut elf_packs = HashMap::<u64, Vec<u64>>::new();

    for data_row in data {
        if data_row.len() == 0 {
            elf_packs.insert(elf_count, current_array);
            current_array = Vec::<u64>::new();
            elf_count = elf_count + 1;
        } else {
            current_array.push(data_row.parse::<u64>().unwrap());
        }
    }

    if !current_array.is_empty() {
        elf_packs.insert(elf_count, current_array);
    }

    return elf_packs;
}

fn puzzle1(input: &[&str]) -> u64 {
    let elf_packs = parse_elf_packs(&input);

    let mut values: Vec<u64> = elf_packs.iter().map(|(_key, value)| -> u64 {
        value.iter().sum()
    }).collect();

    values.sort();

    return values.last().unwrap().clone();
}

fn puzzle2(data: &[&str]) -> u64 {
    let elf_packs = parse_elf_packs(&data);

    let mut values: Vec<u64> = elf_packs.iter().map(|(_key, value)| -> u64 {
        value.iter().sum()
    }).collect();

    values.sort();
    values.reverse();

    return values.iter().take(3).sum();
}

#[cfg(test)]
mod tests {
    use crate::day1::puzzle1;
    use crate::day1::puzzle2;
    use crate::utils::{read_file_strings};

    const TEST_DATA: [&str; 14] = ["1000",
                         "2000",
                         "3000",
                         "",
                         "4000",
                         "",
                         "5000",
                         "6000",
                         "",
                         "7000",
                         "8000",
                         "9000",
                         "",
                         "10000"];

    #[test]
    fn test_puzzle1() {

        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, 24000);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut strs= Vec::new();
        let vec = read_file_strings("../data/Day1.txt");
        vec.iter()
            .map(|string| string.trim())
            .for_each(|string| strs.push(string));

        let return_value = puzzle1(&strs);
        assert_eq!(return_value, 71780);
    }

    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA);
        assert_eq!(return_value, 45000);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut strs= Vec::new();
        let vec = read_file_strings("../data/Day1.txt");
        vec.iter()
            .map(|string| string.trim())
            .for_each(|string| strs.push(string));

        let return_value = puzzle2(&strs);
        assert_eq!(return_value, 212489);
    }
}