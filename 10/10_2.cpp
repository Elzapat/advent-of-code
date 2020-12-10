#include <iostream>
#include <fstream>
#include <vector>
#include <unordered_set>
#include <algorithm>
#include <unordered_map>

long long nbArrangements (std::vector<int> adapters);

int main () {

    std::ifstream input("10.txt");
    std::string line;
    std::vector<int> adapt;

    while (std::getline(input, line))
        adapt.push_back(std::stoi(line));

    adapt.push_back(0);
    std::sort(adapt.begin(), adapt.end());
    adapt.push_back(adapt[adapt.size() - 1] + 3);

    std::cout << nbArrangements(adapt) << std::endl;

    input.close();
    return 0;
}

long long nbArrangements (std::vector<int> adapters) {

    std::unordered_map<int, long long> map;
    map[0] = 1;

    for (int i = 1; i < adapters.size(); ++i) {
        long long count = 0;
        for (int j = i - 3; j < i; ++j) {
            if (adapters[i] - adapters[j] <= 3)
                count += map[j]; 
        }
        map[i] = count;
    }

    return map[adapters.size() - 1];
}
