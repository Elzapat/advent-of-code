/* Thanks to Noam for the Algorithm idea.
 * I just couldn't come up with one that
 * didn't take less than an hour myself.
 * It's close to a complete copy of his
 * code (with a slight optimization) 
 * but I made sure I thoroughly
 * understood it, and I'm sure he's ok
 * with it. Not like he has a choice. */

#include <iostream>
#include <vector>
#include <map>

int main() {

    int n = 30000000;
    std::vector<int> numbers = {20, 0, 1, 11, 6, 3};
    std::map<int, int> last_spoken;

    for (int i = 0; i < numbers.size(); ++i) {
        last_spoken[numbers[i]] = i + 1;
    }

    for (int i = numbers.size(); i < n; ++i) {
        int size = numbers.size();
        if (last_spoken.find(numbers[size - 1]) == last_spoken.end()) {
            last_spoken[numbers[size - 1]] = i;
            numbers.push_back(0);
        } else {
            numbers.push_back(i - last_spoken[numbers[size - 1]]);
            last_spoken[numbers[size - 2]] = i;
        }
    }

    std::cout << numbers[numbers.size() - 1] << std::endl;

    return 0;
}
