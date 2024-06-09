#include <stdlib.h>
#include <stdio.h>

#include "remixer.h"

int main() {
    printf("hello, world\n");
    char* values = "";

    get_words(WORD_FOX, &values);
    printf("some values: %s\n", values);
}
