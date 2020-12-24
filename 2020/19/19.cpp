#include <iostream>
#include <fstream>
#include <sstream>
#include <map>
#include <vector>
#include <tuple>

bool is_valid(std::multimap<int, std::vector<std::string>>& rules, int rule_id, int& index_to_check, std::string& message);

int main() {

    std::ifstream input("19_2.txt");
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

    int valid = 0;
    while (std::getline(input, line)) {
        int index_to_check = 0;
        if (is_valid(rules, 0, index_to_check, line) && index_to_check == line.size())
            valid++;
    }

    std::cout << valid << std::endl;

    input.close();
    return 0;
}

bool is_valid(std::multimap<int, std::vector<std::string>>& rules, int rule_id, int& index_to_check, std::string& message) {

    if (index_to_check >= message.size()) return true;

    if (rules.find(rule_id)->second[0].find('"') != std::string::npos) {
        char letter = rules.find(rule_id)->second[0][1];
        if (message[index_to_check++] == letter) return true;
        else return false;
    }

    int remember_this_index = index_to_check;
    auto range = rules.equal_range(rule_id); 
    bool valid;
    for (auto it = range.first; it != range.second; ++it) {
        valid = true;
        for (auto rule_to_follow : it->second) {
            if (std::stoi(rule_to_follow) == rule_id); 
            valid = valid && is_valid(rules, std::stoi(rule_to_follow), index_to_check, message);
        }
        if (valid) break;
        index_to_check = remember_this_index;
    }

    return valid;
}
