/**
 * @file systick.h
 * @brief SysTick timer driver interface
 */

#ifndef SYSTICK_H
#define SYSTICK_H

#include "types.h"

/**
 * Initialize SysTick timer for 1ms ticks
 */
void systick_init(void);

/**
 * Get current tick count (milliseconds since boot)
 * @return Tick count
 */
u32 systick_get_ticks(void);

/**
 * Delay for specified milliseconds
 * @param ms Milliseconds to delay
 */
void delay_ms(u32 ms);

/**
 * Check if timeout has elapsed
 * @param start_tick Starting tick value
 * @param timeout_ms Timeout in milliseconds
 * @return true if timeout elapsed
 */
bool timeout_elapsed(u32 start_tick, u32 timeout_ms);

#endif /* SYSTICK_H */
