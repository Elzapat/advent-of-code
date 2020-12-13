#include <iostream>
#include <fstream>
#include <vector>
#include "../split.hpp"

long long chinese_remainder(std::vector<long long>& n, std::vector<long long>& a);
long long extended_euclidean_algorithm(long long a, long long b);

int main () {

    std::ifstream input("13.txt");
    std::string line;
    int min_depart = 0;
    std::string busses_s;

    input >> min_depart;
    input >> busses_s;

    std::vector<std::string> busses = split(busses_s, ","); 
    std::vector<int> busses_ids;
    for (auto bus : busses) {
        if (bus == "x") busses_ids.push_back(-1);
        else busses_ids.push_back(std::stoi(bus));
    }

    std::vector<long long> n, a;
    for (int i = 0; i < busses.size(); ++i) {
        if (busses[i] != "x") {
            int bus = std::stoi(busses[i]);
            n.push_back(bus);
            a.push_back((-i + bus) % bus);
        }
    }

    std::cout << chinese_remainder(n, a) << std::endl;

    input.close();
    return 0;
}

long long chinese_remainder(std::vector<long long>& n, std::vector<long long>& a) {

    long long N = 1, p = 0;;
    for (auto n_ : n)
        N *= n_;

    long long x = 0;

    for (int i = 0; i < n.size(); ++i) {
        p = N / n[i];
        x += a[i] * extended_euclidean_algorithm(p, n[i]) * p;
    }

    return x % N;
}

long long extended_euclidean_algorithm(long long a, long long b) {

    long long b0 = b, t = 0,
              x0 = 0, x1 = 1,
              q = 0;

    if (b == 0) return 1;
    
    while (a > 1) {
        q = a / b;
        t = b, b = a % b, a = t;
        t = x0, x0 = x1 - q * x0, x1 = t;
    }

    if (x1 < 0) x1 += b0;

    return x1;
}
