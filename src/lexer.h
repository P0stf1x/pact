#ifndef PACT_LEXER
#define PACT_LEXER

#include <stdlib.h>

/* In place replace \r\n and \r with \n */
size_t lex(char input[], size_t input_length);

/* In place replace all trigraphs from input string with their actual symbol */
// size_t replace_trigraphs(char input[], size_t input_length);

#endif
