#include <iostream>
#include <fstream>
#include <deque>
#include <vector>

int part_1(std::deque<int> p1_deck, std::deque<int> p2_deck);
int part_2(std::deque<int> p1_deck, std::deque<int> p2_deck, int game_id);

int main() {

    std::ifstream input("22.txt");
    std::string line;
    std::deque<int> p1_deck, p2_deck;
    bool insert_p1 = false;

    while (std::getline(input, line)) {
        if (line.find("Player") != std::string::npos) {
            insert_p1 = insert_p1 ? false : true;
            continue;
        } else if (line == "") continue;

        if (insert_p1) p1_deck.push_back(std::stoi(line));
        else p2_deck.push_back(std::stoi(line));
    }

    std::cout << "Part 1: " << part_1(p1_deck, p2_deck) << std::endl;
    std::cout << "Part 2: " << part_2(p1_deck, p2_deck, 1) << std::endl;

}

int part_2(std::deque<int> p1_deck, std::deque<int> p2_deck, int game_id) {

    std::vector<std::deque<int>> p1_previous_decks,
                                 p2_previous_decks;

    while (true) {
        for (auto p1_previous_deck : p1_previous_decks)
            if (p1_previous_deck == p1_deck) return 1;
        for (auto p2_previous_deck : p2_previous_decks)
            if (p2_previous_deck == p2_deck) return 1;

        int p1_card = p1_deck.front();
        int p2_card = p2_deck.front();
        p1_previous_decks.push_back(p1_deck);
        p2_previous_decks.push_back(p2_deck);
        p1_deck.pop_front();
        p2_deck.pop_front();

        std::deque<int>* round_winner = nullptr;

        if (p1_deck.size() >= p1_card && p2_deck.size() >= p2_card) {
            std::deque<int> p1_deck_copy = p1_deck,
                            p2_deck_copy = p2_deck;

            while (p1_deck_copy.size() > p1_card) p1_deck_copy.pop_back();
            while (p2_deck_copy.size() > p2_card) p2_deck_copy.pop_back();

            if (part_2(p1_deck_copy, p2_deck_copy, game_id + 1) == 1)
                round_winner = &p1_deck;
            else round_winner = &p2_deck;
        } else round_winner = p1_card > p2_card ? &p1_deck : &p2_deck;

        round_winner->push_back(round_winner == &p1_deck ? p1_card : p2_card);
        round_winner->push_back(round_winner == &p1_deck ? p2_card : p1_card);

        if (p1_deck.empty() || p2_deck.empty()) {
            std::deque<int>* winner = p1_deck.empty() ? &p2_deck : &p1_deck;

            if (game_id != 1) return winner == &p1_deck ? 1 : 2;
            else {
                int winner_score = 0;
                int winner_deck_size = winner->size();

                for (int i = 1; i <= winner_deck_size; ++i) {
                    winner_score += i * winner->back();
                    winner->pop_back();
                }

                return winner_score;
            }
        }
    }
}

int part_1(std::deque<int> p1_deck, std::deque<int> p2_deck) {

    while (true) {
        if (p1_deck.front() > p2_deck.front()) {
            p1_deck.push_back(p1_deck.front());
            p1_deck.push_back(p2_deck.front());
            p1_deck.pop_front();
            p2_deck.pop_front();
        } else if (p2_deck.front() > p1_deck.front()) {
            p2_deck.push_back(p2_deck.front());
            p2_deck.push_back(p1_deck.front());
            p2_deck.pop_front();
            p1_deck.pop_front();
        }

        if (p1_deck.empty() || p2_deck.empty()) break;
    }

    std::deque<int>* winner = p1_deck.empty() ? &p2_deck : &p1_deck;

    int winner_deck_size = winner->size();
    int winner_score = 0;
    for (int i = 1; i <= winner_deck_size; ++i) {
        winner_score += i * winner->back();
        winner->pop_back();
    }

    return winner_score;
}
