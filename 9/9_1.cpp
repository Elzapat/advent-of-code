#include <iostream>
#include <fstream>
#include <vector>

int main () {

    std::ifstream input("9.txt");
    std::string line;
    std::vector<long> numbers;
    bool valid = false;

    while (std::getline(input, line)) {
        numbers.push_back(std::stol(line));
    }

    for (int i = 25; i < numbers.size(); ++i) {
        for (int j = i - 25; j < i; ++j) {
            for (int k = i - 25; k < i; ++k) {
                if (numbers[i] == numbers[j] + numbers[k]) valid = true;
            }
        }
        if (valid) valid = false;
        else {
            std::cout << numbers[i] << std::endl;
            break;
        }
    }

    input.close();
    return 0;
}
