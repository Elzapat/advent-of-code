#include <fstream>
#include <iostream>
#include <vector>

int main () {

    std::ifstream input("4.txt");
    std::string line;

    int n = 0;
    bool byr = false, iyr = false, eyr = false, hgt = false, hcl = false, ecl = false, pid = false;

    while (std::getline(input, line)) {
        if (line.find("byr:") != std::string::npos) byr = true;
        if (line.find("iyr:") != std::string::npos) iyr = true;
        if (line.find("eyr:") != std::string::npos) eyr = true;
        if (line.find("hgt:") != std::string::npos) hgt = true;
        if (line.find("hcl:") != std::string::npos) hcl = true;
        if (line.find("ecl:") != std::string::npos) ecl = true;
        if (line.find("pid:") != std::string::npos) pid = true;

        if (line.size() < 4 && byr && iyr && eyr && hgt && hcl && ecl && pid) {
            n++;
            byr = false;
            iyr = false;
            eyr = false;
            hgt = false;
            hcl = false;
            ecl = false;
            pid = false;
        } else if (line.size() < 4) {
            byr = false;
            iyr = false;
            eyr = false;
            hgt = false;
            hcl = false;
            ecl = false;
            pid = false;
        }
    }


    std::cout << n << std::endl;
    return 0;
}
