/**
 * @file gpio.h
 * @brief GPIO driver interface
 */

#ifndef GPIO_H
#define GPIO_H

#include "types.h"

/* GPIO port base addresses */
#define GPIOA_BASE  0x40010800UL
#define GPIOB_BASE  0x40010C00UL
#define GPIOC_BASE  0x40011000UL

/* GPIO port type */
typedef struct {
    vu32 CRL;   /* Configuration register low */
    vu32 CRH;   /* Configuration register high */
    vu32 IDR;   /* Input data register */
    vu32 ODR;   /* Output data register */
    vu32 BSRR;  /* Bit set/reset register */
    vu32 BRR;   /* Bit reset register */
    vu32 LCKR;  /* Lock register */
} gpio_port_t;

/* Port definitions */
#define GPIOA   ((gpio_port_t *)GPIOA_BASE)
#define GPIOB   ((gpio_port_t *)GPIOB_BASE)
#define GPIOC   ((gpio_port_t *)GPIOC_BASE)

/**
 * Initialize GPIO peripheral clocks
 */
void gpio_init(void);

/**
 * Configure a GPIO pin
 * @param port GPIO port (GPIOA, GPIOB, GPIOC)
 * @param pin  Pin number (0-15)
 * @param mode Pin mode
 * @return STATUS_OK on success
 */
status_t gpio_configure(gpio_port_t *port, u8 pin, pin_mode_t mode);

/**
 * Set GPIO pin output state
 * @param port  GPIO port
 * @param pin   Pin number
 * @param state PIN_HIGH or PIN_LOW
 */
void gpio_write(gpio_port_t *port, u8 pin, pin_state_t state);

/**
 * Read GPIO pin input state
 * @param port GPIO port
 * @param pin  Pin number
 * @return Current pin state
 */
pin_state_t gpio_read(gpio_port_t *port, u8 pin);

/**
 * Toggle GPIO pin output state
 * @param port GPIO port
 * @param pin  Pin number
 */
void gpio_toggle(gpio_port_t *port, u8 pin);

#endif /* GPIO_H */
