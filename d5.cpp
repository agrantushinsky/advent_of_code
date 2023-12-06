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
    std::vector<unsigned long> seeds;
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
            seed_map.seeds = split_numbers(seeds);
            continue;
        }

        size_t colon_pos = line.find(":");
        if(colon_pos != std::string::npos || line == lines.back()) {
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

void find_destinations(const std::vector<mapping>& mappings, std::vector<unsigned long>& destinations, int depth = 0) {
    unsigned long source = destinations.back();
    unsigned long destination = source;
    for(const auto& mapping : mappings[depth].ranges) {
        if(source >= mapping.src && source <= mapping.src + mapping.len - 1) {
            destination = source - mapping.src + mapping.dst;
            break;
        }
    }

    destinations.push_back(destination);
    if(depth < mappings.size() - 1) {
        find_destinations(mappings, destinations, depth + 1);
    }
}

int solve_part_1(const seed_map& seed_map) {
    std::unordered_map<unsigned long, std::vector<unsigned long>> seed_destinations{};
    for(const auto seed : seed_map.seeds) {
        seed_destinations[seed].push_back(seed);
        find_destinations(seed_map.mappings, seed_destinations[seed]);
    }

    unsigned long min = ULONG_MAX;
    for(auto& dest : seed_destinations) {
        if(dest.second.back() < min) {
            min = dest.second.back();
        }
        for(const auto num : dest.second) {
            std::cout << num << " ";
        }
        std::cout << "\n";
    }
    return min;
}

int solve_part_2(const seed_map& seed_map) {
    unsigned long min = ULONG_MAX;
    for(int i = 0; i < seed_map.seeds.size(); i += 2) {
        auto base_seed = seed_map.seeds[i];
        for(int j = 0; j < seed_map.seeds[i + 1]; j++) {
            const auto seed = base_seed + j;
            std::unordered_map<unsigned long, std::vector<unsigned long>> seed_destinations{};
            seed_destinations[seed].push_back(seed);
            find_destinations(seed_map.mappings, seed_destinations[seed]);
            for(auto& dest : seed_destinations) {
                if(dest.second.back() < min) {
                    min = dest.second.back();
                }
            }
        }
        std::cout << "searched " << seed_map.seeds[i + 1] << " seeds. (" << i / 2 + 1 << "/" << seed_map.seeds.size() / 2 << ")\n";
    }

    return min;
}

int main() {
    std::string temp;
    std::vector<std::string> lines;
    while (std::getline(std::cin, temp) && temp != "d") {
        lines.push_back(temp);
    }

    const auto& seed_map = populate_map(lines);
    //std::cout << solve_part_1(seed_map) << std::endl;
    std::cout << solve_part_2(seed_map) << std::endl;

    return 0;
}