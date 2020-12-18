#include <iostream>
#include <fstream>

long evaluate_expression(std::string expression);
int find_matching_parenthesis(std::string expression);

int main() {

    std::ifstream input("18.txt");
    std::string line;
    long sum = 0;

    while (std::getline(input, line)) {
        sum += evaluate_expression(line);
    }

    std::cout << sum << std::endl;

    input.close();
    return 0;
}

long evaluate_expression(std::string expression) {

    long val = 0;
    char op = '+';

    for (int i = 0; i < expression.size(); ++i) {
        if (expression[i] == '+')
            op = '+';
        else if (expression[i] == '*')
            op = '*';
        else if (std::isdigit(expression[i])) {
            if (op == '+')
                val += std::stoi(expression.substr(i));
            else if (op == '*')
                val *= std::stoi(expression.substr(i));
        } else if (expression[i] == '(') {
            int start = i + 1;
            i += find_matching_parenthesis(expression.substr(i, expression.size() - i));

            long par_value = evaluate_expression(expression.substr(start, i - start));

            if (op == '+')
                val += par_value;
            else if (op == '*')
                val *= par_value;
        }
    }

    return val;
}

int find_matching_parenthesis(std::string expression) {

    int level = 0;

    for (int i = 0; i < expression.size(); ++i) {
        if (expression[i] == '(')
            level++;
        else if (expression[i] == ')') {
            level--;
        }
        if (level == 0) {
            return i;
        }
    }

    return expression.size();
}
