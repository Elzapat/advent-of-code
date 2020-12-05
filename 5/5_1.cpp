#include <iostream>
#include <fstream>
#include <math.h>

int main () {

    std::ifstream input("5.txt");
    std::string line;

    int maxID = 0;
    int seatRow = 0;
    int seatCol = 0;

    while (std::getline(input, line)) {
        for (int i = 0; i < 7; ++i) {
            if (line[i] == 'B') seatRow += pow(2, 7 - i - 1);
        }
        for (int i = 7; i < 10; ++i) {
            if (line[i] == 'R') seatCol += pow(2, 3 - (i - 7) - 1);
        }
        int ID = seatRow * 8 + seatCol;
        if (ID > maxID) maxID = ID;
        seatRow = 0;
        seatCol = 0;
    }

    std::cout << maxID << std::endl;
    return 0;
}
