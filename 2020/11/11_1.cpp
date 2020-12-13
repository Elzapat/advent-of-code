#include <iostream>
#include <fstream>
#include <vector>
#include <unordered_set>

int main () {

    std::ifstream input("11.txt");
    std::string line;
    std::vector<std::string> layout;
    std::vector<std::string> layout2;

    while (std::getline(input, line)) {
        layout.push_back(line);
        layout2.push_back(line);
    }

    int changes = 1;
    int occupied = 0;
    while (changes) {
        changes = 0;
        for (int i = 0; i < layout.size(); ++i) {
            for (int j = 0; j < layout[i].size(); ++j) {
                occupied = 0;
                for (int k = i - 1; k <= i + 1; ++k) {
                    for (int l = j - 1; l <= j + 1; ++l) {
                        if ((k != i || l != j) && k >= 0 && l >= 0 && k < layout.size() && l < layout[i].size()) {
                            if (layout[k][l] == '#') occupied++;
                        }
                    }
                }
                if (layout[i][j] == 'L' && occupied == 0) {
                    layout2[i][j] = '#';
                    changes++;
                } else if (layout[i][j] == '#' && occupied >= 4) {
                    layout2[i][j] = 'L';
                    changes++;
                }
            }
        }
        std::cout << changes << std::endl;
        for (int i = 0; i < layout.size(); ++i) layout[i] = layout2[i];
    }

    int nbOccupied = 0;
    for (auto row : layout) {
        for (auto seat : row) {
            if (seat == '#') nbOccupied++;
        }
    }

    std::cout << nbOccupied << std::endl;

    input.close();
    return 0;
}
