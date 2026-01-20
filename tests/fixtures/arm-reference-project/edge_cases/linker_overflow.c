/* SPDX-License-Identifier: Apache-2.0 */
/* File with large array to trigger linker memory overflow */

#include <stdint.h>

/* 128KB array - exceeds STM32F103C8 64KB flash */
const uint8_t large_array[131072] = {0};

int main(void) {
    /* Access array to prevent optimization */
    volatile uint8_t val = large_array[0];
    (void)val;
    
    while (1) {
        /* Infinite loop */
    }
    
    return 0;
}
