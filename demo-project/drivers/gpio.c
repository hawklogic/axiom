/**
 * @file gpio.c
 * @brief GPIO driver implementation for STM32F1
 */

#include "gpio.h"
#include "config.h"

/* RCC base address for clock control */
#define RCC_BASE        0x40021000UL
#define RCC_APB2ENR     (*(vu32 *)(RCC_BASE + 0x18))

/* Clock enable bits */
#define RCC_IOPAEN      (1 << 2)
#define RCC_IOPBEN      (1 << 3)
#define RCC_IOPCEN      (1 << 4)

/* Configuration register values */
#define GPIO_MODE_INPUT         0x0
#define GPIO_MODE_OUTPUT_10MHZ  0x1
#define GPIO_MODE_OUTPUT_2MHZ   0x2
#define GPIO_MODE_OUTPUT_50MHZ  0x3

#define GPIO_CNF_INPUT_ANALOG   0x0
#define GPIO_CNF_INPUT_FLOATING 0x1
#define GPIO_CNF_INPUT_PUPD     0x2

#define GPIO_CNF_OUTPUT_PP      0x0
#define GPIO_CNF_OUTPUT_OD      0x1
#define GPIO_CNF_AF_PP          0x2
#define GPIO_CNF_AF_OD          0x3

void gpio_init(void)
{
    /* Enable GPIO port clocks */
    RCC_APB2ENR |= RCC_IOPAEN | RCC_IOPBEN | RCC_IOPCEN;
    
    /* Small delay for clock to stabilize */
    for (volatile int i = 0; i < 10; i++);
}

status_t gpio_configure(gpio_port_t *port, u8 pin, pin_mode_t mode)
{
    if (port == NULL || pin > 15) {
        return STATUS_INVALID_PARAM;
    }
    
    u32 config = 0;
    
    switch (mode) {
        case PIN_MODE_INPUT:
            config = (GPIO_CNF_INPUT_FLOATING << 2) | GPIO_MODE_INPUT;
            break;
            
        case PIN_MODE_OUTPUT:
            config = (GPIO_CNF_OUTPUT_PP << 2) | GPIO_MODE_OUTPUT_2MHZ;
            break;
            
        case PIN_MODE_ALTERNATE:
            config = (GPIO_CNF_AF_PP << 2) | GPIO_MODE_OUTPUT_50MHZ;
            break;
            
        case PIN_MODE_ANALOG:
            config = (GPIO_CNF_INPUT_ANALOG << 2) | GPIO_MODE_INPUT;
            break;
            
        default:
            return STATUS_INVALID_PARAM;
    }
    
    /* Configure the pin in CRL (pins 0-7) or CRH (pins 8-15) */
    if (pin < 8) {
        u32 shift = pin * 4;
        port->CRL &= ~(0xF << shift);
        port->CRL |= (config << shift);
    } else {
        u32 shift = (pin - 8) * 4;
        port->CRH &= ~(0xF << shift);
        port->CRH |= (config << shift);
    }
    
    return STATUS_OK;
}

void gpio_write(gpio_port_t *port, u8 pin, pin_state_t state)
{
    if (state == PIN_HIGH) {
        port->BSRR = (1 << pin);  /* Set bit */
    } else {
        port->BRR = (1 << pin);   /* Reset bit */
    }
}

pin_state_t gpio_read(gpio_port_t *port, u8 pin)
{
    return (port->IDR & (1 << pin)) ? PIN_HIGH : PIN_LOW;
}

void gpio_toggle(gpio_port_t *port, u8 pin)
{
    port->ODR ^= (1 << pin);
}
