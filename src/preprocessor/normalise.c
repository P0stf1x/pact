#include <stdlib.h>
#include <stdio.h>
#include <regex.h>
#include <memory.h>

size_t static regex_call(char input[], size_t input_length, char regex_string[], char replacing_with[]) {
    size_t replacing_with_len = strlen(replacing_with);

    regex_t regex;
    regmatch_t results[1];
    int return_value;

    return_value = regcomp(&regex, regex_string, REG_ENHANCED || REG_EXTENDED);
    if (return_value != 0) {
        char msgbuf[300];
        regerror(return_value, &regex, msgbuf, sizeof(msgbuf));
        fprintf(stderr, "Regex compile failed: %s\n", msgbuf);
        fprintf(stderr, "Could not compile regex\n");
        exit(1);
    }

    while (1==1) {
        return_value = regexec(&regex, input, 1, results, 0);
        if (return_value == 0) {
            memcpy(input + results[0].rm_so, replacing_with, replacing_with_len);
            memmove(input + results[0].rm_so + replacing_with_len, input + results[0].rm_eo, input_length-results[0].rm_eo+1);
        } else if (return_value == REG_NOMATCH) {
            return strlen(input);
        } else {
            return -1;
        }
    }
}

size_t fix_bad_newline(char input[], size_t input_length) {
    return regex_call(input, input_length, "\r\n|\r", "\n");
}

size_t replace_trigraphs(char input[], size_t input_length) {
    size_t new_length = input_length;
    new_length = regex_call(input, new_length, "\\?\\?<", "{");
    new_length = regex_call(input, new_length, "\\?\\?>", "}");
    new_length = regex_call(input, new_length, "\\?\\?\\(", "[");
    new_length = regex_call(input, new_length, "\\?\\?)", "]");
    new_length = regex_call(input, new_length, "\\?\\?/", "\\");
    new_length = regex_call(input, new_length, "\\?\\?'", "^");
    new_length = regex_call(input, new_length, "\\?\\?!", "|");
    new_length = regex_call(input, new_length, "\\?\\?-", "~");
    return new_length;
}
