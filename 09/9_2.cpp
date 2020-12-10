#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

int main () {

    std::ifstream input("9.txt");
    std::string line;
    std::vector<long> numbers;
    std::vector<long> sumNumbers;
    long firstInv = 2089807806;
    long long sum = 0;

    while (std::getline(input, line)) {
        numbers.push_back(std::stol(line));
    }

    for (int i = 0; i < numbers.size(); ++i) {
        for (int j = i; j < numbers.size(); ++j) {
            sum += numbers[j];
            sumNumbers.push_back(numbers[j]);
            if (sum == firstInv) {
                std::cout << "found!" << std::endl;
                goto end;
            } else if (sum > firstInv) {
                sum = 0;
                sumNumbers.clear();
                break;
            }
        }
    }

end:
    std::sort(sumNumbers.begin(), sumNumbers.end());
    std::cout << sumNumbers[0] + sumNumbers[sumNumbers.size() - 1] << std::endl;

    input.close();
    return 0;
}
