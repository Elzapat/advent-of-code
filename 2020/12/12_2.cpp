#include <iostream>
#include <fstream>
#include <vector>
#include <cmath>

int main() {

    std::ifstream input("12.txt");
    std::string line;
    std::vector<std::pair<std::string, int>> instructions;

    while (std::getline(input, line))
        instructions.push_back(std::pair<std::string, int>(line.substr(0, 1), std::stoi(line.substr(1, line.size() - 1))));

    int waypointX = 10;
    int waypointY = -1;
    int ferryX = 0;
    int ferryY = 0;
    int rotation = 0;

    for (auto instr : instructions) {
        if (instr.first == "F") {
            for (int i = 0; i < instr.second; ++i) {
                ferryX += waypointX;
                ferryY += waypointY;
            }
        } else if (instr.first == "R") {
            int rotation = instr.second / 90;
            for (int i = 0; i < rotation; ++i) {
                int temp = waypointX;
                waypointX = -waypointY;
                waypointY = temp;
            }
        } else if (instr.first == "L") {
            int rotation = instr.second / 90;
            for (int i = 0; i < rotation; ++i) {
                int temp = -waypointX;
                waypointX = waypointY;
                waypointY = temp;
            }
        } else if (instr.first == "N") waypointY -= instr.second;
        else if (instr.first == "S") waypointY += instr.second;
        else if (instr.first == "W") waypointX -= instr.second;
        else if (instr.first == "E") waypointX += instr.second;
    }

    std::cout << std::abs(ferryX) + std::abs(ferryY) << std::endl;

    input.close();
    return 0;
}
