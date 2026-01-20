/* SPDX-License-Identifier: Apache-2.0 */
/**
 * @file timer.c
 * @brief Timer driver implementation (stub)
 */

#include "timer.h"

/**
 * @brief Initialize timer
 * REQ-TIMER-001: Timer driver shall support initialization
 */
void Timer_Init(uint32_t timer, Timer_Config_t *config) {
    /* Stub implementation */
    (void)timer;
    (void)config;
}

/**
 * @brief Start timer
 * REQ-TIMER-002: Timer driver shall support start operation
 */
void Timer_Start(uint32_t timer) {
    /* Stub implementation */
    (void)timer;
}

/**
 * @brief Stop timer
 * REQ-TIMER-003: Timer driver shall support stop operation
 */
void Timer_Stop(uint32_t timer) {
    /* Stub implementation */
    (void)timer;
}

/**
 * @brief Get timer count
 * REQ-TIMER-004: Timer driver shall support count read operation
 */
uint32_t Timer_GetCount(uint32_t timer) {
    /* Stub implementation */
    (void)timer;
    return 0;
}
