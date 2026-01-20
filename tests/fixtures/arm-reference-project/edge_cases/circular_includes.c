/* SPDX-License-Identifier: Apache-2.0 */
/* File with circular include dependencies for testing */

#include "circular_a.h"
#include "circular_b.h"

uint32_t get_a_value(CircularA* a) {
    if (a == 0) return 0;
    return a->value_a;
}

uint32_t get_b_value(CircularB* b) {
    if (b == 0) return 0;
    return b->value_b;
}

uint32_t test_circular(void) {
    CircularA a = { .value_a = 10, .b_ptr = 0 };
    CircularB b = { .value_b = 20, .a_ptr = &a };
    a.b_ptr = &b;
    
    return get_a_value(&a) + get_b_value(&b);
}
