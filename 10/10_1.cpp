#include <iostream>
#include <fstream>
#include <vector>
#include <unordered_set>
#include <algorithm>

int main () {

    std::ifstream input("10.txt");
    std::string line;
    std::vector<int> adapt;
    int nb1Jolt = 0;
    int nb3Jolt = 0;

    while (std::getline(input, line))
        adapt.push_back(std::stoi(line));

    adapt.push_back(0);
    std::sort(adapt.begin(), adapt.end());
    adapt.push_back(adapt[adapt.size() - 1] + 3);

    for (int i = 0; i < adapt.size(); ++i) {
        int diff = 0;
        if (i == 0) diff = adapt[i];
        else diff = adapt[i] - adapt[i - 1];

        if (diff == 1) nb1Jolt++;
        else if (diff == 3) nb3Jolt++;
    }

    std::cout << nb1Jolt *  nb3Jolt << std::endl;

    input.close();
    return 0;
}
