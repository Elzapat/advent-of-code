#include <iostream>
#include <fstream>
#include <vector>

int main() {

    std::ifstream input("17.txt");
    std::string line;
    std::vector<std::string> slice;
    std::vector<std::vector<std::string>> cubes;
    std::vector<std::vector<std::string>> cubes_temp;

    while (std::getline(input, line)) {
        slice.push_back(line);
    }

    int size = slice.size();

    for (int i = -size; i <= size; ++i) {
        if (i == 0) {
            int a = 0;
            for (int b = 0; b < slice.size(); ++b) {
                while (slice[b].size() != 2 * size + 1) {
                    if (a % 2) slice[b] = "." + slice[b];
                    else slice[b] = slice[b] + ".";
                    a++;
                }
            }

            std::string row = "";
            for (int k = -size; k <= size; ++k) {
                row += '.';
            }
            for (int b = 0; b < size - 4; ++b) {
                slice.insert(slice.begin(), row);
                slice.push_back(row);
            }

            cubes.push_back(slice);
            cubes_temp.push_back(slice);
            continue;
        }
        std::vector<std::string> temp;
        for (int j = -size; j <= size; ++j) {
            std::string row = "";
            for (int k = -size; k <= size; ++k) {
                row += '.';
            }
            temp.push_back(row);
        }

        cubes.push_back(temp);
        cubes_temp.push_back(temp);
    }

    /*for (int i = 0; i < 20; ++i) {
        if (i == 10) {
            cubes.push_back(slice);
            cubes_temp.push_back(slice);
            continue;
        }

        std::vector<std::string> temp;
        for (int j = 0; j < 8 + i * 2; ++j) {
            
        }
        cubes.push_back(temp);
        cubes_temp.push_back(temp);
    }*/

    /*int nb_layers = slice.size() + 3;
    for (int i = 0; i < nb_layers; ++i) {
        std::vector<std::string> temp;
        int size = slice.size() + (nb_layers * 2) - ((i + 1) * 2);
        for (int j = 0; j < size; ++j) {
            std::string row = "";
            for (int k = 0; k < size; ++k)
                row += '.';
            temp.push_back(row);
        }

        cubes.push_back(temp);
        cubes_temp.push_back(temp);
    }

    cubes.push_back(slice);
    cubes_temp.push_back(slice);

    for (int i = nb_layers - 1; i >= 0; --i) {
        std::vector<std::string> temp;
        int size = slice.size() + (nb_layers * 2) - ((i + 1) * 2);
        for (int j = 0; j < size; ++j) {
            std::string row = "";
            for (int k = 0; k < size; ++k)
                row += '.';
            temp.push_back(row);
        }

        cubes.push_back(temp);
        cubes_temp.push_back(temp);
    }*/

    for (int nb_phase = 0; nb_phase < 6; ++nb_phase) {
        for (auto slice : cubes) {
            for (auto line : slice) {
                std::cout << line << std::endl;
            }
            std::cout << "---------" << std::endl;
        }
        std::cout << "||||||||" << std::endl;
        for (int i = 0; i < cubes.size(); ++i) {
            for (int j = 0; j < cubes[i].size(); ++j) {
                for (int k = 0; k < cubes[i][j].size(); ++k) {
                    int active = 0;
                    for (int l = i - 1; l <= i + 1; ++l) {
                        for (int m = j - 1; m <= j + 1; ++m) {
                            for (int n = k - 1; n <= k + 1; ++n) {
                                if (l == i && m == j && n == k) continue;
                                if (l >= 0 && l < cubes.size() && m >= 0 && m < cubes[l].size() && n >= 0 && m < cubes[l][m].size()) {
                                //std::cout << cubes[l][m][n];
                   //                 std::cout << active << cubes[l][m][n];
                                    if (cubes[l][m][n] == '#') active++;
                                }
                            }
                        }
                    }
                    //std::cout << "end" << active << std::endl;

                    //std::cout << active << std::endl;
                    if (cubes[i][j][k] == '#' && (active < 2 || active > 3)) {
                        cubes_temp[i][j][k] = '.';
                    } 

                    if (cubes[i][j][k] == '.' && active == 3) {
                        cubes_temp[i][j][k] = '#';
                    }
                }
            }
        }
        for (int a = 0; a < cubes.size(); ++a) {
            for (int b = 0; b < cubes[a].size(); ++b) {
                cubes[a][b] = cubes_temp[a][b];
            }
        }
    }

        for (auto slice : cubes) {
            for (auto line : slice) {
                std::cout << line << std::endl;
            }
            std::cout << "---------" << std::endl;
        }
        std::cout << "||||||||" << std::endl;
    int nb_active = 0;
    int nb_active2 = 0;
    for (int i = 0; i < cubes.size(); ++i) {
        for (int j = 0; j < cubes[i].size(); ++j) {
            for (int k = 0; k < cubes[i][j].size(); ++k) {
                if (cubes[i][j][k] == '#') nb_active2++;
            }
        }
    }

    for (auto slice : cubes) {
        for (auto line : slice) {
            for (auto cube : line) {
                if (cube == '#') nb_active++;
            }
        }
    }

    std::cout << nb_active << " " << nb_active2 << std::endl;

    input.close();
    return 0;
}
