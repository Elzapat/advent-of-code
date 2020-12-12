#include <iostream>
#include <fstream>
#include <vector>
#include <map>
#include <cmath>

enum Direction {
    NORTH, WEST, SOUTH, EAST
};

int main() {

    std::ifstream input("12.txt");
    std::string line;
    std::vector<std::pair<std::string, int>> instructions;

    while (std::getline(input, line)) {
        instructions.push_back(std::pair<std::string, int>(line.substr(0, 1), std::stoi(line.substr(1, line.size() - 1))));
    }

    std::map<Direction, int> map;
    map[NORTH] = 0;
    map[WEST] = 0;
    map[SOUTH] = 0;
    map[EAST] = 0;

    Direction facing = EAST;
    int rotation = 0;

    for (auto instr : instructions) {
        if (instr.first == "F") map[facing] += instr.second;
        else if (instr.first == "R") {
            rotation = instr.second / 90;
            for (int i = 0; i < rotation; ++i) {
                if (facing == EAST) facing = SOUTH;
                else if (facing == SOUTH) facing = WEST;
                else if (facing == WEST) facing = NORTH;
                else if (facing == NORTH) facing = EAST;
            }
        } else if (instr.first == "L") {
            rotation = instr.second / 90;
            for (int i = 0; i < rotation; ++i) {
                if (facing == EAST) facing = NORTH;
                else if (facing == NORTH) facing = WEST;
                else if (facing == WEST) facing = SOUTH;
                else if (facing == SOUTH) facing = EAST;
            }
        } else if (instr.first == "N") map[NORTH] += instr.second;
        else if (instr.first == "S") map[SOUTH] += instr.second;
        else if (instr.first == "E") map[EAST] += instr.second;
        else if (instr.first == "W") map[WEST] += instr.second;
    }

    std::cout << std::abs(map[SOUTH] - map[NORTH]) + std::abs(map[EAST] - map[WEST]) << std::endl;

    input.close();
    return 0;
}
