#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <optional>
#include <unordered_map>
#include <functional>
#include <set>

inline bool isSymbol(char c) { 
    return 
        ((c >= 58 && c <= 64) || (c >= 33 && c <= 47)) && c != '.'; 
}
inline bool isDigit(char c) { return c >= '0' && c <= '9'; }

std::optional<std::pair<int, int>> hasAdjacentSymbol(const std::vector<std::string>& lines, int x, int y, std::function<bool(char)> isSymbolCheck) {
    for(int i = x - 1; i <= x + 1; i++) {
        if(i < 0 || i >= lines[0].length()) {
            continue;
        }

        for(int j = y - 1; j <= y + 1; j++) {
            if(j < 0 || j >= lines.size()) {
                continue;
            }
            // Don't check the current element
            if(i == x && j == y) {
                continue;
            }

            if(isSymbolCheck(lines[j][i])) {
                return std::make_optional(std::make_pair(i, j));
            }
        }
    }
    return std::nullopt;
}


int solve_part_1(const std::vector<std::string>& lines) {
    int sum = 0;
    for(int i = 0; i < lines.size(); i++) {
        const auto& line = lines[i];
        size_t digit_start = std::string::npos;
        bool hasAdjacent = false;

        for(int j = 0; j < line.length(); j++) {
            const auto& c = line[j];
            bool is_c_digit = isDigit(c);

            if(is_c_digit) {
                if(digit_start == std::string::npos) {
                    digit_start = j;
                }

                if(hasAdjacentSymbol(lines, j, i, isSymbol).has_value()) {
                    hasAdjacent = true;
                }
            } 
            if(!is_c_digit || j == line.length() - 1) {
                if(hasAdjacent && digit_start != std::string::npos) {
                    int part_number = atoi(line.substr(digit_start, j).c_str());
                    sum += part_number;
                } 

                digit_start = std::string::npos;
                hasAdjacent = false;
            }
        }
    }
    return sum;
}

long solve_part_2(const std::vector<std::string>& lines) {
    long sum = 0;
    std::unordered_map<int, std::vector<int>> gears{};
    for(int i = 0; i < lines.size(); i++) {
        const auto& line = lines[i];
        size_t digit_start = std::string::npos;
        bool hasAdjacent = false;

        std::set<int> part_gears{};
        for(int j = 0; j < line.length(); j++) {
            const auto& c = line[j];
            bool is_c_digit = isDigit(c);

            if(is_c_digit) {
                if(digit_start == std::string::npos) {
                    digit_start = j;
                }

                auto symbol = hasAdjacentSymbol(lines, j, i, [](char c) { return c == '*'; });
                if(symbol.has_value()) {
                    int gear_id = symbol.value().first + symbol.value().second * line.length();
                    part_gears.insert(gear_id);

                    hasAdjacent = true;
                }
            } 
            if(!is_c_digit || j == line.length() - 1) {
                if(hasAdjacent && digit_start != std::string::npos) {
                    int part_number = atoi(line.substr(digit_start, j).c_str());
                    
                    for(const auto& gear : part_gears) {
                        gears[gear].push_back(part_number);
                    }
                    part_gears = {};
                } 

                digit_start = std::string::npos;
                hasAdjacent = false;
            }
        }
    }

    for(const auto& [gear_id, part_numbers] : gears) {
        if(part_numbers.size() <= 1) {
            continue;
        }
        std::cout << "\n\nid: " << gear_id << "\n";
        long mult = 1;
        for(const int part_number : part_numbers)  {
            std::cout << part_number << " ";
            mult *= part_number;
        }
        sum += mult;
    }
    std::cout << std::endl;

    return sum;
}

int main() {
    std::string temp;
    std::vector<std::string> lines;
    while(std::getline(std::cin, temp) && !temp.empty()) {
        lines.emplace_back(temp);
    }

    std::cout << solve_part_1(lines) << std::endl;
    std::cout << solve_part_2(lines) << std::endl;

    return 0;
}