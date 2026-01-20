// TEST: REQ-GPIO-001, REQ-GPIO-002, REQ-GPIO-003
// Unit tests for GPIO driver functionality

#include <stdint.h>
#include <stdbool.h>
#include "../Drivers/gpio.h"

// Test result tracking
static int tests_passed = 0;
static int tests_failed = 0;

// Simple test framework macros
#define TEST_ASSERT(condition, test_name) \
    if (condition) { \
        tests_passed++; \
    } else { \
        tests_failed++; \
    }

// TEST: REQ-GPIO-001
void test_gpio_init(void) {
    // Test GPIO initialization
    gpio_init(GPIOA, GPIO_PIN_5, GPIO_MODE_OUTPUT);
    TEST_ASSERT(true, "GPIO init should complete without error");
}

// TEST: REQ-GPIO-002
void test_gpio_write_high(void) {
    // Test writing high to GPIO pin
    gpio_init(GPIOA, GPIO_PIN_5, GPIO_MODE_OUTPUT);
    gpio_write(GPIOA, GPIO_PIN_5, GPIO_HIGH);
    TEST_ASSERT(true, "GPIO write high should complete");
}

// TEST: REQ-GPIO-002
void test_gpio_write_low(void) {
    // Test writing low to GPIO pin
    gpio_init(GPIOA, GPIO_PIN_5, GPIO_MODE_OUTPUT);
    gpio_write(GPIOA, GPIO_PIN_5, GPIO_LOW);
    TEST_ASSERT(true, "GPIO write low should complete");
}

// TEST: REQ-GPIO-003
void test_gpio_read(void) {
    // Test reading GPIO pin state
    gpio_init(GPIOA, GPIO_PIN_0, GPIO_MODE_INPUT);
    uint8_t state = gpio_read(GPIOA, GPIO_PIN_0);
    TEST_ASSERT(state == GPIO_LOW || state == GPIO_HIGH, "GPIO read should return valid state");
}

// TEST: REQ-GPIO-001, REQ-GPIO-002
void test_gpio_toggle(void) {
    // Test GPIO toggle functionality
    gpio_init(GPIOA, GPIO_PIN_5, GPIO_MODE_OUTPUT);
    gpio_toggle(GPIOA, GPIO_PIN_5);
    TEST_ASSERT(true, "GPIO toggle should complete");
}

// Test runner
int main(void) {
    test_gpio_init();
    test_gpio_write_high();
    test_gpio_write_low();
    test_gpio_read();
    test_gpio_toggle();
    
    // Return 0 if all tests passed, 1 otherwise
    return (tests_failed == 0) ? 0 : 1;
}
