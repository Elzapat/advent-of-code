#include <iostream>
#include <fstream>
#include <vector>
#include <map>
#include <cmath>

int main () {

    std::ifstream input("14.txt");
    std::string line;
    std::vector<int> mask;
    std::map<long, long> mem;

    while (std::getline(input, line)) {
        if (line.find("mask") != std::string::npos) {
            mask.clear();
            for (int i = 7; i < line.size(); ++i) {
                if (line[i] == 'X') mask.push_back(2);
                else if (line[i] == '0') mask.push_back(0);
                else if (line[i] == '1') mask.push_back(1);
                else std::cout << "problem" << std::endl;
            }
        } else {
            size_t pos = line.find("]");
            long addr = std::stol(line.substr(4, pos - 4));
            long value = std::stol(line.substr(pos + 4, line.size() - pos + 4));

            long addr_temp = addr;
            std::vector<int>  addr_bit;
            std::vector<long> addresses;

            for (int i = 35; i >= 0; --i) {
                if (addr_temp >= std::pow(2, i)) {
                    addr_bit.push_back(1);
                    addr_temp -= std::pow(2, i);
                } else addr_bit.push_back(0);
            }

            for (int i = 0; i < 36; ++i) {
                if (mask[i] == 0) continue;
                else addr_bit[i] = mask[i];
            }

            std::vector<int> Xpos;
            for (int i = 0; i < addr_bit.size(); ++i)
                if (addr_bit[i] == 2) Xpos.push_back(i);

            for (int i = 0; i < std::pow(2, Xpos.size()); ++i) {
                for (int j = 0; j < Xpos.size(); ++j) {
                    if ((i % (int)std::pow(2, j)) == 0) {
                        addr_bit[Xpos[j]] = addr_bit[Xpos[j]] == 0 ? 1 : 0;
                    }
                }

                long address = 0;
                for (int j = 0; j < 36; ++j)
                    address += addr_bit[j] == 1 ? std::pow(2, j) : 0;

                addresses.push_back(address);
            }

            for (auto address : addresses)
                mem[address] = value;
            addresses.clear();
        }
    }

    long sum = 0;
    for (auto& [addr, value] : mem)
        sum += value;

    std::cout << sum << std::endl;

    input.close();
    return 0;
}
