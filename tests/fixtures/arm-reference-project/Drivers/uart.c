/* SPDX-License-Identifier: Apache-2.0 */
/**
 * @file uart.c
 * @brief UART driver implementation (stub)
 */

#include "uart.h"

/**
 * @brief Initialize UART
 * REQ-UART-001: UART driver shall support initialization
 */
void UART_Init(uint32_t uart, UART_Config_t *config) {
    /* Stub implementation */
    (void)uart;
    (void)config;
}

/**
 * @brief Send byte via UART
 * REQ-UART-002: UART driver shall support byte transmission
 */
void UART_SendByte(uint32_t uart, uint8_t data) {
    /* Stub implementation */
    (void)uart;
    (void)data;
}

/**
 * @brief Receive byte from UART
 * REQ-UART-003: UART driver shall support byte reception
 */
uint8_t UART_ReceiveByte(uint32_t uart) {
    /* Stub implementation */
    (void)uart;
    return 0;
}

/**
 * @brief Send string via UART
 * REQ-UART-004: UART driver shall support string transmission
 */
void UART_SendString(uint32_t uart, const char *str) {
    /* Stub implementation */
    (void)uart;
    while (*str) {
        UART_SendByte(uart, *str++);
    }
}
