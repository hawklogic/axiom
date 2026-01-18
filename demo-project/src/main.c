/**
 * @file types.h
 * @brief Common type definitions
 */

#ifndef TYPES_H
#define TYPES_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

/* Fixed-width types */
typedef uint8_t   u8;
typedef uint16_t  u16;
typedef uint32_t  u32;
typedef uint64_t  u64;

typedef int8_t    i8;
typedef int16_t   i16;
typedef int32_t   i32;
typedef int64_t   i64;

/* Volatile types for hardware registers */
typedef volatile uint8_t   vu8;
typedef volatile uint16_t  vu16;
typedef volatile uint32_t  vu32;

/* Status codes */
typedef enum {
    STATUS_OK = 0,
    STATUS_ERROR,
    STATUS_BUSY,
    STATUS_TIMEOUT,
    STATUS_INVALID_PARAM,
    STATUS_NOT_INITIALIZED,
} status_t;

/* GPIO pin state */
typedef enum {
    PIN_LOW = 0,
    PIN_HIGH = 1,
} pin_state_t;

/* GPIO pin mode */
typedef enum {
    PIN_MODE_INPUT,
    PIN_MODE_OUTPUT,
    PIN_MODE_ALTERNATE,
    PIN_MODE_ANALOG,
} pin_mode_t;

/* Callback function type */
typedef void (*callback_fn)(void *ctx);

#endif /* TYPES_H */
