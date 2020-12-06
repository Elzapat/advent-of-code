#include <iostream>
#include <fstream>
#include <vector>
#include <map>

int main () {

    std::ifstream input("6.txt");
    std::string line;
    std::vector<std::string> tab;
    std::map<char, int> map;
    int count = 0;
    int n = 0;

    while (std::getline(input, line)) {
        if (line == "") {
            for (auto l : tab) {
                for (auto c : l) {
                    map[c] += 1;
                }
            }
            for (auto& [k, v] : map) {
                if (v == tab.size()) n++;
            }
            count += n;
            n = 0;
            tab.clear();
            map.clear();
            continue;
        }

        tab.push_back(line);
    }

    std::cout << count << std::endl;
    return 0;
}
