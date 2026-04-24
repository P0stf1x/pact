#ifndef PACT_NORMALISATION_STAGE
#define PACT_NORMALISATION_STAGE

#include <stdlib.h>

/* In place replace \r\n and \r with \n */
size_t fix_bad_newline(char input[], size_t input_length);

/* In place replace all trigraphs from input string with their actual symbol */
size_t replace_trigraphs(char input[], size_t input_length);

#endif
