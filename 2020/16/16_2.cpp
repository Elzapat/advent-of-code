#include <iostream>
#include <fstream>
#include <vector>
#include <map>
#include <algorithm>
#include <unordered_set>
#include "../split.hpp"

int main() {

    std::ifstream input("16.txt");
    std::string line;
    std::vector<std::pair<std::string, std::pair<std::pair<int, int>, std::pair<int, int>>>> fields;

    for (int i = 0; i < 20; ++i) {
        std::getline(input, line);

        size_t colon_pos = line.find(':');
        std::string field = line.substr(0, colon_pos);
        
        size_t hyphen_pos = line.find('-');
        size_t or_pos = line.find("or", colon_pos);

        std::pair<int, int> first_range(std::stoi(line.substr(colon_pos + 2, hyphen_pos - (colon_pos + 2))),
                                        std::stoi(line.substr(hyphen_pos + 1, (or_pos - 1) - (hyphen_pos + 1))));

        hyphen_pos = line.find('-', or_pos);

        std::pair<int, int> second_range(std::stoi(line.substr(or_pos + 3, hyphen_pos - (or_pos + 3))), 
                                         std::stoi(line.substr(hyphen_pos + 1, line.size() - (hyphen_pos + 1))));

        fields.push_back(std::pair<std::string, std::pair<std::pair<int, int>, std::pair<int, int>>>
                (field, std::pair<std::pair<int, int>, std::pair<int, int>>(first_range, second_range)));
    }

    std::vector<int> my_ticket;
    for (int i = 0; i < 5; ++i) {
        std::getline(input, line);
        if (i == 2) {
            std::vector<std::string> numbers = split(line, ",");
            for (auto l : numbers)
                my_ticket.push_back(std::stoi(l));
        }
    }

    std::map<std::string, std::vector<int>> fields_proba;

    while (std::getline(input, line)) {
        std::vector<std::string> numbers = split(line, ",");
        std::vector<int> fields_values;
        for (auto l : numbers) fields_values.push_back(std::stoi(l));
        int invalid = 0;
        bool invalid_ticket = false;

        for (auto field_value : fields_values) {
            invalid = 0;
            for (auto field : fields) {
                if (!((field_value >= field.second.first.first && field_value <= field.second.first.second) ||
                    (field_value >= field.second.second.first && field_value <= field.second.second.second))) {
                    invalid++;
                }
            }
            if (invalid == 20) {
                invalid_ticket = true;
                break;
            }
        }
        if (invalid_ticket) continue;

        int i = 0;
        for (auto field_value : fields_values) {
            for (auto field : fields) {
                if ((field_value >= field.second.first.first && field_value <= field.second.first.second) ||
                    (field_value >= field.second.second.first && field_value <= field.second.second.second)) {
                    fields_proba[field.first].push_back(i);
                }
            }
            i++;
        }
    }

    std::map<std::string, int> fields_pos;
    std::unordered_set<int> taken;

    for (auto& [field, positions] : fields_proba) {
        std::map<int, int> counter;
        int field_pos = 0;
        int max = 0;

        for (auto pos : positions)
            counter[pos]++;

        for (auto [pos, count] : counter) {
            if (count > max) {
                if (taken.find(pos) == taken.end()) {
                    taken.insert(pos);
                    field_pos = pos;
                    max = count;
                }
            }
        }

        fields_pos[field] = field_pos;
    }

    long product = 1;

    for (auto [field, field_pos] : fields_pos) {
        std::cout << field << " " << field_pos << std::endl;
        if (field.find("departure") != std::string::npos) {
            product *= my_ticket[field_pos];
        }
    }

    std::cout << product << std::endl;

    input.close();
    return 0;
}
