use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref VALUE_REGEX: Regex = Regex::new(r"^(\d+)$").unwrap();
    static ref FORMULA_REGEX: Regex = Regex::new(r"^(\w+) (\+|-|/|\*) (\w+)$").unwrap();
}

trait Instruction {
    fn get_value(&self, instruction_map: &HashMap<String, Box<dyn Instruction>>) -> i64;
    fn has_equal_halves(&self, instruction_map: &HashMap<String, Box<dyn Instruction>>) -> bool;
}

struct ValueInstruction {
    value: i64,
}

impl Instruction for ValueInstruction {
    fn get_value(&self, _: &HashMap<String, Box<dyn Instruction>>) -> i64 {
        return self.value;
    }
    fn has_equal_halves(&self, _: &HashMap<String, Box<dyn Instruction>>) -> bool {
        return false;
    }
}

struct FormulaInstruction {
    lhs_reference: String,
    rhs_reference: String,
    operation: String,
}

impl Instruction for FormulaInstruction {
    fn get_value(&self, instruction_map: &HashMap<String, Box<dyn Instruction>>) -> i64 {
        let lhs_instruction = instruction_map.get(self.lhs_reference.as_str()).unwrap();
        let lhs_value = lhs_instruction.get_value(instruction_map);

        let rhs_instruction = instruction_map.get(self.rhs_reference.as_str()).unwrap();
        let rhs_value = rhs_instruction.get_value(instruction_map);

        match self.operation.as_str() {
            "*" => lhs_value * rhs_value,
            "/" => lhs_value / rhs_value,
            "-" => lhs_value - rhs_value,
            "+" => lhs_value + rhs_value,
            _ => panic!("Unknown Operation Type")
        }
    }

    fn has_equal_halves(&self, instruction_map: &HashMap<String, Box<dyn Instruction>>) -> bool {
        let lhs_instruction = instruction_map.get(self.lhs_reference.as_str()).unwrap();
        let lhs_value = lhs_instruction.get_value(instruction_map);

        let rhs_instruction = instruction_map.get(self.rhs_reference.as_str()).unwrap();
        let rhs_value = rhs_instruction.get_value(instruction_map);

        return lhs_value == rhs_value;
    }
}

fn get_instruction_map(input_array: &[&str]) -> HashMap<String, Box<dyn Instruction>> {
    let mut hash_map = HashMap::<String, Box<dyn Instruction>>::new();

    for input_string in input_array {
        let (monkey_name, instruction_string) = input_string.split_at(4);

        let trimmed_instruction = instruction_string[2..].trim();
        if VALUE_REGEX.is_match(trimmed_instruction) {
            let value = VALUE_REGEX.captures(trimmed_instruction).unwrap().get(1).unwrap().as_str().parse::<i64>().unwrap();

            let value_instruction = ValueInstruction {
                value
            };

            hash_map.insert(String::from(monkey_name), Box::new(value_instruction));
        } else {
            let captures = FORMULA_REGEX.captures(trimmed_instruction).unwrap();
            let lhs_reference = String::from(captures.get(1).unwrap().as_str());
            let operation = String::from(captures.get(2).unwrap().as_str());
            let rhs_reference = String::from(captures.get(3).unwrap().as_str());

            let formula_instruction = FormulaInstruction {
                lhs_reference,
                operation,
                rhs_reference,
            };

            hash_map.insert(String::from(monkey_name), Box::new(formula_instruction));
        }
    }

    return hash_map;
}

fn puzzle1(input_array: &[&str]) -> i64 {
    let instruction_map = get_instruction_map(input_array);
    let root_instruction = instruction_map.get("root").unwrap();

    return (*root_instruction).get_value(&instruction_map);
}

fn path_to_node(monkeys: &HashMap<String, Box<dyn Instruction>>, node: &str) -> Option<Vec<String>> {
    let mut path_to_node = Vec::new();

    let mut stk = Vec::new();
    let root = monkeys.get(&"root".to_string()).unwrap();
    let root_deps = root.depends_on().unwrap();
    stk.push((root_deps[0].clone(), vec!["root".to_string()]));
    stk.push((root_deps[1].clone(), vec!["root".to_string()]));
    while let Some((monkey_name, path)) = stk.pop() {
        if monkey_name == node {
            path_to_node = path;
            break;
        }

        let monkey = monkeys.get(&monkey_name).context("monkey not found")?;
        match monkey.depends_on() {
            Some(deps) => {
                let mut path = path.clone();
                path.push(monkey_name);
                stk.push((deps[0].clone(), path.clone()));
                stk.push((deps[1].clone(), path.clone()));
            }
            None => (),
        }
    }
    Some(path_to_node)
}

fn puzzle2(input_array: &[&str]) -> i64 {
    let mut instruction_map = get_instruction_map(input_array);

    let path_to_humn = path_to_node(&instruction_map, "humn").unwrap();

    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day21::{puzzle1, puzzle2};
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 15] = [
        "root: pppw + sjmn",
        "dbpl: 5",
        "cczh: sllz + lgvd",
        "zczc: 2",
        "ptdq: humn - dvpt",
        "dvpt: 3",
        "lfqf: 4",
        "humn: 5",
        "ljgn: 2",
        "sjmn: drzm * dbpl",
        "sllz: 4",
        "pppw: cczh / lfqf",
        "lgvd: ljgn * ptdq",
        "drzm: hmdt - zczc",
        "hmdt: 32"];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, 152);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day21.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle1(&data);
        assert_eq!(return_value, 56490240862410);
    }


    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA);
        assert_eq!(return_value, 301);
    }

    #[test]
    #[ignore]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day21.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data);
        assert_eq!(return_value, 23641658401);
    }
}