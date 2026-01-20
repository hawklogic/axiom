/* SPDX-License-Identifier: Apache-2.0 */
#ifndef CIRCULAR_B_H
#define CIRCULAR_B_H

#include <stdint.h>

// Forward declaration
struct CircularA;

typedef struct CircularB {
    uint32_t value_b;
    struct CircularA* a_ptr;
} CircularB;

uint32_t get_b_value(CircularB* b);

#endif /* CIRCULAR_B_H */
