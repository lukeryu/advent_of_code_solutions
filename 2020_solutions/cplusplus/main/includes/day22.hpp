#ifndef ADVENT_OF_CODE_2020_DAY22_HPP
#define ADVENT_OF_CODE_2020_DAY22_HPP

#include <deque>

namespace Day22 {
    enum class PlayerWinner {
        PLAYER1,
        PLAYER2
    };

    struct Puzzle2Result {
        PlayerWinner player;
        size_t count;
    };


    size_t puzzle1(std::deque<size_t> player1, std::deque<size_t> player2);
    Puzzle2Result puzzle2(std::deque<size_t> player1, std::deque<size_t> player2, unsigned int game_number);
}

#endif //ADVENT_OF_CODE_2020_DAY22_HPP