#include <iostream>

int determine_loop_size(const int KEY, const int SUBJECT_NUMBER, const int DIV_VALUE);
int determine_encryption_key(const int LOOP_SIZE, const int SUBJECT_NUMBER, const int DIV_VALUE);

int main() {

    const int CARD_KEY = 12578151;
    const int DOOR_KEY = 5051300;
    const int INITIAL_SUBJECT_NUMBER = 7;
    const int DIV_VALUE = 20201227;
    // Loop size was determined via dumb
    // bruteforce, with the function below
    // It took around 15 minutes.
    const int CARD_LOOP_SIZE = 538014;

    std::cout << "Part 1: " << determine_encryption_key(CARD_LOOP_SIZE, DOOR_KEY, DIV_VALUE) << std::endl;

    return 0;
}

int determine_encryption_key(const int LOOP_SIZE, const int SUBJECT_NUMBER, const int DIV_VALUE) {

    long value = 1;
    for (int i = 0; i < LOOP_SIZE; ++i) 
        value = (value * SUBJECT_NUMBER) % DIV_VALUE;

    return value;
}

int determine_loop_size(const int KEY, const int SUBJECT_NUMBER, const int DIV_VALUE) {

    int i = 0;
    while (++i) {
        long value = 1;
        for (int j = 0; j < i; ++j)
            value = (value * SUBJECT_NUMBER) % DIV_VALUE;

        if (value == KEY) break;
    }

    return i;
}
