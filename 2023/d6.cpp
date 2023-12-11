
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <optional>
#include <unordered_map>
#include <functional>
#include <set>
#include <cmath>

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

/*
t=time
b=button time
d=record distance

b*(t-b) > d
bt - b^2 > d

bt - b^2 - d = 0

-b^2 + bt - d=0

-(b^2 - bt + d)=0

ciel 
then
floor
*/

unsigned long long get_race_solutions(unsigned long long time, unsigned long long distance) {
    // ax^2 + bx + c
    //-b^2 + bt - d=0

    // last bug was here: c = -distance. distance is unsigned.
    long double a = -1, b = time, c = -(long double)distance;

    long double discriminant = b*b - 4*a*c;
    if(discriminant > 0) {
        long double x1 = (-b + std::sqrt(discriminant)) / (2*a);
        long double x2 = (-b - std::sqrt(discriminant)) / (2*a);
        //std::cout << "time " << time << ", distance " << distance << " has solutions " << x1 << " " << x2 << " " << discriminant << std::endl;

        unsigned long long x1_ceil = std::ceil(x1) == x1 ? x1 + 1 : std::ceil(x1);
        unsigned long long x2_floor = std::floor(x2) == x2 ? x2 - 1 : std::floor(x2);

        return x2_floor - x1_ceil + 1;
    }
    return 0;
}

int solve_part_1(const std::vector<std::string> &lines) {
    const std::string& time_line = lines[0];
    const std::string& distance_line = lines[1];

    const auto& times = split_numbers(time_line.substr(time_line.find(":") + 1));
    const auto& distances = split_numbers(distance_line.substr(distance_line.find(":") + 1));

    // Lets just assume there will always be at least one solution (we multiply later)
    int solutions = 1;
    for(int i = 0; i < times.size(); i++) {
        const int time = times[i];
        const int distance = distances[i];

        solutions *= get_race_solutions(time, distance);
    }

    return solutions;
}

unsigned long long solve_part_2(const std::vector<std::string>& lines) {
    std::string time_line = lines[0].substr(lines[0].find(":") + 1);
    std::string distance_line = lines[1].substr(lines[1].find(":") + 1);

    time_line.erase(std::remove(time_line.begin(), time_line.end(), ' '), time_line.end());
    distance_line.erase(std::remove(distance_line.begin(), distance_line.end(), ' '), distance_line.end());

    auto time = std::stoull(time_line.c_str());
    auto distance = std::stoull(distance_line.c_str());

    return get_race_solutions(time, distance);
}

int main() {
    std::string temp;
    std::vector<std::string> lines;
    while (std::getline(std::cin, temp) && !temp.empty()) {
        lines.push_back(temp);
    }

    //std::cout << solve_part_1(lines) << std::endl;
    std::cout << solve_part_2(lines) << std::endl;

    return 0;
}