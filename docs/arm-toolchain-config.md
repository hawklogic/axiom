# ARM Toolchain Configuration

This document provides examples and documentation for configuring ARM embedded development in Axiom.

## Quick Start

For a complete, well-commented example configuration file, see [`example-toolchain.toml`](./example-toolchain.toml) in this directory. Copy it to your project root as `.axiom/toolchain.toml` and customize for your needs.

## Configuration File Location

Project-specific ARM toolchain settings are stored in `.axiom/toolchain.toml` in your project root.

Global settings can be configured in `~/.axiom/settings.toml`.

## Basic Configuration

### Minimal Configuration

```toml
[arm]
mcu = "cortex-m4"
linker_script = "STM32F407.ld"
```

### Complete Configuration

```toml
[arm]
# Target MCU (cortex-m0, cortex-m3, cortex-m4, cortex-m7)
mcu = "cortex-m7"

# FPU type (fpv4-sp-d16 for M4, fpv5-d16 for M7, empty for M0/M3)
fpu = "fpv5-d16"

# Float ABI (soft, softfp, hard)
float_abi = "hard"

# Linker script path (relative to project root)
linker_script = "STM32H750VBTX_FLASH.ld"

# Preprocessor defines
[arm.defines]
values = [
    "STM32H750xx",
    "USE_HAL_DRIVER",
    "USE_PWR_LDO_SUPPLY",
    "HSE_VALUE=25000000"
]

# Include paths (relative to project root)
[arm.include_paths]
values = [
    "Core/Inc",
    "Drivers/STM32H7xx_HAL_Driver/Inc",
    "Drivers/STM32H7xx_HAL_Driver/Inc/Legacy",
    "Drivers/CMSIS/Device/ST/STM32H7xx/Include",
    "Drivers/CMSIS/Include"
]
```

## MCU Presets

### STM32F1 Series (Cortex-M3)

```toml
[arm]
mcu = "cortex-m3"
fpu = ""
float_abi = "soft"
linker_script = "STM32F103C8_FLASH.ld"

[arm.defines]
values = ["STM32F103xB", "USE_HAL_DRIVER"]
```

### STM32F4 Series (Cortex-M4 with FPU)

```toml
[arm]
mcu = "cortex-m4"
fpu = "fpv4-sp-d16"
float_abi = "hard"
linker_script = "STM32F407VGTx_FLASH.ld"

[arm.defines]
values = ["STM32F407xx", "USE_HAL_DRIVER"]
```

### STM32H7 Series (Cortex-M7 with Double-Precision FPU)

```toml
[arm]
mcu = "cortex-m7"
fpu = "fpv5-d16"
float_abi = "hard"
linker_script = "STM32H750VBTx_FLASH.ld"

[arm.defines]
values = [
    "STM32H750xx",
    "USE_HAL_DRIVER",
    "USE_PWR_LDO_SUPPLY"
]
```

### Nordic nRF52 Series (Cortex-M4 with FPU)

```toml
[arm]
mcu = "cortex-m4"
fpu = "fpv4-sp-d16"
float_abi = "hard"
linker_script = "nrf52840.ld"

[arm.defines]
values = [
    "NRF52840_XXAA",
    "BOARD_PCA10056",
    "CONFIG_GPIO_AS_PINRESET"
]
```

## Compliance Configuration

For safety-critical avionics software development:

```toml
[compliance]
# Enable DO-178C software airworthiness compliance
do178c_enabled = true

# Enable DO-330 tool qualification support
do330_enabled = true

# Enable ARP4754A system-level integration
arp4754a_enabled = false

# Design Assurance Level (A, B, C, D, or E)
# Level A: Catastrophic failure conditions
# Level B: Hazardous failure conditions
# Level C: Major failure conditions
# Level D: Minor failure conditions
# Level E: No safety effect
dal = "B"
```

## Toolchain Detection

Axiom automatically detects ARM toolchains from:

- **Homebrew** (macOS): `/opt/homebrew/bin/arm-none-eabi-gcc`
- **STM32CubeIDE** (all platforms): Searches plugin directories
- **System paths**: `/usr/bin`, `/usr/local/bin`

To use a custom toolchain path:

```toml
[toolchains.arm]
path = "/custom/path/to/arm-none-eabi-gcc"
```

## Linker Scripts

Linker scripts define memory layout for your MCU. Example for STM32F407:

```ld
/* STM32F407VGTx_FLASH.ld */

MEMORY
{
  FLASH (rx)  : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM (rwx)   : ORIGIN = 0x20000000, LENGTH = 128K
  CCMRAM (rw) : ORIGIN = 0x10000000, LENGTH = 64K
}

SECTIONS
{
  .text :
  {
    . = ALIGN(4);
    *(.text)
    *(.text*)
    . = ALIGN(4);
  } >FLASH

  .data :
  {
    . = ALIGN(4);
    *(.data)
    *(.data*)
    . = ALIGN(4);
  } >RAM AT> FLASH

  .bss :
  {
    . = ALIGN(4);
    *(.bss)
    *(.bss*)
    *(COMMON)
    . = ALIGN(4);
  } >RAM
}
```

## Build Optimization Levels

Optimization can be configured per-file or globally:

- `-O0`: No optimization (fastest compile, largest code, easiest debug)
- `-O1`: Basic optimization
- `-O2`: Moderate optimization (recommended for production)
- `-O3`: Aggressive optimization (may increase code size)
- `-Os`: Optimize for size
- `-Og`: Optimize for debugging experience

## Debugging Configuration

```toml
[arm.debug]
# Generate debug symbols
debug_symbols = true

# Debug symbol format (dwarf-2, dwarf-3, dwarf-4)
debug_format = "dwarf-2"

# Generate map file for memory analysis
generate_map = true
map_path = "build/firmware.map"
```

## Common Issues

### Toolchain Not Found

If Axiom doesn't detect your ARM toolchain:

1. Check that `arm-none-eabi-gcc` is in your PATH
2. Verify the toolchain is complete (all tools present)
3. Manually specify the path in `.axiom/toolchain.toml`

### Memory Overflow

If linking fails with "will not fit in region":

1. Check your linker script memory sizes match your MCU
2. Reduce code size with `-Os` optimization
3. Enable garbage collection: `-Wl,--gc-sections` (enabled by default)
4. Review the map file to identify large symbols

### Missing Headers

If compilation fails with "fatal error: file not found":

1. Add the missing include path to `[arm.include_paths]`
2. Verify the path is relative to your project root
3. Check that the header file actually exists

## Example Project Structure

```
my-stm32-project/
├── .axiom/
│   └── toolchain.toml          # This configuration file
├── Core/
│   ├── Inc/
│   │   ├── main.h
│   │   └── stm32h7xx_it.h
│   └── Src/
│       ├── main.c
│       └── stm32h7xx_it.c
├── Drivers/
│   ├── STM32H7xx_HAL_Driver/
│   └── CMSIS/
├── STM32H750VBTX_FLASH.ld      # Linker script
└── Makefile                     # Optional
```

## See Also

- [ARM GCC Documentation](https://gcc.gnu.org/onlinedocs/gcc/ARM-Options.html)
- [STM32CubeMX](https://www.st.com/en/development-tools/stm32cubemx.html) - For generating initialization code
- [DO-178C Standard](https://en.wikipedia.org/wiki/DO-178C) - Software airworthiness
