use std::cmp::max;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").unwrap();
}

struct BlueprintStartingInfo {
    id_number: usize,
    ore_robot_ore_cost: usize,
    clay_robot_ore_cost: usize,
    obsidian_robot_ore_cost: usize,
    obsidian_robot_clay_cost: usize,
    geode_robot_ore_cost: usize,
    geode_robot_obsidian_cost: usize,
    max_ore_robots: usize,
}

#[derive(Copy, Clone)]
struct BlueprintCurrentState {
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robot: usize,
    ore_count: usize,
    clay_count: usize,
    obsidian_count: usize,
    geode_count: usize,
}

impl BlueprintCurrentState {
    fn init_state() -> Self {
        return Self {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robot: 0,
            ore_count: 0,
            clay_count: 0,
            obsidian_count: 0,
            geode_count: 0,
        };
    }

    fn iterate(&mut self) {
        self.ore_count = self.ore_count + self.ore_robots;
        self.clay_count = self.clay_count + self.clay_robots;
        self.obsidian_count = self.obsidian_count + self.obsidian_robots;
        self.geode_count = self.geode_count + self.geode_robot;
    }

    fn iterate_many(&mut self, iter_count: usize) {
        self.ore_count = self.ore_count + (self.ore_robots * iter_count);
        self.clay_count = self.clay_count + (self.clay_robots * iter_count);
        self.obsidian_count = self.obsidian_count + (self.obsidian_robots * iter_count);
        self.geode_count = self.geode_count + (self.geode_robot * iter_count);
    }
}

fn parse_blueprint_starting_info(input_string: &str) -> BlueprintStartingInfo {
    let captures = REGEX.captures(input_string).unwrap();

    let id_number = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let ore_robot_ore_cost = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let clay_robot_ore_cost = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
    let obsidian_robot_ore_cost = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
    let obsidian_robot_clay_cost = captures.get(5).unwrap().as_str().parse::<usize>().unwrap();
    let geode_robot_ore_cost = captures.get(6).unwrap().as_str().parse::<usize>().unwrap();
    let geode_robot_obsidian_cost = captures.get(7).unwrap().as_str().parse::<usize>().unwrap();

    let max_ore_robots = max(max(geode_robot_ore_cost, obsidian_robot_ore_cost), clay_robot_ore_cost);

    return BlueprintStartingInfo {
        id_number,
        ore_robot_ore_cost,
        clay_robot_ore_cost,
        obsidian_robot_ore_cost,
        obsidian_robot_clay_cost,
        geode_robot_ore_cost,
        geode_robot_obsidian_cost,
        max_ore_robots,
    };
}

fn calculate_minutes_to_robot(robot_cost: usize, number_of_robots: usize, current_amount: usize) -> usize {
    if current_amount >= robot_cost {
        return 1;
    }

    return max((robot_cost - current_amount + number_of_robots - 1) / number_of_robots, 0) + 1;
}

