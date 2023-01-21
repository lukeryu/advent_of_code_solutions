use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone)]
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

fn compare_node_vecs(lhs: &[Node], rhs: &[Node]) -> Ordering {
    let left_first = lhs.first();
    let right_first = rhs.first();

    if left_first.is_none() {
        return Ordering::Less;
    }

    if right_first.is_none() {
        return Ordering::Greater;
    }

    let lhs_node = left_first.unwrap();
    let rhs_node = right_first.unwrap();

    match lhs_node {
        Node::VALUE(lhs_int) => {
            match rhs_node {
                Node::VALUE(rhs_int) => {
                    if lhs_int == rhs_int {
                        return compare_node_vecs(&lhs[1..], &rhs[1..]);
                    } else {
                        return lhs_int.cmp(rhs_int);
                    }
                }
                Node::LIST(rhs_list) => {
                    let lhs_list = vec![Node::VALUE(*lhs_int)];
                    return compare_node_vecs(&lhs_list, rhs_list);
                }
            };
        }
        Node::LIST(lhs_list) => {
            match rhs_node {
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

    // let order = compare_node_vecs(&vec![lhs_parse], &vec![rhs_parse]);
    let order = compare(&lhs_parse, &rhs_parse);
    return order != Ordering::Greater;
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

fn compare(first: &Node, second: &Node) -> Ordering {
    match (first, second) {
        (Node::VALUE(f_val), Node::VALUE(s_val)) => (*f_val).cmp(s_val),
        (Node::LIST(f_list), Node::LIST(s_list)) => {
            let mut i = 0;
            while i < f_list.len() && i < s_list.len() {
                match compare(&f_list[i], &s_list[i]) {
                    Ordering::Equal => {}
                    other => return other,
                };
                i += 1;
            }
            f_list.len().cmp(&s_list.len())
        }
        (l, Node::VALUE(v)) => compare(l, &Node::LIST(vec![Node::VALUE(*v)])),
        (Node::VALUE(v), l) => compare(&Node::LIST(vec![Node::VALUE(*v)]), l),
    }
}


// fn solve(data: &str) {
//     let mut nodes = data
//         .split('\n')
//         .filter(|l| !l.is_empty())
//         .map(|l| parse_2(l))
//         .collect::<Vec<_>>();
//     nodes.append(&mut vec![parse_2("[[2]]"), parse_2("[[6]]")]);
//     nodes.sort_by(|x, y| compare(x, y));
//     let divider_nodes = vec![parse_2("[[2]]"), parse_2("[[6]]")];
//     let decoder_key = (0..nodes.len())
//         .filter(|&i| divider_nodes.contains(&nodes[i]))
//         .map(|i| i + 1)
//         .product::<usize>();
//     println!("decoder_key: {}", decoder_key);
// }

fn puzzle2(input_array: &[&str]) -> usize {
    let mut vec_nodes = Vec::<Node>::new();

    let node1 = parse_2("[[2]]");
    vec_nodes.push(node1.clone());
    let node2 = parse_2("[[6]]");
    vec_nodes.push(node2.clone());

    for input in input_array {
        if !input.trim().is_empty() {
            let node = parse_2(*input);

            vec_nodes.push(node);
        }
    }

    vec_nodes.sort_by(compare);

    let index1 = vec_nodes.iter().position(|r| r == &node1).unwrap() + 1;
    let index2 = vec_nodes.iter().position(|r| r == &node2).unwrap() + 1;

    println!("2={}, 6={}", index1, index2);

    return index1 * index2;
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::day13::{compare_node_vecs, parse_2, puzzle1, puzzle2};
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
        assert_eq!(return_value, 6072);
    }

    #[test]
    fn test_thing() {
        let lhs = "[[3],[],[],[9,6,[0,8,[2,2,4,8],0,[5,2,7,4,8]]],[[5],[[],0,[6,0],9,7],2]]";
        let rhs = "[[3]]";

        let lhs_node = parse_2(lhs);
        let rhs_node = parse_2(rhs);

        let comp = compare_node_vecs(&[lhs_node], &[rhs_node]);
        assert_eq!(comp, Ordering::Greater);
    }


    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA);
        assert_eq!(return_value, 140);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day13.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data);
        assert_eq!(return_value, 22184);
    }
}