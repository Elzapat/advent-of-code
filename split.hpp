#include <iostream>
#include <vector>

std::vector<std::string> split (std::string string, std::string delimiter) {

    size_t pos = 0;
    std::string token;
    std::vector<std::string> arr;

    while ((pos = string.find(delimiter)) != std::string::npos) {
        token = string.substr(0, pos);
        arr.push_back(token); 
        string.erase(0, pos + delimiter.length());
    }
    arr.push_back(string.substr(0, pos));

    return arr;
}
