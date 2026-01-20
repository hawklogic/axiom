/* SPDX-License-Identifier: Apache-2.0 */
#ifndef CIRCULAR_A_H
#define CIRCULAR_A_H

#include <stdint.h>

// Forward declaration
struct CircularB;

typedef struct CircularA {
    uint32_t value_a;
    struct CircularB* b_ptr;
} CircularA;

uint32_t get_a_value(CircularA* a);

#endif /* CIRCULAR_A_H */
