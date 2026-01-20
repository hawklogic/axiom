/* SPDX-License-Identifier: Apache-2.0 */
/**
 * @file uart.h
 * @brief UART driver header (stub)
 */

#ifndef __UART_H
#define __UART_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

/* UART Configuration */
typedef struct {
    uint32_t baudrate;
    uint8_t  data_bits;
    uint8_t  stop_bits;
    uint8_t  parity;
} UART_Config_t;

/* Function Prototypes */
void UART_Init(uint32_t uart, UART_Config_t *config);
void UART_SendByte(uint32_t uart, uint8_t data);
uint8_t UART_ReceiveByte(uint32_t uart);
void UART_SendString(uint32_t uart, const char *str);

#ifdef __cplusplus
}
#endif

#endif /* __UART_H */
