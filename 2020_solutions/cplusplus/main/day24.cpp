#include "day24.hpp"
#include <set>
#include <map>
#include <array>

namespace Day24 {

    enum class HexDirection {
        EAST,
        WEST,
        NORTH_EAST,
        NORTH_WEST,
        SOUTH_WEST,
        SOUTH_EAST

    };

    std::array<HexDirection, 6> getAllHexDirections() {
        return std::array{HexDirection::EAST,
                          HexDirection::WEST,
                          HexDirection::NORTH_EAST,
                          HexDirection::NORTH_WEST,
                          HexDirection::SOUTH_WEST,
                          HexDirection::SOUTH_EAST};
    }

    struct Coordinate {
        int64_t x;
        int64_t y;

        bool operator<(const Coordinate &rhs) const {
            if (x < rhs.x)
                return true;
            if (rhs.x < x)
                return false;
            return y < rhs.y;
        }

        bool operator>(const Coordinate &rhs) const {
            return rhs < *this;
        }

        bool operator<=(const Coordinate &rhs) const {
            return !(rhs < *this);
        }

        bool operator>=(const Coordinate &rhs) const {
            return !(*this < rhs);
        }

        bool operator==(const Coordinate &rhs) const {
            return x == rhs.x &&
                   y == rhs.y;
        }

        bool operator!=(const Coordinate &rhs) const {
            return !(rhs == *this);
        }

        Coordinate applyDirection(HexDirection hexDirection) const {
            switch (hexDirection) {
                case HexDirection::EAST:
                    return Coordinate {x + 1, y - 1};
                case HexDirection::WEST:
                    return Coordinate {x - 1, y + 1};
                case HexDirection::NORTH_EAST:
                    return Coordinate {x, y - 1};
                case HexDirection::NORTH_WEST:
                    return Coordinate {x - 1, y};
                case HexDirection::SOUTH_EAST:
                    return Coordinate {x + 1, y};
                case HexDirection::SOUTH_WEST:
                    return Coordinate {x, y + 1};
                default:
                    return Coordinate{x, y};
            }
        }

    };

    Coordinate createCoordinate(const std::string &string) {
        auto coordinate = Coordinate{0, 0};

        for (auto matchP = string.cbegin();
             matchP != string.cend();
             matchP++) {
            if (*matchP == 'e') {
                coordinate = coordinate.applyDirection(HexDirection::EAST);
            } else if (*matchP == 'w') {
                coordinate = coordinate.applyDirection(HexDirection::WEST);
            } else if (*matchP == 's') {
                matchP++;
                if (*matchP == 'e') {
                    coordinate = coordinate.applyDirection(HexDirection::SOUTH_EAST);
                } else if (*matchP == 'w') {
                    coordinate = coordinate.applyDirection(HexDirection::SOUTH_WEST);
                }
            } else if (*matchP == 'n') {
                matchP++;
                if (*matchP == 'e') {
                    coordinate = coordinate.applyDirection(HexDirection::NORTH_EAST);
                } else if (*matchP == 'w') {
                    coordinate = coordinate.applyDirection(HexDirection::NORTH_WEST);
                }
            }
        }
        return coordinate;
    }

    std::set<Coordinate> createBlackCoordinates(const std::vector<std::string> &input) {
        std::set<Coordinate> tiles;
        for (auto strP = input.cbegin();
             strP != input.cend();
             strP++) {

            const auto coordinate = createCoordinate(*strP);

            auto values = tiles.find(coordinate);
            if (values == tiles.end()) {
                tiles.insert(coordinate);
            } else {
                tiles.erase(coordinate);
            }

        }
        return tiles;
    }

    uint64_t puzzle1(const std::vector<std::string> &input) {

        const auto blackTiles = createBlackCoordinates(input);

        return blackTiles.size();
    }

    uint64_t puzzle2(const std::vector<std::string> &input) {
        auto blackTiles = createBlackCoordinates(input);

        for (uint32_t count = 0; count < 100; count++) {
            std::map<Coordinate, uint32_t> blackAdjacentCount;

            for (auto &blackTile: blackTiles) {
                if (blackAdjacentCount.find(blackTile) == blackAdjacentCount.end()) {
                    blackAdjacentCount[blackTile] = 0;
                }
                for (auto hexDirection: getAllHexDirections()) {
                    const auto adjacentTile = blackTile.applyDirection(hexDirection);
                    blackAdjacentCount[adjacentTile]++;
                }
            }
            std::set<Coordinate> newCoordinates;

            for (auto &keyValue: blackAdjacentCount) {
                if (blackTiles.find(keyValue.first) == blackTiles.end()) {
                    //White
                    if (keyValue.second == 2) {
                        newCoordinates.insert(keyValue.first);
                    }
                } else {
                    //Black
                    if (keyValue.second > 0 && keyValue.second < 3) {
                        newCoordinates.insert(keyValue.first);
                    }
                }
            }

            blackTiles = newCoordinates;

        }

        return blackTiles.size();
    }
}

