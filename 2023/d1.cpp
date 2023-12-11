#include <iostream>
#include <string>
#include <vector>
#include <array>
#include <algorithm>

struct search_term {
    std::string text;
    int value;
};

int main() {
    std::string temp;
    std::vector<std::string> lines;

    while(std::getline(std::cin, temp) && !temp.empty()) {
        lines.push_back(temp);
    }

    // I have a feeling this is cursed...
    const std::array<search_term, 18> search_terms {{ 
        { "1", 1 }, { "2", 2 }, { "3", 3 }, { "4", 4 }, { "5", 5 }, { "6", 6 }, { "7", 7 }, { "8", 8 }, { "9", 9 },
        { "one", 1 }, { "two", 2 }, { "three", 3 }, { "four", 4 }, { "five", 5 }, { "six", 6 }, { "seven", 7 }, { "eight", 8 }, { "nine", 9 }
    }};

    auto find_next_search_term = [search_terms](const std::string& str, size_t start) -> std::pair<const search_term, size_t> {
        std::pair best_match = { search_term{}, std::string::npos };
        for(const auto& term : search_terms) {
            size_t pos = str.find(term.text, start);
            if(pos != std::string::npos) {
                if(pos < best_match.second) {
                    best_match.first = term;
                    best_match.second = pos;
                }
            }
        }
        return best_match;
    };

    int sum = 0;
    search_term first = {}, second = {};
    for(const auto& line : lines) {
        first = second = {};
        size_t current_pos = 0;
        while(true) {
            if(current_pos == line.length()) {
                if(second.value == 0) {
                    second = first;
                }
                break;
            }

            const auto& [match, pos] = find_next_search_term(line, current_pos);

            if(pos == std::string::npos) {
                if(second.value == 0) {
                    second = first;
                }
                break;
            }

            if(first.value == 0) {
                first = match;
            } else {
                second = match;
            }

            current_pos = pos + 1;
        }

        
        // uhhh, this is not cool.
        std::string both = std::to_string(first.value) + std::to_string(second.value);
        sum += atoi(both.c_str());

        std::cout << line << "\n" << "first: " << first.text << " second: " << second.text << "\n\n";
    }
    std::cout << sum << std::endl;
    
    return 0;
}