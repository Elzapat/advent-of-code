#include <iostream>
#include <fstream>
#include <vector>
#include <unordered_set>

int main () {

    std::ifstream input("8.txt");
    std::string line;
    std::vector<std::string> tab;
    std::unordered_set<int> set;

    while (std::getline(input, line)) {
        tab.push_back(line);
    }

    int acc = 0;
    for (int i = 0; i < tab.size();) {
        if (set.find(i) != set.end()) break;
        set.insert(i);

        std::string instr = tab[i].substr(0, 3);
        int nb = std::stoi(tab[i].substr(4, tab[i].size()));

        if (instr == "nop") i++;
        else if (instr == "jmp") i += nb;
        else if (instr == "acc") {
            acc += nb;
            i++;
        }
    }

    std::cout << acc << std::endl;
    input.close();
    return 0;
}
