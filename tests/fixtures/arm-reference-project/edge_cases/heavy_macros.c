/* SPDX-License-Identifier: Apache-2.0 */
/* File with heavy preprocessor macro usage for testing */

#include <stdint.h>

// Nested macro definitions
#define LEVEL_1(x) ((x) * 2)
#define LEVEL_2(x) LEVEL_1(LEVEL_1(x))
#define LEVEL_3(x) LEVEL_2(LEVEL_2(x))
#define LEVEL_4(x) LEVEL_3(LEVEL_3(x))

// Stringification
#define STRINGIFY(x) #x
#define TOSTRING(x) STRINGIFY(x)

// Token pasting
#define CONCAT(a, b) a##b
#define MAKE_FUNC(name) CONCAT(func_, name)

// Variadic macros
#define LOG(fmt, ...) do { } while(0)
#define DEBUG(level, ...) LOG("DEBUG[%d]: ", level, __VA_ARGS__)

// Conditional compilation with nested conditions
#define FEATURE_A 1
#define FEATURE_B 1
#define FEATURE_C 0

#if FEATURE_A
  #if FEATURE_B
    #define COMBINED_FEATURE 1
  #else
    #define COMBINED_FEATURE 0
  #endif
#else
  #define COMBINED_FEATURE 0
#endif

// Complex macro with multiple lines
#define COMPLEX_MACRO(type, name, value) \
    type name = value; \
    type CONCAT(name, _backup) = value; \
    const char* CONCAT(name, _str) = TOSTRING(name)


// Recursive-like macro expansion
#define REPEAT_2(x) x x
#define REPEAT_4(x) REPEAT_2(REPEAT_2(x))
#define REPEAT_8(x) REPEAT_4(REPEAT_4(x))
#define REPEAT_16(x) REPEAT_8(REPEAT_8(x))

// Function-like macros with side effects
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define CLAMP(x, low, high) MIN(MAX(x, low), high)

// Bit manipulation macros
#define BIT(n) (1U << (n))
#define SET_BIT(reg, bit) ((reg) |= BIT(bit))
#define CLEAR_BIT(reg, bit) ((reg) &= ~BIT(bit))
#define TOGGLE_BIT(reg, bit) ((reg) ^= BIT(bit))
#define READ_BIT(reg, bit) (((reg) >> (bit)) & 1U)

// Register access macros
#define REG32(addr) (*(volatile uint32_t*)(addr))
#define REG16(addr) (*(volatile uint16_t*)(addr))
#define REG8(addr) (*(volatile uint8_t*)(addr))

// Test function using all these macros
uint32_t test_heavy_macros(void) {
    COMPLEX_MACRO(uint32_t, test_var, 42);
    
    uint32_t result = LEVEL_4(2);
    result = MAX(result, 100);
    result = CLAMP(result, 50, 200);
    
    LOG("Result: %d", result);
    DEBUG(1, "Test value: %d", test_var);
    
    #if COMBINED_FEATURE
    result += 10;
    #endif
    
    return result;
}

// Function using bit macros
void test_bit_operations(void) {
    uint32_t reg = 0;
    SET_BIT(reg, 5);
    CLEAR_BIT(reg, 3);
    TOGGLE_BIT(reg, 7);
    uint32_t bit_val = READ_BIT(reg, 5);
    (void)bit_val;
}
