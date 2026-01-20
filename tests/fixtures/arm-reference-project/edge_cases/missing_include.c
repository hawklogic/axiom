/* SPDX-License-Identifier: Apache-2.0 */
/* File that references nonexistent header for testing */

#include <stdint.h>
#include "nonexistent_header.h"  /* This file does not exist */

int main(void) {
    /* Try to use something from the missing header */
    nonexistent_function();
    
    return 0;
}
