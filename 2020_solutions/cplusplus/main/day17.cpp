#include "day17.hpp"
#include <set>
#include <map>
//
// Created by luker on 2/22/2021.
//

namespace Day17 {
    struct Point {
        int64_t x;
        int64_t y;
        int64_t z;
        int64_t w;

        bool operator==(const Point &rhs) const {
            return x == rhs.x &&
                   y == rhs.y &&
                   z == rhs.z &&
                   w == rhs.w;
        }

        bool operator!=(const Point &rhs) const {
            return !(rhs == *this);
        }

        bool operator<(const Point &rhs) const {
            if (x < rhs.x)
                return true;
            if (rhs.x < x)
                return false;
            if (y < rhs.y)
                return true;
            if (rhs.y < y)
                return false;
            if (z < rhs.z)
                return true;
            if (rhs.z < z)
                return false;
            return w < rhs.w;
        }

        bool operator>(const Point &rhs) const {
            return rhs < *this;
        }

        bool operator<=(const Point &rhs) const {
            return !(rhs < *this);
        }

        bool operator>=(const Point &rhs) const {
            return !(*this < rhs);
        }
    };

    uint64_t puzzle1(const std::vector<std::string> &input, const uint64_t cycles) {
        std::set<Point> activePoints;

        for (auto rowPtr = input.cbegin(); rowPtr != input.cend(); rowPtr++) {
            for (auto columnPtr = rowPtr->cbegin(); columnPtr != rowPtr->cend(); columnPtr++) {
                if (*columnPtr == '#') {
                    const auto y_index = std::distance(rowPtr, input.cbegin());
                    const auto x_index = std::distance(columnPtr, rowPtr->cbegin());
                    activePoints.insert(Point{x_index, y_index, 0, 0});
                }
            }
        }

        for (uint64_t currentCycle = 0; currentCycle < cycles; currentCycle++) {
            std::map<Point, uint64_t> adjacentActiveCount;

            for (auto &activePoint: activePoints) {
                for (int64_t x_div = -1; x_div <= 1; x_div++) {
                    for (int64_t y_div = -1; y_div <= 1; y_div++) {
                        for (int64_t z_div = -1; z_div <= 1; z_div++) {
                            const Point newPoint{activePoint.x + x_div, activePoint.y + y_div, activePoint.z + z_div,
                                                 0};
                            if (newPoint != activePoint) {
                                adjacentActiveCount[newPoint]++;
                            }
                        }
                    }
                }
            }
            std::set<Point> newPoints;

            for (auto &pair: adjacentActiveCount) {
                if (pair.second == 3) {
                    newPoints.insert(pair.first);
                } else if (pair.second == 2 && activePoints.find(pair.first) != activePoints.end()) {
                    newPoints.insert(pair.first);
                }
            }

            activePoints = newPoints;
        }

        return activePoints.size();
    }

    uint64_t puzzle2(const std::vector<std::string> &input, const uint64_t cycles) {
        std::set<Point> activePoints;

        for (auto rowPtr = input.cbegin(); rowPtr != input.cend(); rowPtr++) {
            for (auto columnPtr = rowPtr->cbegin(); columnPtr != rowPtr->cend(); columnPtr++) {
                if (*columnPtr == '#') {
                    const auto y_index = std::distance(rowPtr, input.cbegin());
                    const auto x_index = std::distance(columnPtr, rowPtr->cbegin());
                    activePoints.insert(Point{x_index, y_index, 0, 0});
                }
            }
        }

        for (uint64_t currentCycle = 0; currentCycle < cycles; currentCycle++) {
            std::map<Point, uint64_t> adjacentActiveCount;

            for (auto &activePoint: activePoints) {
                for (int64_t x_div = -1; x_div <= 1; x_div++) {
                    for (int64_t y_div = -1; y_div <= 1; y_div++) {
                        for (int64_t z_div = -1; z_div <= 1; z_div++) {
                            for (int64_t w_div = -1; w_div <= 1; w_div++) {
                                const Point newPoint{activePoint.x + x_div, activePoint.y + y_div,
                                                     activePoint.z + z_div, activePoint.w + w_div};
                                if (newPoint != activePoint) {
                                    adjacentActiveCount[newPoint]++;
                                }
                            }
                        }
                    }
                }
            }
            std::set<Point> newPoints;

            for (auto &pair: adjacentActiveCount) {
                if (pair.second == 3) {
                    newPoints.insert(pair.first);
                } else if (pair.second == 2 && activePoints.find(pair.first) != activePoints.end()) {
                    newPoints.insert(pair.first);
                }
            }

            activePoints = newPoints;
        }

        return activePoints.size();
    }
}
