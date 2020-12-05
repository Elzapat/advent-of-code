#include <iostream>
#include <fstream>
#include <math.h>
#include <vector>
#include <algorithm>

int main () {

    std::ifstream input("5.txt");
    std::string line;

    int seatRow = 0;
    int seatCol = 0;
    std::vector<int> IDs;

    while (std::getline(input, line)) {
        for (int i = 0; i < 7; ++i) {
            if (line[i] == 'B') seatRow += pow(2, 7 - i - 1);
        }
        for (int i = 7; i < 10; ++i) {
            if (line[i] == 'R') seatCol += pow(2, 3 - (i - 7) - 1);
        }
        int ID = seatRow * 8 + seatCol;

        IDs.push_back(ID);

        seatRow = 0;
        seatCol = 0;
    }

    std::sort(IDs.begin(), IDs.end());

    int ID = 0;
    for (int i = 0; i < IDs.size() - 1; ++i) {
        if (IDs[i] == (IDs[i + 1] - 2))
            ID = IDs[i] + 1;
    }

    std::cout << ID << std::endl;
    return 0;
}
