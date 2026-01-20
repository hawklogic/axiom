/* SPDX-License-Identifier: Apache-2.0 */
/**
 * @file gpio.c
 * @brief GPIO driver implementation
 */

#include "gpio.h"

/* GPIO Register Structure */
typedef struct {
    volatile uint32_t CRL;      /* Port configuration register low */
    volatile uint32_t CRH;      /* Port configuration register high */
    volatile uint32_t IDR;      /* Port input data register */
    volatile uint32_t ODR;      /* Port output data register */
    volatile uint32_t BSRR;     /* Port bit set/reset register */
    volatile uint32_t BRR;      /* Port bit reset register */
    volatile uint32_t LCKR;     /* Port configuration lock register */
} GPIO_TypeDef;

/**
 * @brief Initialize GPIO pin
 * REQ-GPIO-001: GPIO driver shall support pin initialization
 */
void GPIO_Init(uint32_t port, uint8_t pin, GPIO_Mode_t mode, GPIO_Config_t config) {
    GPIO_TypeDef *gpio = (GPIO_TypeDef *)port;
    uint32_t pos = (pin < 8) ? (pin * 4) : ((pin - 8) * 4);
    volatile uint32_t *cr = (pin < 8) ? &gpio->CRL : &gpio->CRH;
    
    /* REQ-GPIO-001.1: Clear configuration bits */
    *cr &= ~(0xF << pos);
    
    /* REQ-GPIO-001.2: Set mode and configuration */
    *cr |= ((mode | (config << 2)) << pos);
}

/**
 * @brief Write to GPIO pin
 * REQ-GPIO-002: GPIO driver shall support pin write operations
 */
void GPIO_WritePin(uint32_t port, uint8_t pin, GPIO_PinState_t state) {
    GPIO_TypeDef *gpio = (GPIO_TypeDef *)port;
    
    if (state == GPIO_PIN_SET) {
        /* REQ-GPIO-002.1: Set pin high */
        gpio->BSRR = (1 << pin);
    } else {
        /* REQ-GPIO-002.2: Set pin low */
        gpio->BSRR = (1 << (pin + 16));
    }
}

/**
 * @brief Read GPIO pin state
 * REQ-GPIO-003: GPIO driver shall support pin read operations
 */
GPIO_PinState_t GPIO_ReadPin(uint32_t port, uint8_t pin) {
    GPIO_TypeDef *gpio = (GPIO_TypeDef *)port;
    
    /* REQ-GPIO-003.1: Read pin state from IDR */
    return (gpio->IDR & (1 << pin)) ? GPIO_PIN_SET : GPIO_PIN_RESET;
}

/**
 * @brief Toggle GPIO pin
 * REQ-GPIO-004: GPIO driver shall support pin toggle operations
 */
void GPIO_TogglePin(uint32_t port, uint8_t pin) {
    GPIO_TypeDef *gpio = (GPIO_TypeDef *)port;
    
    /* REQ-GPIO-004.1: Toggle pin by XOR with ODR */
    gpio->ODR ^= (1 << pin);
}
