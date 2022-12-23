#include "day12.hpp"
#include <algorithm>
#include <stdlib.h>
//
// Created by luker on 2/2/2021.
//

namespace Day12 {
    enum class Action {
        NORTH,
        SOUTH,
        EAST,
        WEST,
        LEFT,
        RIGHT,
        FORWARD
    };
    enum class Direction {
        NORTH, SOUTH, EAST, WEST
    };

    class Ship {
        Direction direction = Direction::EAST;
        int32_t ship_x = 0;
        int32_t ship_y = 0;
        int32_t waypoint_x = 10;
        int32_t waypoint_y = 1;
    public:

        uint32_t distance_from_origin() const {
            return std::abs(ship_x) + std::abs(ship_y);
        }

        void add_ship_x(int32_t amount) {
            ship_x = ship_x + amount;
        }

        void add_ship_y(int32_t amount) {
            ship_y = ship_y + amount;
        }

        void add_waypoint_x(int32_t amount) {
            waypoint_x = waypoint_x + amount;
        }

        void add_waypoint_y(int32_t amount) {
            waypoint_y = waypoint_y + amount;
        }

        void add_ship_forward(int32_t amount) {
            switch (direction) {
                case Direction::EAST:
                    add_ship_x(amount);
                    return;
                case Direction::NORTH :
                    add_ship_y(amount);
                    return;
                case Direction::WEST :
                    add_ship_x(-1 * amount);
                    return;
                case Direction::SOUTH :
                    add_ship_y(-1 * amount);
                    return;
            }
        }

        void add_waypoint_forward(int32_t amount) {
            for (int32_t times = 0; times < amount; times++) {
                add_ship_x(waypoint_x);
                add_ship_y(waypoint_y);
            }

        }

        void ship_left() {
            switch (direction) {
                case Direction::EAST:
                    direction = Direction::NORTH;
                    return;
                case Direction::NORTH:
                    direction = Direction::WEST;
                    return;
                case Direction::WEST:
                    direction = Direction::SOUTH;
                    return;
                case Direction::SOUTH:
                    direction = Direction::EAST;
                    return;
            }
        }

        void ship_right() {
            switch (direction) {
                case Direction::EAST:
                    direction = Direction::SOUTH;
                    return;
                case Direction::NORTH:
                    direction = Direction::EAST;
                    return;
                case Direction::WEST:
                    direction = Direction::NORTH;
                    return;
                case Direction::SOUTH:
                    direction = Direction::WEST;
                    return;
            }
        }

        void waypoint_left() {
            int32_t new_x = -1 * waypoint_y;
            int32_t new_y = waypoint_x;

            waypoint_x = new_x;
            waypoint_y = new_y;
        }

        void waypoint_right() {
            int32_t new_x = waypoint_y;
            int32_t new_y = -1 * waypoint_x;

            waypoint_x = new_x;
            waypoint_y = new_y;
        }
    };

    class ShipInstruction {
        Action action;
        int32_t quantity;
    public:
        ShipInstruction(const std::string string) {
            switch (string[0]) {
                case 'N':
                    action = Action::NORTH;
                    break;
                case 'S':
                    action = Action::SOUTH;
                    break;
                case 'E':
                    action = Action::EAST;
                    break;
                case 'W':
                    action = Action::WEST;
                    break;
                case 'L':
                    action = Action::LEFT;
                    break;
                case 'R':
                    action = Action::RIGHT;
                    break;
                case 'F':
                    action = Action::FORWARD;
                    break;
                default:
                    action = Action::FORWARD;
            }
            quantity = static_cast<int32_t>(std::stol(string.substr(1)));
        }

        void apply_ship(Ship &ship) {
            const auto turns = quantity / 90;
            switch (action) {
                case Action::NORTH:
                    ship.add_ship_y(quantity);
                    break;
                case Action::SOUTH:
                    ship.add_ship_y(-1 * quantity);
                    break;
                case Action::EAST:
                    ship.add_ship_x(quantity);
                    break;
                case Action::WEST:
                    ship.add_ship_x(-1 * quantity);
                    break;
                case Action::FORWARD:
                    ship.add_ship_forward(quantity);
                    break;
                case Action::LEFT:
                    for (auto turn = 0; turn < turns; turn++) {
                        ship.ship_left();
                    }
                    break;
                case Action::RIGHT:
                    for (auto turn = 0; turn < turns; turn++) {
                        ship.ship_right();
                    }
                    break;
            }
        }

        void apply_waypoint(Ship &ship) {
            const auto turns = quantity / 90;
            switch (action) {
                case Action::NORTH:
                    ship.add_waypoint_y(quantity);
                    break;
                case Action::SOUTH:
                    ship.add_waypoint_y(-1 * quantity);
                    break;
                case Action::EAST:
                    ship.add_waypoint_x(quantity);
                    break;
                case Action::WEST:
                    ship.add_waypoint_x(-1 * quantity);
                    break;
                case Action::FORWARD:
                    ship.add_waypoint_forward(quantity);
                    break;
                case Action::LEFT:
                    for (auto turn = 0; turn < turns; turn++) {
                        ship.waypoint_left();
                    }
                    break;
                case Action::RIGHT:
                    for (auto turn = 0; turn < turns; turn++) {
                        ship.waypoint_right();
                    }
                    break;
            }
        }
    };

    uint32_t puzzle1(const std::vector<std::string> &inputValues) {
        Ship ship;

        std::for_each(inputValues.begin(), inputValues.end(), [&ship](std::string string) {
            ShipInstruction instruction(string);
            instruction.apply_ship(ship);
        });

        return ship.distance_from_origin();
    }

    uint32_t puzzle2(const std::vector<std::string> &inputValues) {
        Ship ship;

        std::for_each(inputValues.begin(), inputValues.end(), [&ship](std::string string) {
            ShipInstruction instruction(string);
            instruction.apply_waypoint(ship);
        });

        return ship.distance_from_origin();
    }
}