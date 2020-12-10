#include <iostream>
#include <fstream>
#include <vector>

struct Password {
    int max, min;
    std::string password;
    std::string letter;
};

int main () {

    std::ifstream input("2.txt");

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
        if (pwd.password[pwd.min - 1] == pwd.letter[0]) {
           if (pwd.password[pwd.max - 1] == pwd.letter[0]);
           else { n++; continue;}
        } else if (pwd.password[pwd.max - 1] == pwd.letter[0]) {
            if (pwd.password[pwd.min - 1] == pwd.letter[0]);
            else { n++; continue; }
        }
    }

    std::cout << n << std::endl;
    
    return 0;
}
