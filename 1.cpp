#include <iostream>
#include <fstream>
#include <vector>

int main () {

    std::ifstream input("1.in");

    std::string line;

    std::vector<int> tab;
    while(std::getline(input, line)) {
        tab.push_back(std::stoi(line));
    }

    for (int i : tab) {
        for (int i2 : tab) {
            for (int i3 : tab) {
                if (i + i2 + i3 == 2020) {
                    std::cout << i << " * " << i2 << " * " << i3 << " = " << i * i2 * i3 << std::endl;
                }
            }
        }
    }
}
