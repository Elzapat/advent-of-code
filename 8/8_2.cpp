#include <iostream>
#include <fstream>
#include <vector>
#include <unordered_set>

int bruteForce (std::vector<std::string> tab, int& acc);

int main () {

    std::ifstream input("8.txt");
    std::string line;
    std::vector<std::string> tab;
    std::unordered_multiset<int> set;

    while (std::getline(input, line)) {
        tab.push_back(line);
    }

    for (int i = 0; i < tab.size(); ++i) {
        std::string instr = tab[i].substr(0, 3);
        int nb = std::stoi(tab[i].substr(4, tab[i].size()));

        std::vector<std::string> testTab;
        for (auto l : tab) testTab.push_back(l);
        int testAcc = 0;
        if (instr == "jmp") {
            testTab[i] = "nop " + std::to_string(nb);
            if (bruteForce(testTab, testAcc)) {
                std::cout << testAcc << std::endl;
                break;
            }
        } else if (instr == "nop") {
            testTab[i] = "jmp " + std::to_string(nb);
            if (bruteForce(testTab, testAcc)) {
                std::cout << testAcc << std::endl;
                break;
            }
        }
    }

    input.close();
    return 0;
}

int bruteForce (std::vector<std::string> tab, int& acc) {

    std::unordered_multiset<int> set;
    for (int i = 0; i < tab.size();) {
        if (set.find(i) != set.end()) return 0;
        set.insert(i);

        std::string instr = tab[i].substr(0, 3);
        int nb = std::stoi(tab[i].substr(4, tab[i].size()));
        if (instr == "nop") {
            i++;
            continue;
        } else if (instr == "jmp") i += nb;
        else if (instr == "acc") {
            acc += nb;
            i++;
        }
    }

    return 1;
}
