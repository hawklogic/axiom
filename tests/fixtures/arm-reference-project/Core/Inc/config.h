/* SPDX-License-Identifier: Apache-2.0 */
/**
 * @file config.h
 * @brief Project configuration settings
 */

#ifndef __CONFIG_H
#define __CONFIG_H

#ifdef __cplusplus
extern "C" {
#endif

/* Project Configuration */
#define PROJECT_NAME          "ARM Reference Project"
#define PROJECT_VERSION       "1.0.0"

/* MCU Configuration */
#define STM32F103C8
#define USE_STDPERIPH_DRIVER

/* Clock Configuration */
#define USE_HSE               1
#define PLL_MUL               9  /* 8MHz * 9 = 72MHz */

/* Feature Flags */
#define ENABLE_GPIO           1
#define ENABLE_UART           1
#define ENABLE_TIMER          1
#define ENABLE_DEBUG          1

/* Debug Configuration */
#ifdef ENABLE_DEBUG
  #define DEBUG_LEVEL         2
  #define ENABLE_ASSERT       1
#endif

/* Buffer Sizes */
#define UART_TX_BUFFER_SIZE   256
#define UART_RX_BUFFER_SIZE   256

/* Timing Configuration */
#define SYSTICK_FREQ_HZ       1000  /* 1ms tick */

/* Assert Macro */
#ifdef ENABLE_ASSERT
  #define ASSERT(expr) \
    do { \
      if (!(expr)) { \
        Error_Handler(); \
      } \
    } while(0)
#else
  #define ASSERT(expr) ((void)0)
#endif

#ifdef __cplusplus
}
#endif

#endif /* __CONFIG_H */
