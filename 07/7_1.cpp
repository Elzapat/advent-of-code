#include <iostream>
#include <fstream>
#include <vector>
#include <map>
#include <stdlib.h>
#include <algorithm>

int main () {

    std::ifstream input("7.txt");
    std::string line;
    int n = 0;
    std::string bag;
    std::map<std::string, std::map<std::string, int>> map;
    std::vector<std::string> bags;

    while (std::getline(input, line)) {
        size_t pos = line.find("bags");
        bag = line.substr(0, pos - 1);
        if ((pos = line.find("shiny gold")) != std::string::npos && pos != 0) bags.push_back(bag);
        size_t pos2 = line.find("contain") + 8;
        while (1) {
            char nb = line[pos2];
            size_t pos3 = line.find("bag", pos2) - 1;
            int toAdd = 0;
            if (line[pos3 + 4] == 's') toAdd = 7;
            else toAdd = 6;
            pos2 += 2;
            std::string bagname = line.substr(pos2, pos3 - pos2);
            map[bag][bagname] = atoi(&nb);
            pos2 = pos3 + toAdd;
            if (pos2 >= line.size()) break;
        }
    }

    int size = 0;
    while (size != bags.size()) {
        size = bags.size();
        for (auto l : bags) {
            for (auto [bag1, map2] : map) {
                for (auto [bag2, nb] : map2) {
                    if (l == bag2) {
                        if (std::find(bags.begin(), bags.end(), bag1) == bags.end()) {
                            bags.push_back(bag1);
                        }
                    }
                }
            }
        }
    }

    std::cout << size << std::endl;
    return 0;
}
