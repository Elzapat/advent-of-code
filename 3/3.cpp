#include <iostream>
#include <fstream>
#include <vector>

int main () {

    std::ifstream input("3.in");
    std::string line;

    char** map;
    std::vector<std::string> tab;

    while (std::getline(input, line)) {
        tab.push_back(line);
    }

    int nbTrees;
    int i = 0;
    int k = 0;
    for (auto l : tab) {
        //k++;
        //if (k % 2 == 0) continue;
        if (l[i] == '#') nbTrees++;
        for (int j = 0; j < 3; j++) {
            i++;
            if (i == l.size()) i = 0;
        }
    }

    std::cout << nbTrees << std::endl;
}
