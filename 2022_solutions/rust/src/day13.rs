enum Node {
    VALUE(usize),
    LIST(Vec<Node>),
}

fn parse_2(packet: &str) -> Node {
    let mut num = None;
    let mut vecs = vec![vec![]];
    let push_num = |num: Option<usize>, vecs: &mut Vec<Vec<Node>>| {
        let lev = vecs.len() - 1;
        if let Some(num) = num {
            vecs[lev].push(Node::VALUE(num));
        }
        None
    };
    for c in packet.chars() {
        match c {
            '[' => vecs.push(vec![]),
            ']' => {
                num = push_num(num, &mut vecs);
                let lev = vecs.len() - 1;
                let v = vecs.pop().unwrap();
                vecs[lev - 1].push(Node::LIST(v));
            }
            ' ' => {}
            ',' => num = push_num(num, &mut vecs),
            d => num = Some(num.unwrap_or(0) * 10 + (d as u8 - '0' as u8) as usize),
        };
    }
    Node::LIST(vecs.pop().unwrap())
}

fn compare_node_vecs(lhs: &[Node], rhs: &[Node]) -> bool {
    let left_first = lhs.first();
    let right_first = rhs.first();

    if left_first.is_none() {
        return true;
    }

    if right_first.is_none() {
        return false;
    }

    let lhs_node = left_first.unwrap();
    let rhs_node = right_first.unwrap();

    return match lhs_node {
        Node::VALUE(lhs_int) => {
            return match rhs_node {
                Node::VALUE(rhs_int) => {
                    if lhs_int == rhs_int {
                        return compare_node_vecs(&lhs[1..], &rhs[1..]);
                    } else {
                        return lhs_int < rhs_int;
                    }
                }
                Node::LIST(rhs_list) => {
                    let lhs_list = vec![Node::VALUE(*lhs_int)];
                    return compare_node_vecs(&lhs_list, rhs_list);
                }
            };
        }
        Node::LIST(lhs_list) => {
            return match rhs_node {
                Node::VALUE(rhs_int) => {
                    let rhs_list = vec![Node::VALUE(*rhs_int)];
                    return compare_node_vecs(lhs_list, &rhs_list);
                }
                Node::LIST(rhs_list) => {
                    return compare_node_vecs(lhs_list, rhs_list);
                }
            };
        }
    };
}

fn are_in_the_right_order(lhs: &str, rhs: &str) -> bool {
    let lhs_parse = parse_2(lhs);
    let rhs_parse = parse_2(rhs);

    return compare_node_vecs(&vec![lhs_parse], &vec![rhs_parse]);
}

fn puzzle1(input_array: &[&str]) -> usize {
    let mut vec = Vec::<usize>::new();
    let mut packet_count = 0usize;

    let mut input_iter = input_array.iter();
    loop {
        packet_count = packet_count + 1;
        let lhs_option = input_iter.next();

        if lhs_option == None {
            break;
        }

        let lhs = *lhs_option.unwrap();
        let rhs = *input_iter.next().unwrap();
        let _blank_line = input_iter.next();


        let correct_order = are_in_the_right_order(lhs, rhs);

        if correct_order {
            println!("line: {}: {}", packet_count, lhs);
            println!("line: {}: {}", packet_count, rhs);
            vec.push(packet_count);
        }
    }

    return vec.iter().sum();
}

fn puzzle2(input_array: &[&str]) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day13::{puzzle1, puzzle2};
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 23] = [
        "[1,1,3,1,1]",
        "[1,1,5,1,1]",
        "",
        "[[1],[2,3,4]]",
        "[[1],4]",
        "",
        "[9]",
        "[[8,7,6]]",
        "",
        "[[4,4],4,4]",
        "[[4,4],4,4,4]",
        "",
        "[7,7,7,7]",
        "[7,7,7]",
        "",
        "[]",
        "[3]",
        "",
        "[[[]]]",
        "[[]]",
        "",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]"];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, 13);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day13.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle1(&data);
        assert_eq!(return_value, 102399);
    }


    #[test]
    #[ignore]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA);
        assert_eq!(return_value, 2713310158);
    }

    #[test]
    #[ignore]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day13.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data);
        assert_eq!(return_value, 23641658401);
    }
}