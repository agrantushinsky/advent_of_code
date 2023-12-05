#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <optional>
#include <unordered_map>
#include <functional>
#include <set>
#include <stack>

struct range {
    unsigned long dst;
    unsigned long src;
    unsigned long len;
};

struct mapping {
    std::string map_name;
    std::vector<range> ranges;
};

struct seed_map {
    std::set<unsigned long> seeds;
    std::vector<mapping> mappings;
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

std::vector<unsigned long> split_numbers(const std::string& str) {
    std::vector<unsigned long> numbers;
    const auto& splits = split_string(str, " ", 1);
    for(const auto& split : splits) {
        if(split.empty()) 
            continue;

        numbers.push_back(std::stoul(split));
    }

    return numbers;
}

std::set<unsigned long> convert_string_to_set(const std::string& str) {
    std::set<unsigned long> card_set{};
    const auto& numbers = split_numbers(str);
    for(const auto number : numbers) {
        card_set.insert(number);
    }

    return card_set;
}

seed_map populate_map(const std::vector<std::string>& lines) {
    seed_map seed_map{};
    mapping curr_mapping{};

    for (int i = 0; i < lines.size(); i++) {
        const auto& line = lines[i];
        if(line.empty()) {
            continue;
        }

        // Initial seeds
        if(i == 0) {
            size_t seed_start = line.find(":");
            std::string seeds = line.substr(seed_start + 1);
            seed_map.seeds = convert_string_to_set(seeds);
            continue;
        }

        size_t colon_pos = line.find(":");
        if(colon_pos != std::string::npos) {
            if(!curr_mapping.map_name.empty()) {
                seed_map.mappings.push_back(curr_mapping);
                curr_mapping = {};
            }
            curr_mapping.map_name = line.substr(0, colon_pos);
            std::cout << curr_mapping.map_name << "\n";
            continue;
        }

        const auto& ranges = split_numbers(line);
        range range{};
        range.dst = ranges[0];
        range.src = ranges[1];
        range.len = ranges[2];
        curr_mapping.ranges.push_back(range);
    }
    return seed_map;
}

typedef std::unordered_map<unsigned long, std::stack<unsigned long>> seed_mappings;

void find_mappings(const seed_map& seed_map, seed_mappings& mappings, int depth) {
    for(const auto& map : seed_map.mappings[depth].ranges) {
    }
}

void find_mappings(const seed_map& seed_map, seed_mappings& mappings) {
    find_mappings(seed_map, mappings, 0);
}

int solve_part_1(const seed_map& seed_map) {
    seed_mappings mappings{};
    find_mappings(seed_map, mappings);


}

int main() {
    std::string temp;
    std::vector<std::string> lines;
    while (std::getline(std::cin, temp) && temp != "d") {
        lines.push_back(temp);
    }

    const auto& seed_map = populate_map(lines);
    std::cout << solve_part_1(seed_map) << std::endl;

    return 0;
}