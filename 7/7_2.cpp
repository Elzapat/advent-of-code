#include <iostream>
#include <fstream>
#include <vector>
#include <map>
#include <stdlib.h>
#include <algorithm>

int nbBags (std::map<std::string, std::map<std::string, int>> map, std::map<std::string, int> map2);

int main () {

    std::ifstream input("7.txt");
    std::string line;
    std::string bag;
    std::map<std::string, std::map<std::string, int>> map;

    while (std::getline(input, line)) {
        size_t pos = line.find("bags");
        bag = line.substr(0, pos - 1);
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

    int size = nbBags(map, map["shiny gold"]) - 1;

    std::cout << size << std::endl;
    return 0;
}

int nbBags (std::map<std::string, std::map<std::string, int>> map, std::map<std::string, int> map2) {

    int size = 0;
    for (auto [bag, nb] : map2) {
        if (bag == " other") return 1;
        else size += nb * nbBags(map, map[bag]);
    }

    return size + 1;
}
