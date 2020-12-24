#include <iostream>
#include <algorithm>
#include <vector>
#include <sstream>

std::string part_1(std::vector<int> cups, const int nb_moves, int starting_cup, int part);
std::string part_2(std::vector<int> cups, const int nb_moves, int starting_cup);

int main() {

    const int part_2_nb_cups  = 1000000;
    const int part_2_nb_moves = 10000000;

    std::vector<int> cups_starting_values {2, 1, 9, 7, 4, 8, 3, 6, 5};
    std::vector<int> cups_part_1(9 + 1, 0);
    std::vector<int> cups_part_2(part_2_nb_cups + 1, 0);

    for (int i = 0; i < cups_starting_values.size() - 1; ++i)
        cups_part_1[cups_starting_values[i]] = cups_starting_values[i + 1];
    cups_part_1[cups_starting_values[cups_starting_values.size() - 1]] = cups_starting_values[0];

    for (int i = cups_starting_values.size() + 1; i <= part_2_nb_cups; ++i)
        cups_starting_values.push_back(i);
    for (int i = 0; i < cups_starting_values.size() - 1; ++i)
        cups_part_2[cups_starting_values[i]] = cups_starting_values[i + 1];
    cups_part_2[cups_starting_values[cups_starting_values.size() - 1]] = cups_starting_values[0];

    std::cout << "Part 1: " << part_1(cups_part_1, 100, cups_starting_values[0], 1) << std::endl;
    std::cout << "Part 2: " << part_2(cups_part_2, part_2_nb_moves, cups_starting_values[0]) << std::endl;
    return 0;
}

std::string part_1(std::vector<int> cups, const int nb_moves, int starting_cup, int part) {

    const int highest_label = *std::max_element(cups.begin(), cups.end());
    const int lowest_label = 1;
    const int nb_pick_up = 3;
    int picked_cups[nb_pick_up];

    int current_cup = starting_cup;
    for (int i = 0; i < nb_moves; ++i) {

        int picked_cup = current_cup;
        for (int j = 0; j < nb_pick_up; ++j) {
            picked_cups[j] = cups[picked_cup]; 
            picked_cup = picked_cups[j];
        }
        cups[current_cup] = cups[picked_cup];

        int destination = current_cup - 1;
        while (std::find(picked_cups, picked_cups + nb_pick_up, destination) != picked_cups + nb_pick_up ||
               destination == lowest_label - 1)
            destination = destination == lowest_label - 1 ? highest_label : destination - 1;

        int placed_cup = destination;
        int temp = cups[placed_cup];
        for (int j = 0; j < nb_pick_up; ++j) {
            cups[placed_cup] = picked_cups[j]; 
            placed_cup = cups[placed_cup];
        }
        cups[picked_cup] = temp;

        current_cup = cups[current_cup];
    }

    if (part == 1) {
        std::stringstream answer;
        for (int next_cup = cups[1]; next_cup != 1; next_cup = cups[next_cup]) {
            if (next_cup == 0) continue;
            answer << next_cup;
        }
        return answer.str();
    } else {
        long first = cups[1];
        long second = cups[first];

        return std::to_string(first * second);
    }
}

std::string part_2(std::vector<int> cups, const int nb_moves, int starting_cup) {

    return part_1(cups, nb_moves, starting_cup, 2);
}
