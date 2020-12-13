#include <iostream>
#include <fstream>
#include <vector>
#include "../split.hpp"
#include <map>

int main () {

    std::ifstream input("13.txt");
    std::string line;
    int min_depart = 0;
    int min_time = 100000000;
    std::string busses_s;
    std::map<int, int> map;

    input >> min_depart;
    input >> busses_s;

    std::vector<std::string> busses = split(busses_s, ","); 

    for (auto& bus : busses) {
        if (bus == "x") continue;
        int id = std::stoi(bus);
        int max_time = 0;
        while (max_time < min_depart) max_time += id;
        map[id] = max_time;
    }

    for (auto& [id, time] : map) {
        if (time < min_time) min_time = time;
    }

    int res = 0;
    for (auto& [id, time] : map) {
        if (time == min_time) {
            res = id * (time - min_depart);
            break;
        }
    }

    std::cout << res << std::endl;

    input.close();
    return 0;
}
