/* SPDX-License-Identifier: Apache-2.0 */
/**
 * @file gpio.h
 * @brief GPIO driver header
 */

#ifndef __GPIO_H
#define __GPIO_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>

/* GPIO Mode */
typedef enum {
    GPIO_MODE_INPUT = 0,
    GPIO_MODE_OUTPUT_10MHZ,
    GPIO_MODE_OUTPUT_2MHZ,
    GPIO_MODE_OUTPUT_50MHZ
} GPIO_Mode_t;

/* GPIO Configuration */
typedef enum {
    GPIO_CNF_INPUT_ANALOG = 0,
    GPIO_CNF_INPUT_FLOATING,
    GPIO_CNF_INPUT_PULL,
    GPIO_CNF_OUTPUT_PP,
    GPIO_CNF_OUTPUT_OD,
    GPIO_CNF_OUTPUT_AF_PP,
    GPIO_CNF_OUTPUT_AF_OD
} GPIO_Config_t;

/* GPIO Pin State */
typedef enum {
    GPIO_PIN_RESET = 0,
    GPIO_PIN_SET
} GPIO_PinState_t;

/* Function Prototypes */
void GPIO_Init(uint32_t port, uint8_t pin, GPIO_Mode_t mode, GPIO_Config_t config);
void GPIO_WritePin(uint32_t port, uint8_t pin, GPIO_PinState_t state);
GPIO_PinState_t GPIO_ReadPin(uint32_t port, uint8_t pin);
void GPIO_TogglePin(uint32_t port, uint8_t pin);

#ifdef __cplusplus
}
#endif

#endif /* __GPIO_H */
