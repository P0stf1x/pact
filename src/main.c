#include <stdlib.h>
#include <stdio.h>
#include <memory.h>
#include "preprocessor/normalise.h"
#include "lexer.h"

int main(int argc, char **argv) {
    if (argc != 2) {
        printf("Wrong argc length. expected 1 argument: file path\n");
        exit(-1);
    }

    FILE *file_pointer = fopen(argv[1], "r");

    fseek(file_pointer, 0L, SEEK_END);          // move head to EOF
    size_t file_size = ftell(file_pointer) + 1; // read position of head into file_size
    fseek(file_pointer, 0L, SEEK_SET);          // return head to the beginning

    char *input = malloc(file_size);
    fread(input, 1, file_size - 1, file_pointer);
    input[file_size-1] = '\0';

    size_t input_length = file_size;
    input_length = fix_bad_newline(input, input_length);
    input_length = replace_trigraphs(input, input_length);

    lex(input, input_length);
    free(input);
    exit(-1);

    printf("%s", input);
    free(input);
}
