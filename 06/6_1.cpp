#include <iostream>
#include <fstream>
#include <vector>

int main () {

    std::ifstream input("6.txt");
    std::string line;
    std::vector<char> tab;
    std::string s = "";
    int count = 0;
    int n = 0;

    while (std::getline(input, line)) {
        if (line == "") {
            count += n;
            n = 0;
            s = "";
            continue;
        }
        for (int i = 0; i < line.size(); ++i) {
            if (s.find(line[i]) == std::string::npos) {
                s += line[i];
                n++;
            }
        }
    }

    std::cout << count << std::endl;
    return 0;
}
