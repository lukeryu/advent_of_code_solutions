use std::collections::LinkedList;
use std::ops::AddAssign;

type MonkeyIndexType = usize;
type ItemType = u128;

#[derive(Copy, Clone)]
enum Operation {
    ADD(ItemType),
    MULT(ItemType),
    SQUARE,
}

#[derive(Clone)]
struct Monkey {
    id: MonkeyIndexType,
    items: LinkedList<ItemType>,
    operation: Operation,
    test: ItemType,
    if_true: MonkeyIndexType,
    if_false: MonkeyIndexType,
}

fn perform_operation(operation: &Operation, value: ItemType) -> ItemType {
    match operation {
        Operation::ADD(add_to) => add_to + value,
        Operation::MULT(mult_by) => mult_by * value,
        Operation::SQUARE => value * value,
    }
}

fn perform_test(monkey: &Monkey, value: &ItemType) -> MonkeyIndexType {
    if value % monkey.test == 0 {
        return monkey.if_true;
    } else {
        return monkey.if_false;
    }
}

fn run_simulation(input_array: &mut [Monkey], divisor: ItemType, iterations: usize) -> ItemType {
    let mut monkey_id_count = vec![0u128; input_array.len()];

    let common_div: ItemType = input_array.iter()
        .map(|m| m.test)
        .product();

    for _round_count in 0..iterations {
        for current_monkey_index in 0..input_array.len() {
            let mut current_monkey = input_array.get(current_monkey_index).unwrap().clone();
            let item_length = current_monkey.items.len() as ItemType;
            monkey_id_count.get_mut(current_monkey_index)
                .unwrap()
                .add_assign(item_length);

            let items = input_array[current_monkey_index].items.clone();
            input_array[current_monkey_index].items.clear();
            for item in items {
                let operation = &current_monkey.operation;
                let new_value = perform_operation(operation, item);
                let mut bored_level = new_value / divisor;
                bored_level = bored_level % common_div;
                let next_monkey = perform_test(&current_monkey, &bored_level);
                input_array[next_monkey].items.push_back(bored_level);
            }
        }
    }

    monkey_id_count.sort_by(|a, b| b.cmp(a));

    let monkey_business = monkey_id_count.iter()
        .take(2)
        .product();

    return monkey_business;
}

fn puzzle1(input_array: &mut [Monkey]) -> ItemType {
    return run_simulation(input_array, 3, 20);
}

fn puzzle2(input_array: &mut [Monkey]) -> ItemType {
    return run_simulation(input_array, 1, 10000);
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    use crate::day11::{Monkey, puzzle1};
    use crate::day11::Operation::{ADD, MULT, SQUARE};
    use crate::day11::puzzle2;

    fn get_test_data() -> [Monkey; 4] {
        return [
            Monkey {
                id: 0,
                items: LinkedList::from([79, 98]),
                operation: MULT(19),
                test: 23,
                if_true: 2,
                if_false: 3,
            },
            Monkey {
                id: 1,
                items: LinkedList::from([54, 65, 75, 74]),
                operation: ADD(6),
                test: 19,
                if_true: 2,
                if_false: 0,
            },
            Monkey {
                id: 2,
                items: LinkedList::from([79, 60, 97]),
                operation: SQUARE,
                test: 13,
                if_true: 1,
                if_false: 3,
            },
            Monkey {
                id: 3,
                items: LinkedList::from([74]),
                operation: ADD(3),
                test: 17,
                if_true: 0,
                if_false: 1,
            }
        ];
    }

    fn get_real_data() -> [Monkey; 8] {
        let monkey_0 = Monkey {
            id: 0,
            items: LinkedList::from([50, 70, 54, 83, 52, 78]),
            operation: MULT(3),
            test: 11,
            if_true: 2,
            if_false: 7,
        };
        let monkey_1 = Monkey {
            id: 1,
            items: LinkedList::from([71, 52, 58, 60, 71]),
            operation: SQUARE,
            test: 7,
            if_true: 0,
            if_false: 2,
        };
        let monkey_2 = Monkey {
            id: 2,
            items: LinkedList::from([66, 56, 56, 94, 60, 86, 73]),
            operation: ADD(1),
            test: 3,
            if_true: 7,
            if_false: 5,
        };
        let monkey_3 = Monkey {
            id: 3,
            items: LinkedList::from([83, 99]),
            operation: ADD(8),
            test: 5,
            if_true: 6,
            if_false: 4,
        };
        let monkey_4 = Monkey {
            id: 4,
            items: LinkedList::from([98, 98, 79]),
            operation: ADD(3),
            test: 17,
            if_true: 1,
            if_false: 0,
        };
        let monkey_5 = Monkey {
            id: 5,
            items: LinkedList::from([76]),
            operation: ADD(4),
            test: 13,
            if_true: 6,
            if_false: 3,
        };
        let monkey_6 = Monkey {
            id: 6,
            items: LinkedList::from([52, 51, 84, 54]),
            operation: MULT(17),
            test: 19,
            if_true: 4,
            if_false: 1,
        };
        let monkey_7 = Monkey {
            id: 7,
            items: LinkedList::from([82, 86, 91, 79, 94, 92, 59, 94]),
            operation: ADD(7),
            test: 2,
            if_true: 5,
            if_false: 3,
        };
        return [
            monkey_0,
            monkey_1,
            monkey_2,
            monkey_3,
            monkey_4,
            monkey_5,
            monkey_6,
            monkey_7,
        ];
    }

    #[test]
    fn test_puzzle1() {
        let mut a_test = get_test_data();

        let return_value = puzzle1(&mut a_test);
        assert_eq!(return_value, 10605);
    }

    #[test]
    #[ignore]
    fn test_puzzle1_realdata() {
        let mut test_data= get_real_data();

        let return_value = puzzle1(&mut test_data);
        assert_eq!(return_value, 102399);
    }


    #[test]
    fn test_puzzle2() {
        let mut a_test = get_test_data();

        let return_value = puzzle2(&mut a_test);
        assert_eq!(return_value, 2713310158);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut test_data= get_real_data();

        let return_value = puzzle2(&mut test_data);
        assert_eq!(return_value, 23641658401);
    }
}