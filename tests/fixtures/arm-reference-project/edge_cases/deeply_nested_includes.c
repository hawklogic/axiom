/* SPDX-License-Identifier: Apache-2.0 */
/* File with deeply nested includes for testing */

#include "nested_include_1.h"

uint32_t test_nested_includes(void) {
    return LEVEL_1 + LEVEL_2 + LEVEL_3 + LEVEL_4 + LEVEL_5;
}

uint32_t get_max_nesting(void) {
    return MAX_NESTING_LEVEL;
}
