
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <optional>
#include <unordered_map>
#include <functional>
#include <set>

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

int solve(const std::vector<std::string> &lines) {
    const std::string& time_line = lines[0];
    const std::string& distance_line = lines[1];

    const auto& times = split_numbers(time_line.substr(time_line.find(":") + 1));
    const auto& distances = split_numbers(distance_line.substr(distance_line.find(":") + 1));

    return 0;
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