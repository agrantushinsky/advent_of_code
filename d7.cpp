#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <optional>
#include <unordered_map>
#include <functional>
#include <set>

const std::unordered_map<char, int> card_strength = {{ 
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'
}};

enum class hand_type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
};

struct hand {
    std::string raw_cards;
    std::unordered_map<char, int> cards;
    hand_type type;
    int bet;
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

//so bad
hand_type get_hand_type(const std::unordered_map<char, int>& hand) {
    int pairs = 0;
    hand_type type = hand_type::HighCard;

    for(const auto [card, count] : hand) {
        if(count == 5) {
            return hand_type::FiveOfAKind;
        } else if(count == 4) {
            return hand_type::FourOfAKind;
        } else if(count == 3) {
            type = hand_type::ThreeOfAKind;
            pairs++;
        }
        else if(count >= 2) {
            pairs++;
        }
    }

    if(pairs == 2) {
        if(type == hand_type::ThreeOfAKind) {
            return hand_type::FullHouse;
        } 
        return hand_type::TwoPair;
    } else if(pairs == 1) {
        return hand_type::OnePair;
    }

    return type;
}

std::vector<hand> parse_hands(const std::vector<std::string> &lines) {
    std::vector<hand> hands{};
    for (const auto &line : lines) {
        const auto& split = split_string(line, " ", 1);

        hand hand{};
        hand.raw_cards = split[0];
        hand.bet = std::atoi(split[1].c_str());

        for(char card : hand.raw_cards) {
            hand.cards[card]++;
        }
        hand.type = get_hand_type(hand.cards);

        std::cout << hand.raw_cards << " has type " << (int)hand.type << "\n";

        hands.push_back(hand);
    }
    return hands;
}

int solve_part_1(std::vector<hand>& hands) {
    const auto strength_order = [](hand h1, hand h2) { 
        return h1.type > h2.type; 
    };
    std::sort(hands.begin(), hands.end(), strength_order);

    int sum = 0;
    for(int i = 0; i < hands.size(); i++) {
        const auto& hand = hands[i];
        sum += hand.bet * (i + 1);
    }

    return sum;
}

int main() {
    std::string temp;
    std::vector<std::string> lines;
    while (std::getline(std::cin, temp) && !temp.empty()) {
        lines.push_back(temp);
    }

    auto hands = parse_hands(lines);

    std::cout << solve_part_1(hands) << std::endl;

    return 0;
}