#include <iostream>
#include <fstream>
#include <sstream>
#include <map>
#include <vector>

bool is_valid(std::multimap<int, std::vector<std::string>& rules, std::string<std::string>& rule, std::string& message);

int main() {

    std::ifstream input("19ex.txt");
    std::string line;
    std::multimap<int, std::vector<std::string>> rules;

    while (std::getline(input, line)) {
        if (line == "") break;

        std::stringstream ss;
        ss << line;
        std::string rule_piece;
        std::vector<std::string> rules_to_match;
        int rule_id = 0;

        while (ss >> rule_piece) {
            if (rule_piece.find(':') != std::string::npos) {
                rule_piece.pop_back();
                rule_id = std::stoi(rule_piece);
            } else if (rule_piece == "|") {
                rules.insert(std::make_pair(rule_id, rules_to_match));
                rules_to_match.clear();
            } else {
                rules_to_match.push_back(rule_piece);
            }
        }

        rules.insert(std::make_pair(rule_id, rules_to_match));
    }

    for (auto [id, rules] : rules) {
        std::cout << id << ": ";
        for (auto rule : rules) {
            std::cout << rule << ", ";
        }
        std::cout << std::endl;
    }

    int valid = 0;
    while (std::getline(input, line)) {
        if (is_valid(rules, rules[0], line)) valid++;
    }

    std::cout << valid << std::cout;

    input.close();
    return 0;
}

bool is_valid(std::multimap<int, std::vector<std::string>& rules, std::vector<std::string>& rule, std::string& message) {

    if (rule[0].find())
}
