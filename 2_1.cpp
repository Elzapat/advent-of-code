#include <iostream>
#include <fstream>
#include <vector>

struct Password {
    int max, min;
    std::string password;
    std::string letter;
};

int main () {

    std::ifstream input("2.in");

    std::vector<Password> tab;
    std::string line;
    while (std::getline(input, line)) {
        Password pwd;
        unsigned pos = line.find('-'); 
        unsigned pos2 = line.find(' ');
        pwd.min = std::stoi(line.substr(0, pos));
        pwd.max = std::stoi(line.substr(pos + 1, pos2 - pos));
        pos = line.find(':');
        pwd.letter = line.substr(pos - 1, 1);
        pwd.password = line.substr(pos + 2);
        tab.push_back(pwd);
    }

    int n = 0;
    for (auto pwd : tab) {
        int nbOcc = 0;
        for (auto l : pwd.password) {
            if (l == pwd.letter[0]) nbOcc++;
        }
        if (nbOcc >= pwd.min && nbOcc <= pwd.max) n++;
    }

    std::cout << n << std::endl;
}
