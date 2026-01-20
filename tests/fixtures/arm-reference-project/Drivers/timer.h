/* SPDX-License-Identifier: Apache-2.0 */
/**
 * @file timer.h
 * @brief Timer driver header (stub)
 */

#ifndef __TIMER_H
#define __TIMER_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

/* Timer Configuration */
typedef struct {
    uint32_t prescaler;
    uint32_t period;
    uint8_t  auto_reload;
} Timer_Config_t;

/* Function Prototypes */
void Timer_Init(uint32_t timer, Timer_Config_t *config);
void Timer_Start(uint32_t timer);
void Timer_Stop(uint32_t timer);
uint32_t Timer_GetCount(uint32_t timer);

#ifdef __cplusplus
}
#endif

#endif /* __TIMER_H */
