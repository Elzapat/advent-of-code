#include <iostream>
#include <fstream>
#include <stack>
#include <queue>

std::string shunting_yard_algorithm(std::string expression);
long evaluate_rpn(std::string rpn);

int main() {

    std::ifstream input("18.txt");
    std::string line;
    long sum = 0;

    while (std::getline(input, line)) {
        std::string rpn = shunting_yard_algorithm(line);
        sum += evaluate_rpn(rpn);
    }

    std::cout << sum << std::endl;

    input.close();
    return 0;
}

std::string shunting_yard_algorithm(std::string expression) {

    std::queue<char> output_queue;
    std::stack<char> operator_stack;

    for (int i = 0; i < expression.size(); ++i) {
        char token = expression[i];

        if (token == ' ') continue;

        if (std::isdigit(token))
            output_queue.push(token);
        else if (token == '+' || token == '*') {
            while (!operator_stack.empty() && (token == '+' ? false : operator_stack.top() == '+') && operator_stack.top() != '(') {
                output_queue.push(operator_stack.top());
                operator_stack.pop();
            }
            operator_stack.push(token);
        } else if (token == '(')
            operator_stack.push(token);
        else if (token == ')') {
            while (operator_stack.top() != '(') {
                output_queue.push(operator_stack.top());
                operator_stack.pop();
            }
            if (operator_stack.top() == '(')
                operator_stack.pop();
        }
    }

    while (!operator_stack.empty()) {
        output_queue.push(operator_stack.top());
        operator_stack.pop();
    }

    std::string rpn = "";
    while (!output_queue.empty()) {
        rpn += output_queue.front();
        output_queue.pop();
    }

    return rpn;
}

long evaluate_rpn(std::string rpn) {

    std::stack<std::string> stack;

    for (int i = 0; i < rpn.size(); ++i) {
        std::string token(1, rpn[i]);

        if (token.find_first_not_of("0123456789") == std::string::npos)
            stack.push(token);
        else if (token == "+" || token == "*") {
            long first_operand = std::stol(stack.top());
            stack.pop();
            long second_operand = std::stol(stack.top());
            stack.pop();

            long result = 0;

            if (token == "+")
                result = first_operand + second_operand;
            else if (token == "*")
                result = first_operand * second_operand;

            stack.push(std::to_string(result));
        }
    }

    return std::stol(stack.top());
}
