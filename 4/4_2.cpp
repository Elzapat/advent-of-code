#include <fstream>
#include <iostream>
#include <vector>

int main () {

    std::ifstream input("4.txt");
    std::string line;

    int n = 0;
    int i = 0;
    bool byr = false, iyr = false, eyr = false, hgt = false, hcl = false, ecl = false, pid = false;
    size_t pos = 0;

    while (std::getline(input, line)) {
        if ((pos = line.find("byr:")) != std::string::npos) {
            int year = std::stoi(line.substr(pos + 4, 4));
            if (year >= 1920 && year <= 2002) byr = true;
        }
        if ((pos = line.find("iyr:")) != std::string::npos) {
            int year = std::stoi(line.substr(pos + 4, 4));
            if (year >= 2010 && year <= 2020) iyr = true;
        }
        if ((pos = line.find("eyr:")) != std::string::npos) {
            int year = std::stoi(line.substr(pos + 4, 4));
            if (year >= 2020 && year <= 2030) eyr = true;
        }
        if ((pos = line.find("hgt:")) != std::string::npos) {
            size_t ogPos = pos + 4;
            pos += 4;
            while (std::isdigit(line[(int)pos])) pos++;
            std::string unit = line.substr(pos, 2);
            std::string machin = line.substr(ogPos, pos - ogPos); 
            int nb = std::stoi(machin);

            if (unit == "in" && nb >= 59 && nb <= 76) hgt = true;
            else if (unit == "cm" && nb >= 150 && nb <= 193) hgt = true;
        }
        if ((pos = line.find("hcl:")) != std::string::npos) {
            pos += 5; 
            std::string code = line.substr(pos, 6);
            if (line[pos - 1] == '#' && code.find_first_not_of("0123456789abcdef") == std::string::npos) hcl = true;
        }
        if ((pos = line.find("ecl:")) != std::string::npos) {
            std::string value = line.substr(pos + 4, 3);
            if (value == "amb" || value == "blu" || value == "brn" || value == "gry" || value == "grn" || value == "hzl" || value == "oth")
                ecl = true;
        }
        if ((pos = line.find("pid:")) != std::string::npos) {
            pos += 4;
            size_t ogPos = pos;

            while (std::isdigit(line[pos++])); 
            std::string s = line.substr(ogPos, pos - ogPos - 1);

            if (s.size() == 9) pid = true;
        }

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
