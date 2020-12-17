#include <iostream>
#include <fstream>
#include <vector>

#define SIZE 24

int main() {

    std::ifstream input("17.txt");
    std::string line;

    int cubes[SIZE][SIZE][SIZE] = {0};
    int cubes_copy[SIZE][SIZE][SIZE] = {0};

    int i = 0;
    while (std::getline(input, line)) {
        int size = line.size();
        for (int j = 0; j < size; ++j) {
            if (line[j] == '#') {
                cubes[SIZE / 2][size + i][size + j] = 1;
                cubes_copy[SIZE / 2][size + i][size + j] = 1;
            }
        }
        i++;
    }

    for (int cycle = 0; cycle < 6; ++cycle) {
        /*
        for (int d = 0; d < SIZE; ++d) {
            for (int e = 0; e < SIZE; ++e) {
                for (int f = 0; f < SIZE; ++f) {
                    std::cout << cubes[d][e][f];
                }
                std::cout << std::endl;
            }
            std::cout << "-----" << std::endl;
        }
        */
        for (int i = 1; i < SIZE - 1; ++i) {
            for (int j = 1; j < SIZE - 1; ++j) {
                for (int k = 1; k < SIZE - 1; ++k) {
                    int active = 0;
                    for (int a = i - 1; a <= i + 1; ++a) {
                        for (int b = j - 1; b <= j + 1; ++b) {
                            for (int c = k - 1; c <= k + 1; ++c) {
                                /*
                                if (a == i && b == j && c == k)
                                    continue;
                                else if (a < 0 || a >= SIZE ||
                                         b < 0 || b >= SIZE ||
                                         c < 0 || c >= SIZE)
                                    continue;
                                */
                                if (cubes[a][b][c]) active++;
                            }
                        }
                    }
                    if (!(cubes[i][j][k] && (active == 2 || active == 3)))
                        cubes_copy[i][j][k] = 0;

                    if (!cubes[i][j][k] && active == 3)
                        cubes_copy[i][j][k] = 1;
                }
            }
        }
        for (int i = 0; i < SIZE; ++i) {
            for (int j = 0; j < SIZE; ++j) {
                for (int k = 0; k < SIZE; ++k) {
                    cubes[i][j][k] = cubes_copy[i][j][k];
                }
            }
        }
    }

    int nb_active = 0;
    for (int i = 0; i < SIZE; ++i) {
        for (int j = 0; j < SIZE; ++j) {
            for (int k = 0; k < SIZE; ++k) {
                if (cubes[i][j][k]) nb_active++;
            }
        }
    }

    std::cout << nb_active << std::endl;
}
