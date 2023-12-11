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

struct Card {
    int card_num;
    std::set<int> winning;
    std::set<int> holding;
    int matches;
    size_t instances;
};

std::set<int> convert_string_to_set(const std::string& str) {
    std::set<int> card_set{};
    const auto& splits = split_string(str, " ", 1);
    for(const auto& split : splits) {
        if(split.empty()) 
            continue;

        card_set.insert(atoi(split.c_str()));
    }

    return card_set;
}

int solve_part_1(std::vector<std::string>& lines) {
    std::vector<Card> cards{};
    for(auto& line : lines) {
        static int card_num_index = 1;
        line.erase(0, line.find(":") + 2);

        size_t pipe_pos = line.find("|");

        std::string winning_numbers = line.substr(0, pipe_pos);
        std::string holding_numbers = line.substr(pipe_pos + 2, line.length() - 1);

        Card card{};

        card.winning = convert_string_to_set(winning_numbers);
        card.holding = convert_string_to_set(holding_numbers);
        card.card_num = card_num_index;
        card_num_index++;

        cards.push_back(card);
    }

    int sum = 0;
    // calculate score
    for(const auto& card : cards) {
        int points = 1;
        for(const auto& number : card.holding) {
            if(card.winning.contains(number)) {
                points *= 2;
            }
        }
        //std::cout << points / 2 << "\n";

        sum += points / 2;
    }

	return sum;
}

size_t solve_part_2(std::vector<std::string>& lines) {
    std::vector<Card> cards{};
    for(auto& line : lines) {
        static int card_num_index = 1;
        line.erase(0, line.find(":") + 1);

        size_t pipe_pos = line.find("|");

        std::string winning_numbers = line.substr(0, pipe_pos);
        std::string holding_numbers = line.substr(pipe_pos + 2, line.length() - 1);

        std::cout << winning_numbers << std::endl;
        std::cout << holding_numbers << std::endl;

        Card card{};

        card.winning = convert_string_to_set(winning_numbers);
        card.holding = convert_string_to_set(holding_numbers);
        card.card_num = card_num_index;
        card.instances = 1;
        card_num_index++;

        cards.push_back(card);
    }

    for(auto& card : cards) {
        for(const auto& number : card.holding) {
            if(card.winning.contains(number)) {
                card.matches++;
            }
        }

        // (Cards will never make you copy a card past the end of the table.)
        // What.
        for(int i = 0; i < card.matches; i++) {
            cards[i + card.card_num].instances += card.instances;
            //std::cout << "Card " << card.card_num << " [" << card.matches << "] won " 
            //    << card.instances << " of card " << i + card.card_num + 1 << "\n";
        }
    }

    size_t sum = 0;
    for(const auto& card : cards) {
        //std::cout << "Card " << card.card_num << " [" << card.matches << "]: " << card.instances << "\n";
        sum += card.instances;
    }
	
    return sum;
}

int main() {
	std::string temp;
    std::vector<std::string> lines;
    while(std::getline(std::cin, temp) && !temp.empty()) {
        lines.push_back(temp);
    }

	std::cout << solve_part_1(lines) << std::endl;
	std::cout << solve_part_2(lines) << std::endl;

    return 0;
}