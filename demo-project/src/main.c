/**
 * @file main.c
 * @brief Blink firmware main entry point
 * 
 * A minimal LED blink example demonstrating:
 * - GPIO configuration
 * - SysTick timing
 * - Main loop structure
 */

#include "types.h"
#include "config.h"
#include "gpio.h"
#include "systick.h"

/* Forward declarations */
static void system_init(void);
static void led_init(void);
static void led_set(bool on);
static void main_loop(void);

/**
 * Main entry point
 */
int main(void)
{
    system_init();
    led_init();
    
    DEBUG_PRINT("Blink firmware started\r\n");
    DEBUG_PRINT("LED period: %d ms\r\n", BLINK_PERIOD_MS);
    
    main_loop();
    
    /* Should never reach here */
    return 0;
}

/**
 * Initialize system peripherals
 */
static void system_init(void)
{
    /* Initialize GPIO clocks */
    gpio_init();
    
    /* Initialize SysTick for timing */
    systick_init();
}

/**
 * Initialize LED GPIO pin
 */
static void led_init(void)
{
    status_t status = gpio_configure(LED_PORT, LED_PIN, PIN_MODE_OUTPUT);
    ASSERT(status == STATUS_OK);
    
    /* Start with LED off */
    led_set(false);
}

/**
 * Set LED state
 * @param on true to turn LED on, false to turn off
 */
static void led_set(bool on)
{
#if LED_ACTIVE_LOW
    gpio_write(LED_PORT, LED_PIN, on ? PIN_LOW : PIN_HIGH);
#else
    gpio_write(LED_PORT, LED_PIN, on ? PIN_HIGH : PIN_LOW);
#endif
}

/**
 * Main application loop
 */
static void main_loop(void)
{
    bool led_state = false;
    u32 last_toggle = 0;
    
    while (1) {
        u32 now = systick_get_ticks();
        
        /* Toggle LED at configured interval */
        if ((now - last_toggle) >= BLINK_PERIOD_MS) {
            led_state = !led_state;
            led_set(led_state);
            last_toggle = now;
            
            DEBUG_PRINT("LED: %s\r\n", led_state ? "ON" : "OFF");
        }
        
        /* Other periodic tasks would go here */
    }
}

/**
 * Fault handler - called on assertion failure
 */
void fault_handler(const char *file, int line)
{
    /* Disable interrupts */
    __asm volatile ("cpsid i");
    
    DEBUG_PRINT("FAULT: %s:%d\r\n", file, line);
    
    /* Infinite loop with fast LED blink to indicate fault */
    while (1) {
        gpio_toggle(LED_PORT, LED_PIN);
        for (volatile int i = 0; i < 100000; i++);
    }
}
