use lazy_static::lazy_static;
use regex::Regex;
use crate::cartesian::{Grid, Point};

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels lead to valves ([A-Z]{2})(?:, ([A-Z]{2}))+$").unwrap();
}

struct Valve {
    flow_rate: usize,
    name: String,
    lead_to_valves: Vec<String>,
}

fn parse_valves(input_array: &[&str]) -> Vec<Valve> {
    let mut valves = Vec::new();

    for input in input_array {
        let captures = REGEX.captures(*input).unwrap();

        let name_str = captures.get(1).unwrap().as_str();
        let flow_rate = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let name = String::from(name_str);

        let mut lead_to_valves = Vec::new();
        for match_opt in captures.iter().skip(3) {
            lead_to_valves.push(String::from(match_opt.unwrap().as_str()));
        }

        valves.push(Valve {
            flow_rate,
            name,
            lead_to_valves,
        });
    }

    return valves;
}

fn puzzle1(input_array: &[&str]) -> usize {
    let mut minutes_remaining = 30;
    let mut pressure_released = 0;
    let valves = parse_valves(input_array);

    return 0;
}

fn puzzle2(input_array: &[&str]) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day16::{puzzle1, puzzle2};
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 10] = [
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
        "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
        "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
        "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
        "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
        "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
        "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
        "Valve HH has flow rate=22; tunnel leads to valve GG",
        "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
        "Valve JJ has flow rate=21; tunnel leads to valve II"];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, 1651);
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
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA);
        assert_eq!(return_value, 2713310158);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day13.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data);
        assert_eq!(return_value, 23641658401);
    }
}