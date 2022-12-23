//
// Created by luker on 6/5/2021.
//
#include "day22.hpp"
#include <algorithm>
#include <numeric>
#include <optional>

namespace Day22 {

    template <typename T>
    std::optional<T> pop_front(std::deque<T> &queue) {
        if(queue.empty()) {
            return {};
        }
        const auto value = queue.front();
        queue.pop_front();

        return value;
    }

    struct GameState {
        std::deque<size_t> player1_state;
        std::deque<size_t> player2_state;

        bool operator==(const GameState &rhs) const {
            return player1_state == rhs.player1_state &&
                   player2_state == rhs.player2_state;
        }

        bool operator!=(const GameState &rhs) const {
            return !(rhs == *this);
        }
    };

    size_t count_score(std::deque<size_t> &player_hand) {
        std::vector<size_t> mapped_values;
        mapped_values.resize(player_hand.size());

        size_t index = 0;
        for(auto it = player_hand.rbegin(); it != player_hand.rend(); it++) {
            size_t indexusize = index + 1;
            mapped_values[index] = indexusize * *it;
            index++;
        }

        return std::accumulate(mapped_values.begin(), mapped_values.end(), 0);
    }

    size_t puzzle1(std::deque<size_t> player1, std::deque<size_t> player2) {
        while (!player1.empty() && !player2.empty()) {
            const size_t player1_value = pop_front(player1).value();
            const size_t player2_value = pop_front(player2).value();

            if (player1_value > player2_value) {
                player1.push_back(player1_value);
                player1.push_back(player2_value);
            } else {
                player2.push_back(player2_value);
                player2.push_back(player1_value);
            }
        }

        if (player1.empty()) {
            return count_score(player2);
        } else {
            return count_score(player1);
        }
    }

    Puzzle2Result puzzle2(std::deque<size_t> player1, std::deque<size_t> player2, unsigned int game_number) {
        uint32_t round = 1;
        std::vector<GameState> previous_states;

        GameState current_state;
        current_state.player1_state = std::deque(player1);
        current_state.player2_state = std::deque(player2);

        while (!player1.empty() && !player2.empty()
               && !std::any_of(previous_states.begin(), previous_states.end(), [&current_state](GameState &gameState){return gameState == current_state;})) {

            previous_states.push_back(current_state);
            const auto player1_value = pop_front(player1).value();
            const auto player2_value = pop_front(player2).value();

            PlayerWinner winner;
            if (player1_value <= player1.size() && player2_value <= player2.size()) {
                auto player1_clone = std::deque(player1);
                auto player2_clone = std::deque(player2);
                winner = puzzle2(player1_clone, player2_clone, game_number + 1).player;
            } else if (player1_value > player2_value) {
                winner = PlayerWinner::PLAYER1;
            } else {
                winner = PlayerWinner::PLAYER2;
            }
            switch (winner) {
                    case PlayerWinner::PLAYER1: {
                        player1.push_back(player1_value);
                        player1.push_back(player2_value);
                        break;
                    }
                    case PlayerWinner::PLAYER2: {
                        player2.push_back(player2_value);
                        player2.push_back(player1_value);
                        break;
                    }
            }
            current_state = GameState();
            current_state.player1_state = std::deque(player1);
            current_state.player2_state = std::deque(player2);
            round += 1;
        }

        if (player1.empty()) {
            Puzzle2Result result;
            result.player = PlayerWinner::PLAYER2;
            result.count = count_score(player2);
            return result;
        } else {
            Puzzle2Result result;
            result.player = PlayerWinner::PLAYER1;
            result.count = count_score(player1);
            return result;
        }
    }
}