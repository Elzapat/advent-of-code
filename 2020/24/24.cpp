#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <map>
#include <tuple>

typedef std::tuple<int, int, int> tuple_3int;

int part_1(std::map<tuple_3int, int> hexagons);
int part_2(std::map<tuple_3int, int> hexagons);

int main() {

    std::ifstream input("24.txt");
    std::string line;
    std::vector<std::vector<std::string>> tiles_instr;

    while (std::getline(input, line)) {
        std::stringstream ss;
        std::vector<std::string> instr;
        for (int i = 0; i < line.size();) {
            if (line[i] == 'n' || line[i] == 's') {
                ss << line[i] << line[i + 1];
                i += 2;
            } else {
                ss << line[i];
                i++;
            }
            instr.push_back(ss.str());
            ss.str(std::string());
            ss.clear();
        }
        tiles_instr.push_back(instr);
    }

    std::map<tuple_3int, int> hexagons;
    int size = 18;

    for (int x = -size; x <= size; ++x) {
        for (int y = -size; y <= size; ++y) {
            for (int z = -size; z <= size; ++z) {
                hexagons[std::make_tuple(x, y, z)] = 0;
            }
        }
    }

    for (auto tile_instr : tiles_instr) {
        int x = 0, y = 0, z = 0;
        for (auto instr : tile_instr) {
            if (instr == "e") x++, y++;
            else if (instr == "ne") y++, z++;
            else if (instr == "se") x++, z--;
            else if (instr == "w") x--, y--;
            else if (instr == "nw") x--, z++;
            else if (instr == "sw") y--, z--;
        }

        hexagons[std::make_tuple(x, y, z)] = hexagons[std::make_tuple(x, y, z)] == 0 ? 1 : 0;
    }

    std::cout << "Part 1: " << part_1(hexagons) << std::endl;
    std::cout << "Part 2: " << part_2(hexagons) << std::endl;

    input.close();
    return 0;
}

int part_2(std::map<tuple_3int, int> hexagons) {

    const int nb_days = 100;
    std::vector<tuple_3int> pos_diff {
        std::make_tuple(-1, 0, 1),
        std::make_tuple(0, 1, 1),
        std::make_tuple(1, 1, 0),
        std::make_tuple(1, 0, -1),
        std::make_tuple(0, -1, -1),
        std::make_tuple(-1, -1, 0),
    };

    for (int i = 0; i < nb_days; ++i) {
        std::map<tuple_3int, int> hexagons_copy(hexagons);
        for (auto& [pos, facing] : hexagons) {
            int nb_neighbour_black_tiles = 0; 
            for (const auto diff : pos_diff) {
                int x = std::get<0>(pos) + std::get<0>(diff);
                int y = std::get<1>(pos) + std::get<1>(diff);
                int z = std::get<2>(pos) + std::get<2>(diff);

                tuple_3int neighbour_pos = std::make_tuple(x, y, z);
                if (hexagons.find(neighbour_pos) != hexagons.end()) {
                    if (hexagons[neighbour_pos])
                        nb_neighbour_black_tiles++;
                } else {
                    int black_tiles = 0;
                    for (const auto diff2 : pos_diff) {
                        int x2 = std::get<0>(neighbour_pos) + std::get<0>(diff2);
                        int y2 = std::get<1>(neighbour_pos) + std::get<1>(diff2);
                        int z2 = std::get<2>(neighbour_pos) + std::get<2>(diff2);

                        if (hexagons.find(neighbour_pos) != hexagons.end()) {
                            if (hexagons[neighbour_pos])
                                black_tiles++;
                        }
                    }
                    if (black_tiles == 2) hexagons_copy[neighbour_pos] = 1;
                    else hexagons_copy[neighbour_pos] = 0;
                }
            }

            if (facing && (nb_neighbour_black_tiles == 0 || nb_neighbour_black_tiles > 2))
                hexagons_copy[pos] = 0;
            else if (!facing && nb_neighbour_black_tiles == 2)
                hexagons_copy[pos] = 1;
        }
        hexagons = hexagons_copy;
    }

    return part_1(hexagons);
}

int part_1(std::map<tuple_3int, int> hexagons) {

    int nb_black_tiles = 0;
    for (const auto [pos, facing] : hexagons)
        if (facing) nb_black_tiles++;

    return nb_black_tiles;
}
