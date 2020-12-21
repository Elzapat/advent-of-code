#include <iostream>
#include <fstream>
#include <unordered_map>
#include <set>
#include <vector>
#include <algorithm>
#include "../split.hpp"

int main() {

    std::ifstream input("21.txt");
    std::string line;
    std::vector<std::set<std::string>> ingredients;
    std::vector<std::vector<std::string>> allergens;

    while (std::getline(input, line)) {
        std::vector<std::string> list;
        list = split(line, " ");

        std::vector<std::string> alle;
        std::set<std::string> ingred;
        size_t pos = 0;
        for (auto ingr : list) {
            bool is_allergen = false;
            if ((pos = ingr.find('(')) != std::string::npos) {
                ingr.erase(ingr.begin() + pos);
                is_allergen = true;
            }
            if ((pos = ingr.find(',')) != std::string::npos) {
                ingr.erase(ingr.begin() + pos);
                is_allergen = true;
            }
            if ((pos = ingr.find(')')) != std::string::npos) {
                ingr.erase(ingr.begin() + pos);
                is_allergen = true;
            }

            if (is_allergen && ingr != "contains") alle.push_back(ingr);
            else if (ingr != "contains") ingred.insert(ingr);
        }

        allergens.push_back(alle);
        ingredients.push_back(ingred);
    }

    std::unordered_map<std::string, std::set<std::string>> map; 

    for (int i = 0; i < allergens.size(); ++i) {
        for (auto allergen : allergens[i]) {
            if (map[allergen].empty()) {
                for (auto ingredient : ingredients[i])
                    map[allergen].insert(ingredient);
            } else {
                std::set<std::string> intersection;

                std::set_intersection(ingredients[i].begin(), ingredients[i].end(),
                                      map[allergen].begin(), map[allergen].end(),
                                      std::inserter(intersection, intersection.begin()));

                map[allergen] = intersection;
            }
        }
    }

    /*
    From this, I get this list that I then
    manually sorted to figure which ingredient
    contained which allergen:
        eggs rdksxt
        eggs vfvvnm
        dairy rdksxt
        dairy vfvvnm
        soy hxntcz
        soy rdksxt
        soy srzqtccv
        fish rdksxt
        wheat bvgm
        wheat gbtmdb
        wheat vfvvnm
        peanuts bktzrz
        peanuts hxntcz
        peanuts rdksxt
        peanuts vfvvnm
        sesame bktzrz
        sesame gbtmdb
        sesame vfvvnm
        nuts srzqtccv
        nuts xknb
    */
    
    std::unordered_map<std::string, std::string> allergenic_ingredients = {
        {"xknb","nuts"},
        {"vfvvnm", "dairy"},
        {"bktzrz", "sesame"},
        {"gbtmdb", "wheat"},
        {"srzqtccv", "soy"},
        {"rdksxt", "fish"},
        {"bvgm", "eggs"},
        {"hxntcz", "peanuts"},
    };

    int ans = 0;
    for (auto ingredient_list : ingredients) {
        for (auto ingredient : ingredient_list) {
            if (allergenic_ingredients.find(ingredient) == allergenic_ingredients.end())
                ans++;
        }
    }

    std::cout << "Part 1: " << ans << std::endl;
    // Part 2 manually
    std::cout << "Part 2: " << "vfvvnm,bvgm,rdksxt,xknb,hxntcz,bktzrz,srzqtccv,gbtmdb" << std::endl;

    input.close();
    return 0;
}
