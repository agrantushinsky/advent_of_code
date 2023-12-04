#include <iostream>
#include <string>
#include <array>
#include <vector>
#include <algorithm>
#include <ranges>
#include <string_view>
#include <unordered_map>

struct Cube {
    std::string colour;
    int value;
};

struct Game {
    int id;
    std::vector<std::vector<Cube>> sub_games;
};

std::vector<std::string> split_string(const std::string& str, const char* delimiter, size_t len) {
    std::vector<std::string> splits;

    size_t curr = 0, start = 0;
    while((curr = str.find(delimiter, start, len)) != std::string::npos) {
        splits.push_back(str.substr(start, curr - start));
        start = curr + len;
    }
    splits.push_back(str.substr(start, str.length() - start));

    return splits;
}

// Part #1
int solve(const std::vector<Game>& games, const std::unordered_map<std::string, int>& constraints) {
    int sum = 0;
    for(const auto& game : games) {
        bool passesConstraints = true;
        for(const auto& sub_game : game.sub_games) {
            std::unordered_map<std::string, int> game_values{};

            for(const auto& cube : sub_game) {
                game_values[cube.colour] += cube.value;
            }

            for(const auto& [constraint_colour, constraint_value] : constraints) {
                if(game_values[constraint_colour] > constraint_value) {
                    passesConstraints = false;
                    break;
                }
            }
            if(!passesConstraints) {
                break;
            }
        }

        if(passesConstraints) {
            sum += game.id;
        }
    }
    return sum;
}

// Part #2
int solve(const std::vector<Game>& games) {
    int sum = 0;
    for(const auto& game : games) {
        std::unordered_map<std::string, int> game_values{};

        for(const auto& sub_game : game.sub_games) {
            std::unordered_map<std::string, int> sub_game_values{};

            for(const auto& cube : sub_game) {
                sub_game_values[cube.colour] += cube.value;

                if(sub_game_values[cube.colour] > game_values[cube.colour]) {
                    game_values[cube.colour] = sub_game_values[cube.colour];
                }
            }
        }
        int power = 1;
        for(const auto& [colour, value] : game_values) {
            power *= value;
        }
        std::cout << game.id << " " << power << std::endl;
        sum += power;
    }
    return sum;
}

int main() {
    std::vector<Game> games;

    std::string temp;
    std::vector<std::string> lines;
    while(std::getline(std::cin, temp) && !temp.empty()) {
        lines.emplace_back(temp);
    }

    for(auto& line : lines) {
        Game game{};

        size_t games_start = line.find(":");
        game.id = atoi(line.substr(5, games_start).c_str());
        line.erase(0, games_start + 2);

        const auto& split_games = split_string(line, "; ", 2);
        for(const auto& split_game : split_games) {
            std::vector<Cube> sub_games;

            const auto& cubes = split_string(split_game, ", ", 2);
            for(const auto& cube_data : cubes) {
                Cube cube;
                size_t splitter = cube_data.find(' ');

                cube.colour = cube_data.substr(splitter + 1, cube_data.length() - splitter);
                cube.value = atoi(cube_data.substr(0, splitter).c_str());

                sub_games.push_back(cube);
            }

            game.sub_games.push_back(sub_games);
        }

        games.emplace_back(game);
    }

    //const std::unordered_map<std::string, int> constraints { { "red", 12 }, { "green", 13 }, { "blue", 14 } };
    std::cout << solve(games/*, constraints */) << std::endl;

    return 0;
}