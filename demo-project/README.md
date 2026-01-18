# Blink Firmware

A minimal embedded firmware example for ARM Cortex-M microcontrollers.

## Structure

```
src/           - Application source files
include/       - Header files
drivers/       - Hardware abstraction drivers
tests/         - Unit tests
```

## Build

```bash
make          # Build firmware
make flash    # Flash to target
make clean    # Clean build artifacts
```

## Target

- MCU: STM32F103C8T6 (Blue Pill)
- Clock: 72 MHz
- Flash: 64KB
- RAM: 20KB
