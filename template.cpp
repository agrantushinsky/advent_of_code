#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <optional>
#include <unordered_map>
#include <functional>
#include <set>

int solve(const std::vector<std::string>& lines) {
    for(const auto& line : lines) {

    }
    
	return 0;
}

int main() {
	std::string temp;
    std::vector<std::string> lines;
    while(std::getline(std::cin, temp) && !temp.empty()) {
        lines.push_back(temp);
    }

	std::cout << solve(lines) << std::endl;

    return 0;
}