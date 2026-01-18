/**
 * @file systick.c
 * @brief SysTick timer driver implementation
 */

#include "systick.h"
#include "config.h"

/* SysTick registers */
#define SYSTICK_BASE    0xE000E010UL
#define SYSTICK_CTRL    (*(vu32 *)(SYSTICK_BASE + 0x00))
#define SYSTICK_LOAD    (*(vu32 *)(SYSTICK_BASE + 0x04))
#define SYSTICK_VAL     (*(vu32 *)(SYSTICK_BASE + 0x08))

/* Control register bits */
#define SYSTICK_ENABLE      (1 << 0)
#define SYSTICK_TICKINT     (1 << 1)
#define SYSTICK_CLKSOURCE   (1 << 2)

/* Tick counter - incremented by SysTick ISR */
static volatile u32 tick_count = 0;

void systick_init(void)
{
    /* Calculate reload value for 1ms tick */
    u32 reload = (SYSCLK_FREQ_HZ / 1000) - 1;
    
    SYSTICK_LOAD = reload;
    SYSTICK_VAL = 0;  /* Clear current value */
    
    /* Enable SysTick with processor clock and interrupt */
    SYSTICK_CTRL = SYSTICK_ENABLE | SYSTICK_TICKINT | SYSTICK_CLKSOURCE;
}

u32 systick_get_ticks(void)
{
    return tick_count;
}

void delay_ms(u32 ms)
{
    u32 start = tick_count;
    while ((tick_count - start) < ms) {
        /* Wait */
    }
}

bool timeout_elapsed(u32 start_tick, u32 timeout_ms)
{
    return (tick_count - start_tick) >= timeout_ms;
}

/**
 * SysTick interrupt handler
 * Called every 1ms
 */
void SysTick_Handler(void)
{
    tick_count++;
}
