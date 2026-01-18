/**
 * @file config.h
 * @brief System configuration constants
 */

#ifndef CONFIG_H
#define CONFIG_H

/* Clock Configuration */
#define SYSCLK_FREQ_HZ      72000000UL
#define APB1_FREQ_HZ        36000000UL
#define APB2_FREQ_HZ        72000000UL

/* GPIO Configuration */
#define LED_PORT            GPIOC
#define LED_PIN             13
#define LED_ACTIVE_LOW      1

/* Timing Configuration */
#define BLINK_PERIOD_MS     500
#define DEBOUNCE_MS         50

/* UART Configuration */
#define UART_BAUD           115200
#define UART_TX_PIN         9
#define UART_RX_PIN         10

/* Watchdog Configuration */
#define WDT_TIMEOUT_MS      1000
#define WDT_ENABLED         0

/* Debug Configuration */
#define DEBUG_ENABLED       1
#define ASSERT_ENABLED      1

#if DEBUG_ENABLED
    #define DEBUG_PRINT(fmt, ...) uart_printf(fmt, ##__VA_ARGS__)
#else
    #define DEBUG_PRINT(fmt, ...) ((void)0)
#endif

#if ASSERT_ENABLED
    #define ASSERT(cond) do { if (!(cond)) { fault_handler(__FILE__, __LINE__); } } while(0)
#else
    #define ASSERT(cond) ((void)0)
#endif

#endif /* CONFIG_H */
