#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <optional>
#include <unordered_map>
#include <functional>
#include <set>
#include <queue>
#include <tuple>
#include <Windows.h>
enum Direction {
    North,
    East,
    South,
    West
};

struct Tile {
    char character;
    std::optional<std::vector<Direction>> connections;
    bool explored;
};

struct Map {
    std::vector<std::vector<Tile>> grid;
    int start_x, start_y;
};

std::optional<std::vector<Direction>> get_connections(char tile) {
    switch(tile) {
        case '|':
            return std::make_optional(std::vector<Direction> { Direction::North, Direction::South});
        case '-':
            return std::make_optional(std::vector<Direction> { Direction::East, Direction::West});
        case 'L':
            return std::make_optional(std::vector<Direction> { Direction::North, Direction::East});
        case 'J':
            return std::make_optional(std::vector<Direction> { Direction::North, Direction::West});
        case '7':
            return std::make_optional(std::vector<Direction> { Direction::South, Direction::West});
        case 'F':
            return std::make_optional(std::vector<Direction> { Direction::South, Direction::East});
        case 'S':
            return std::make_optional(std::vector<Direction> { Direction::South, Direction::East}); //  Direction::North, Direction::South
    }

    return std::nullopt;
}

Direction reverse_direction(Direction dir) {
    switch(dir) {
        case North:
            return South;
        case South:
            return North;
        case East:
            return West;
        case West:
            return East;
    }
}

Tile* get_tile(Map& map, int x, int y) {
    if(x > map.grid[0].size() - 1 || y > map.grid.size() - 1 || x < 0 || y < 0) {
        return nullptr;
    }
    return &map.grid[y][x];
}

struct bfs_node {
    int x;
    int y;
    int steps;
};

int solve(const std::vector<std::string> &lines) {
    Map map{};
    for (int i = 0; i < lines.size(); i++) {
        const auto& line = lines[i];
        std::vector<Tile> row{};
        for(int j = 0; j < line.length(); j++) {
            const auto character = line[j];
            if(character == 'S') {
                map.start_y = i;
                map.start_x = j;
            }

            Tile tile{}; // I have made the 2x2 tile box.
            tile.character = character;
            tile.connections = get_connections(character);
            tile.explored = false; // sanity.

            row.push_back(tile);
        }
        map.grid.push_back(row);
    }

    // https://en.wikipedia.org/wiki/Breadth-first_search
    std::queue<bfs_node> nodes{};
    bfs_node start = { .x = map.start_x, .y = map.start_y };
    nodes.push(start);

    const std::vector<std::tuple<int, int, Direction>> adjacents_offsets = { 
        std::make_tuple(1, 0, East), 
        std::make_tuple(-1, 0, West), 
        std::make_tuple(0, 1, South), 
        std::make_tuple(0, -1, North) 
    };

    int highest_steps = 0;

    while(!nodes.empty()) {
        // Interesting reference.......... maybe i shouldve stayed with rust for day 10
        const bfs_node/*&*/ node = nodes.front();
        nodes.pop();

        auto current = get_tile(map, node.x, node.y);
        current->explored = true;

        const auto& current_con = current->connections;

        for(const auto& [off_x, off_y, off_dir] : adjacents_offsets) {
            // only attempt to move to adjacent tiles with connections from our current tile
            if(std::find(current_con.value().begin(), current_con.value().end(), off_dir) == current_con.value().end()) {
                continue;
            }
            
            auto adj_tile = get_tile(map, node.x + off_x, node.y + off_y);
            if(adj_tile && !adj_tile->explored && adj_tile->connections.has_value()) {
                if(std::find(adj_tile->connections.value().begin(), adj_tile->connections.value().end(), reverse_direction(off_dir)) != adj_tile->connections.value().end()) {
                    bfs_node adj_node = { .x = node.x + off_x, .y = node.y + off_y, .steps = node.steps + 1 };
                    if(adj_node.steps > highest_steps) {
                        highest_steps = adj_node.steps;
                    }
                    nodes.push(adj_node);
                }
            }
        }
    }

    HANDLE std_out = GetStdHandle(STD_OUTPUT_HANDLE);
    for(const auto& row : map.grid) {
        for(const auto& tile : row) {
            if(tile.explored) {
                SetConsoleTextAttribute(std_out, BACKGROUND_BLUE);
            } else {
                SetConsoleTextAttribute(std_out, 0);
            }
            std::cout << tile.character;
        }
        std::cout << "\n";
    }

    return highest_steps;
}

int main() {
    std::string temp;
    std::vector<std::string> lines;
    while (std::getline(std::cin, temp) && !temp.empty()) {
        lines.push_back(temp);
    }

    std::cout << solve(lines) << std::endl;

    return 0;
}
