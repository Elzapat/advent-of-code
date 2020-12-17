#include <iostream>
#include <fstream>

#define SIZE 24

int main() {

    std::ifstream input("17.txt");
    std::string line;

    int cubes[SIZE][SIZE][SIZE][SIZE] = {0};
    int cubes_copy[SIZE][SIZE][SIZE][SIZE] = {0};

    int i = 0;
    while (std::getline(input, line)) {
        int size = line.size();
        for (int j = 0; j < size; ++j) {
            if (line[j] == '#') {
                cubes[SIZE / 2][SIZE / 2][size + i][size + j] = 1;
                cubes_copy[SIZE / 2][SIZE / 2][size + i][size + j] = 1;
            }
        }
        i++;
    }

    for (int cycle = 0; cycle < 6; ++cycle) {
        for (int i = 1; i < SIZE - 1; ++i) {
            for (int j = 1; j < SIZE - 1; ++j) {
                for (int k = 1; k < SIZE - 1; ++k) {
                    for (int l = 1; l < SIZE - 1; ++l) {
                        int active = 0;
                        for (int a = i - 1; a <= i + 1; ++a) {
                            for (int b = j - 1; b <= j + 1; ++b) {
                                for (int c = k - 1; c <= k + 1; ++c) {
                                    for (int d = l - 1; d <= l + 1; ++d) {
                                        if (a == i && b == j && c == k && d == l)
                                            continue;
                                        if (cubes[a][b][c][d]) active++;
                                    }
                                }
                            }
                        }
                        if (!(cubes[i][j][k][l] && (active == 2 || active == 3)))
                            cubes_copy[i][j][k][l] = 0;

                        if (!cubes[i][j][k][l] && active == 3)
                            cubes_copy[i][j][k][l] = 1;
                    }
                }
            }
        }
        for (int i = 0; i < SIZE; ++i) {
            for (int j = 0; j < SIZE; ++j) {
                for (int k = 0; k < SIZE; ++k) {
                    for (int l = 0; l < SIZE; ++l) {
                        cubes[i][j][k][l] = cubes_copy[i][j][k][l];
                    }
                }
            }
        }
    }

    int nb_active = 0;
    for (int i = 0; i < SIZE; ++i) {
        for (int j = 0; j < SIZE; ++j) {
            for (int k = 0; k < SIZE; ++k) {
                for (int l = 0; l < SIZE; ++l) {
                    if (cubes[i][j][k][l]) nb_active++;
                }
            }
        }
    }

    std::cout << nb_active << std::endl;

    return 0;
}
