#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct BusDeparture {
    time_to_next_bus: u64,
    bus_number: u64,
}
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct BusSchedule {
    index: usize,
    bus_number: u64,
}

const EMPTY_SCHEDULE: BusSchedule = BusSchedule { bus_number: 0, index: 0 };

fn puzzle1(timestamp: u64, bus_schedule: String) -> u64 {
    let next_bus_departure = bus_schedule.split(",")
        .filter(|input| *input != "x")
        .map(|input| (*input).parse::<u64>().unwrap())
        .map(|bus_number| {
            let value = timestamp / bus_number;
            let time_to_next_bus = (bus_number * (value + 1));
            return BusDeparture {
                time_to_next_bus: time_to_next_bus,
                bus_number: bus_number,
            };
        })
        .min().unwrap();

    return (next_bus_departure.time_to_next_bus - timestamp) * next_bus_departure.bus_number;
}

fn puzzle2(bus_schedule: String) -> u64 {
    let buses = bus_schedule.split(",")
        .enumerate()
        .map(|bus_number| {
            if bus_number.1 == "x" {
                return EMPTY_SCHEDULE;
            }
            return BusSchedule {
                index: bus_number.0,
                bus_number: bus_number.1.parse::<u64>().unwrap(),
            };
        }).filter( |schedule| *schedule != EMPTY_SCHEDULE)
        .collect::<Vec<BusSchedule>>();

    let mut solution = 0u64;
    let mut lcd = 1u64;
    for bus in buses.iter() {
        while (solution + (bus.index as u64)) % bus.bus_number != 0 {
            solution += lcd;
        }
        lcd *= bus.bus_number;
    };
    return solution
}

#[cfg(test)]
mod tests {
    use crate::day13::{puzzle1, puzzle2};

    #[test]
    fn test_puzzle_1_1() {
        let result = puzzle1(939, String::from("7,13,x,x,59,x,31,19"));
        assert_eq!(result, 295);
    }

    #[test]
    fn test_puzzle_1_2() {
        let result = puzzle1(1004098, String::from("23,x,x,x,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,509,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,401,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,19"));
        assert_eq!(result, 2406);
    }

    #[test]
    fn test_puzzle_2_1() {
        let result = puzzle2(String::from("7,13,x,x,59,x,31,19"));
        assert_eq!(result, 1068781);

        let result2 = puzzle2(String::from("17,x,13,19"));
        assert_eq!(result2, 3417);

        let result3 = puzzle2(String::from("67,7,59,61"));
        assert_eq!(result3, 754018);

        let result4 = puzzle2(String::from("67,x,7,59,61"));
        assert_eq!(result4, 779210);

        let result5 = puzzle2(String::from("67,7,x,59,61"));
        assert_eq!(result5, 1261476);

        let result6 = puzzle2(String::from("1789,37,47,1889"));
        assert_eq!(result6, 1202161486);
    }

    #[test]
    fn test_puzzle_2_2() {
        let result = puzzle2(String::from("23,x,x,x,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,509,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,401,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,19"));
        assert_eq!(result, 225850756401039);
    }
}