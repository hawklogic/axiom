/* SPDX-License-Identifier: Apache-2.0 */
/**
 * @file main.h
 * @brief Main header file for STM32F103C8 reference project
 */

#ifndef __MAIN_H
#define __MAIN_H

#ifdef __cplusplus
extern "C" {
#endif

/* Includes */
#include <stdint.h>
#include <stdbool.h>

/* STM32F103C8 Base Addresses */
#define PERIPH_BASE           0x40000000UL
#define APB1PERIPH_BASE       PERIPH_BASE
#define APB2PERIPH_BASE       (PERIPH_BASE + 0x00010000UL)
#define AHBPERIPH_BASE        (PERIPH_BASE + 0x00020000UL)

/* GPIO Base Addresses */
#define GPIOA_BASE            (APB2PERIPH_BASE + 0x00000800UL)
#define GPIOB_BASE            (APB2PERIPH_BASE + 0x00000C00UL)
#define GPIOC_BASE            (APB2PERIPH_BASE + 0x00001000UL)

/* RCC Base Address */
#define RCC_BASE              (AHBPERIPH_BASE + 0x00001000UL)

/* System Clock */
#define HSI_VALUE             8000000UL  /* Internal High Speed oscillator */
#define HSE_VALUE             8000000UL  /* External High Speed oscillator */
#define SYSTEM_CLOCK          72000000UL /* System clock frequency */

/* GPIO Pin Definitions */
#define LED_PIN               13
#define LED_PORT              GPIOC_BASE

/* CMSIS-like intrinsics for bare-metal compilation */
static inline void __disable_irq(void) {
    __asm volatile ("cpsid i" : : : "memory");
}

static inline void __enable_irq(void) {
    __asm volatile ("cpsie i" : : : "memory");
}

/* Function Prototypes */
void SystemInit(void);
void SystemClock_Config(void);
void Error_Handler(void);

#ifdef __cplusplus
}
#endif

#endif /* __MAIN_H */
