enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

enum Action {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    LEFT,
    RIGHT,
    FORWARD,
}

struct Ship {
    direction: Direction,
    ship_x: i32,
    ship_y: i32,
    waypoint_x: i32,
    waypoint_y: i32,
}

impl Ship {
    fn new() -> Self {
        return Self {
            direction: Direction::EAST,
            ship_x: 0,
            ship_y: 0,
            waypoint_x: 10,
            waypoint_y: 1,
        };
    }

    fn distance_from_origin(&self) -> u32 {
        return (self.ship_x.abs() + self.ship_y.abs()) as u32;
    }

    fn add_ship_x(&mut self, amount: i32) {
        self.ship_x = self.ship_x + amount;
    }

    fn add_ship_y(&mut self, amount: i32) {
        self.ship_y = self.ship_y + amount;
    }

    fn add_waypoint_x(&mut self, amount: i32) {
        self.waypoint_x = self.waypoint_x + amount;
    }

    fn add_waypoint_y(&mut self, amount: i32) {
        self.waypoint_y = self.waypoint_y + amount;
    }

    fn add_ship_forward(&mut self, amount: i32) {
        match self.direction {
            Direction::EAST => self.add_ship_x(amount),
            Direction::NORTH => self.add_ship_y(amount),
            Direction::WEST => self.add_ship_x(-1 * amount),
            Direction::SOUTH => self.add_ship_y(-1 * amount),
        }
    }

    fn add_waypoint_forward(&mut self, amount: i32) {
        for times in 0..amount {
            self.add_ship_x(self.waypoint_x);
            self.add_ship_y(self.waypoint_y);
        }

    }

    fn ship_left(&mut self) {
        self.direction = match self.direction {
            Direction::EAST => Direction::NORTH,
            Direction::NORTH => Direction::WEST,
            Direction::WEST => Direction::SOUTH,
            Direction::SOUTH => Direction::EAST,
        }
    }

    fn ship_right(&mut self) {
        self.direction = match self.direction {
            Direction::EAST => Direction::SOUTH,
            Direction::NORTH => Direction::EAST,
            Direction::WEST => Direction::NORTH,
            Direction::SOUTH => Direction::WEST,
        }
    }

    fn waypoint_left(&mut self) {
        let new_x = -1 * self.waypoint_y;
        let new_y = self.waypoint_x;

        self.waypoint_x = new_x;
        self.waypoint_y = new_y;
    }

    fn waypoint_right(&mut self) {
        let new_x = self.waypoint_y;
        let new_y = -1 * self.waypoint_x;

        self.waypoint_x = new_x;
        self.waypoint_y = new_y;
    }
}

struct ShipInstruction {
    action: Action,
    quantity: i32,
}

impl ShipInstruction {
    fn new(string: &String) -> Self {
        let first_char = string.chars().next().unwrap();
        let quantity = &string[1..].parse::<i32>().unwrap();

        let action = match first_char {
            'N' => Action::NORTH,
            'S' => Action::SOUTH,
            'E' => Action::EAST,
            'W' => Action::WEST,
            'L' => Action::LEFT,
            'R' => Action::RIGHT,
            'F' => Action::FORWARD,
            _ => Action::FORWARD
        };

        return Self {
            action,
            quantity: *quantity,
        };
    }

    fn apply_ship(&self, ship: &mut Ship) {
        match self.action {
            Action::NORTH => ship.add_ship_y(self.quantity),
            Action::SOUTH => ship.add_ship_y(-1 * self.quantity),
            Action::EAST => ship.add_ship_x(self.quantity),
            Action::WEST => ship.add_ship_x(-1 * self.quantity),
            Action::LEFT => {
                let turns = self.quantity / 90;
                for turn in 0..turns {
                    ship.ship_left()
                }
            },
            Action::RIGHT => {
                let turns = self.quantity / 90;
                for turn in 0..turns {
                    ship.ship_right()
                }
            },
            Action::FORWARD => ship.add_ship_forward(self.quantity)
        }
    }

    fn apply_waypoint(&self, ship: &mut Ship) {
        match self.action {
            Action::NORTH => ship.add_waypoint_y(self.quantity),
            Action::SOUTH => ship.add_waypoint_y(-1 * self.quantity),
            Action::EAST => ship.add_waypoint_x(self.quantity),
            Action::WEST => ship.add_waypoint_x(-1 * self.quantity),
            Action::LEFT => {
                let turns = self.quantity / 90;
                for turn in 0..turns {
                    ship.waypoint_left()
                }
            },
            Action::RIGHT => {
                let turns = self.quantity / 90;
                for turn in 0..turns {
                    ship.waypoint_right()
                }
            },
            Action::FORWARD => ship.add_waypoint_forward(self.quantity)
        }
    }
}

fn puzzle1(lines: Vec<String>) -> u32 {
    let mut ship = Ship::new();

    lines.iter()
        .map(ShipInstruction::new)
        .for_each(|instruction| instruction.apply_ship(&mut ship));

    return ship.distance_from_origin();
}

fn puzzle2(lines: Vec<String>) -> u32 {

    let mut ship = Ship::new();

    lines.iter()
        .map(ShipInstruction::new)
        .for_each(|instruction| instruction.apply_waypoint(&mut ship));

    return ship.distance_from_origin();

}

#[cfg(test)]
mod tests {
    use crate::day12::puzzle1;
    use crate::day12::puzzle2;
    use crate::utils::read_lines;

    #[test]
    fn test_puzzle_1() {
        let strs = vec!["F10",
                        "N3",
                        "F7",
                        "R90",
                        "F11"];

        let strings = strs.into_iter()
            .map(String::from)
            .collect();

        let result = puzzle1(strings);
        assert_eq!(result, 25);
    }

    #[test]
    fn test_puzzle_1_2() {
        let strs = read_lines("data/Day12.txt").unwrap();

        let result = puzzle1(strs);
        assert_eq!(result, 381);
    }

    #[test]
    fn test_puzzle_2() {
        let strs = vec!["F10",
                        "N3",
                        "F7",
                        "R90",
                        "F11"];

        let strings = strs.into_iter()
            .map(String::from)
            .collect();

        let result = puzzle2(strings);
        assert_eq!(result, 286);
    }

    #[test]
    fn test_puzzle_2_2() {
        let strings = read_lines("data/Day12.txt").unwrap();

        let result = puzzle2(strings);
        assert_eq!(result, 28591);
    }
}