fn run_blueprint_scenario(blueprint_starting_info: &BlueprintStartingInfo, blueprint_current_state: BlueprintCurrentState, minutes_remaining: usize) -> usize {
    if minutes_remaining <= 0 {
        return blueprint_current_state.geode_count;
    }

    let mut results = Vec::<usize>::with_capacity(5);

    //Build a Geode Robot
    if blueprint_current_state.obsidian_robots > 0 {
        let minutes_to_obsidian = calculate_minutes_to_robot(blueprint_starting_info.geode_robot_obsidian_cost, blueprint_current_state.obsidian_robots, blueprint_current_state.obsidian_count);
        let minutes_to_ore = calculate_minutes_to_robot(blueprint_starting_info.geode_robot_ore_cost, blueprint_current_state.ore_robots, blueprint_current_state.ore_count);
        let minutes_to_geode_robot = max(minutes_to_obsidian, minutes_to_ore);
        if minutes_remaining > minutes_to_geode_robot {
            let mut asdf = blueprint_current_state.clone();
            asdf.iterate_many(minutes_to_geode_robot);
            asdf.obsidian_count = asdf.obsidian_count - blueprint_starting_info.geode_robot_obsidian_cost;
            asdf.ore_count = asdf.ore_count - blueprint_starting_info.geode_robot_ore_cost;
            asdf.geode_robot = asdf.geode_robot + 1;
            let result = run_blueprint_scenario(blueprint_starting_info, asdf, minutes_remaining - minutes_to_geode_robot);
            results.push(result);
        }
    }

    //Build an Obsidian Robot
    if blueprint_current_state.clay_robots > 0 {
        let minutes_to_clay = calculate_minutes_to_robot(blueprint_starting_info.obsidian_robot_clay_cost, blueprint_current_state.clay_robots, blueprint_current_state.clay_count);
        let minutes_to_ore = calculate_minutes_to_robot(blueprint_starting_info.obsidian_robot_ore_cost, blueprint_current_state.ore_robots, blueprint_current_state.ore_count);
        let minutes_to_obsidian_robot = max(minutes_to_clay, minutes_to_ore);
        if minutes_remaining > minutes_to_obsidian_robot {
            let mut asdf = blueprint_current_state.clone();
            asdf.iterate_many(minutes_to_obsidian_robot);
            asdf.clay_count = asdf.clay_count - blueprint_starting_info.obsidian_robot_clay_cost;
            asdf.ore_count = asdf.ore_count - blueprint_starting_info.obsidian_robot_ore_cost;
            asdf.obsidian_robots = asdf.obsidian_robots + 1;
            let result = run_blueprint_scenario(blueprint_starting_info, asdf, minutes_remaining - minutes_to_obsidian_robot);
            results.push(result);
        }
    }

    //Build a Clay Robot
    if blueprint_starting_info.obsidian_robot_clay_cost >= blueprint_current_state.clay_count {
        let minutes_to_clay_robot = calculate_minutes_to_robot(blueprint_starting_info.clay_robot_ore_cost, blueprint_current_state.ore_robots, blueprint_current_state.ore_count);
        if minutes_remaining > minutes_to_clay_robot {
            let mut asdf = blueprint_current_state.clone();
            asdf.iterate_many(minutes_to_clay_robot);
            asdf.ore_count = asdf.ore_count - blueprint_starting_info.clay_robot_ore_cost;
            asdf.clay_robots = asdf.clay_robots + 1;
            let result = run_blueprint_scenario(blueprint_starting_info, asdf, minutes_remaining - minutes_to_clay_robot);
            results.push(result);
        }
    }

    //Build an Ore Robot
    if blueprint_current_state.ore_robots <= blueprint_starting_info.max_ore_robots {
        let minutes_to_ore_robot = calculate_minutes_to_robot(blueprint_starting_info.ore_robot_ore_cost, blueprint_current_state.ore_robots, blueprint_current_state.ore_count);
        if minutes_remaining > minutes_to_ore_robot {
            let mut asdf = blueprint_current_state.clone();
            asdf.iterate_many(minutes_to_ore_robot);
            asdf.ore_count = asdf.ore_count - blueprint_starting_info.ore_robot_ore_cost;
            asdf.ore_robots = asdf.ore_robots + 1;
            let result = run_blueprint_scenario(blueprint_starting_info, asdf, minutes_remaining - minutes_to_ore_robot);
            results.push(result);
        }
    }

    {
        let mut asdf = blueprint_current_state.clone();
        asdf.iterate_many(minutes_remaining);
        results.push(asdf.geode_count);
    }

    return results.iter().max().unwrap().clone();
}

fn puzzle1(input_array: &[&str]) -> usize {
    let mut sum = 0;
    for input_string in input_array {
        let blueprint_starting_info = parse_blueprint_starting_info(input_string);
        let blueprint_current_state = BlueprintCurrentState::init_state();
        let blueprint_result = run_blueprint_scenario(&blueprint_starting_info, blueprint_current_state, 24);
        println!("print id: {} result {}", blueprint_starting_info.id_number, blueprint_result);
        sum = sum + (blueprint_result * blueprint_starting_info.id_number);
    }
    return sum;
}

fn puzzle2(input_array: &[&str]) -> usize {
    let mut product = 1;
    for input_string in input_array {
        let blueprint_starting_info = parse_blueprint_starting_info(input_string);
        let blueprint_current_state = BlueprintCurrentState::init_state();
        let blueprint_result = run_blueprint_scenario(&blueprint_starting_info, blueprint_current_state, 32);
        println!("print id: {} result {}", blueprint_starting_info.id_number, blueprint_result);
        product = product * blueprint_result;
    }
    return product;
}

#[cfg(test)]
mod tests {
    use crate::day19::{puzzle1, puzzle2};
    use crate::utils::read_file_strings;

    const TEST_DATA: [&str; 2] = [
        "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.",
        "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."
    ];

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(&TEST_DATA);
        assert_eq!(return_value, 33);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day19.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle1(&data);
        assert_eq!(return_value, 1725);
    }


    #[test]
    #[ignore]
    fn test_puzzle2() {
        let return_value = puzzle2(&TEST_DATA);
        assert_eq!(return_value, 3472);
    }

    #[test]
    #[ignore]
    fn test_puzzle2_realdata() {
        let mut data = Vec::new();
        let vec = read_file_strings("../data/Day19.txt");
        vec.iter()
            .for_each(|string| data.push(string.as_str().trim()));

        let return_value = puzzle2(&data[..3]);
        assert_eq!(return_value, 15510);
    }
}