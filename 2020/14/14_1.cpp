#include <iostream>
#include <fstream>
#include <vector>
#include <map>
#include <cmath>

int main () {

    std::ifstream input("14.txt");
    std::string line;
    std::vector<int> mask;
    std::map<int, long> mem;

    while (std::getline(input, line)) {
        if (line.find("mask") != std::string::npos) {
            mask.clear();
            for (int i = 7; i < line.size(); ++i) {
                if (line[i] == 'X') mask.push_back(-1);
                else if (line[i] == '0') mask.push_back(0);
                else if (line[i] == '1') mask.push_back(1);
                else std::cout << "problem" << std::endl;
            }
        } else {
            size_t pos = line.find("]");
            int addr = std::stoi(line.substr(4, pos - 4));
            long value = std::stol(line.substr(pos + 4, line.size() - pos + 4));

            long value_temp = value;
            std::vector<int> value_bit;

            for (int i = 35; i >= 0; --i) {
                if (value_temp >= std::pow(2, i)) {
                    value_bit.push_back(1);
                    value_temp -= std::pow(2, i);
                } else value_bit.push_back(0);
            }

            value = 0;
            for (int i = 0; i < 36; ++i) {
                if (mask[i] == 1) value_bit[i] = 1;
                else if (mask[i] == 0) value_bit[i] = 0;

                value += value_bit[i] == 1 ? std::pow(2, 35 - i) : 0;
            }

            mem[addr] = value;
        }
    }

    long sum = 0;
    for (auto& [addr, value] : mem)
        sum += value;

    std::cout << sum << std::endl;

    input.close();
    return 0;
}
