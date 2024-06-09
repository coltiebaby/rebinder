#include <string.h>

// OPTIONS
const char* WORD_FOX = "fox";
const char* WORD_HD = "hd";

// fetch a group of words based on an option
void get_words(const char* option, char** buf) {
    char* fox = "The quick brown fox jumps over the lazy dog.";
    char* hd =
        "Humpty Dumpty sat on a wall,\n"
        "Humpty Dumpty had a great fall.\n"
        "Four-score Men and Four-score more,\n"
        "Could not make Humpty Dumpty where he was before.";

    if (option == WORD_FOX) {
        *buf = fox;
    }

    if (option == WORD_HD) {
        *buf = hd;
    }
}

int multi_two(int value) {
    return value * 2;
}
