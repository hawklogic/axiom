/* SPDX-License-Identifier: Apache-2.0 */
/* File with intentional syntax errors for testing */

#include <stdint.h>

/* Missing semicolon */
int missing_semicolon = 42

/* Unclosed brace */
void unclosed_function(void) {
    int x = 10;

/* Undefined type */
undefined_type_t variable;

/* Missing closing parenthesis */
void bad_function(int x {
    return x + 1;
}

/* Invalid syntax */
this is not valid C code at all!
