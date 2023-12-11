#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <optional>
#include <unordered_map>
#include <functional>
#include <set>
#include <numeric>

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

//typedef char location_code[3];
typedef std::string location_code;

struct location {
    location_code left, right;
};

struct camel_map {
    std::string directions;
    std::unordered_map<location_code, location> map;
};

camel_map parse_map(const std::vector<std::string> &lines) {
    camel_map map{};
    for(int i = 0; i < lines.size(); i++) {
        const std::string& line = lines[i];
        // directions
        if(i == 0) { map.directions = line; continue; }
        // empty line!
        if(line.empty()) { continue; }

        location_code curr = line.substr(0, 3);
        location loc{};
        loc.left = line.substr(7, 3);
        loc.right = line.substr(12, 3);

        map.map[curr] = loc;
    }
    return map;
}

int solve_part_1(const camel_map& map, const location_code& curr, int steps = 0) {
    // directions[steps % directions.length()]
    if(curr == "ZZZ") {
        return steps;
    }
    const char next_dir = map.directions[steps % (map.directions.length())];
    const location& curr_location = map.map.at(curr);
    const location_code& next = next_dir == 'L' ? curr_location.left : curr_location.right;
    return solve_part_1(map, next, steps + 1);
}

struct part_2_metadata {
    location_code current_location;
    int first_z_steps;
};

unsigned long long solve_part_2(const camel_map& map) {
    std::vector<part_2_metadata> locations{};
    
    // start with all nodes ending with 'A'
    for(const auto&[curr, loc] : map.map) {
        if(curr.ends_with('A')) {
            part_2_metadata data{};
            data.first_z_steps = -1;
            data.current_location = curr;
            locations.push_back(data);
        }
    }

    int steps = 0;

    const auto search_complete = [](part_2_metadata metadata) {
        return metadata.first_z_steps != -1;
    };

    while(!std::all_of(locations.begin(), locations.end(), search_complete)) {
        const bool go_left = map.directions[steps % (map.directions.length())] == 'L';
        for(auto& loc : locations) {
            if(search_complete(loc)) { continue; } // TODO
            if(loc.current_location.ends_with('Z')) {
                if(loc.first_z_steps == -1) {
                    loc.first_z_steps = steps;
                }
            }

            const location& curr_location = map.map.at(loc.current_location);
            loc.current_location = go_left ? curr_location.left : curr_location.right;
        }
        steps++;
    }

    return std::accumulate(locations.begin(), locations.end(), (unsigned long long)1,
        [](unsigned long long a, const part_2_metadata& loc) { return std::lcm(a, loc.first_z_steps); });
}

int main() {
    std::string temp;
    std::vector<std::string> lines;
    while (std::getline(std::cin, temp) && temp != "d") {
        lines.push_back(temp);
    }

    const auto map = parse_map(lines);

    //std::cout << solve_part_1(map, "AAA") << std::endl;
    std::cout << solve_part_2(map) << std::endl;

    return 0;
}