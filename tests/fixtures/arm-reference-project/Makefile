# SPDX-License-Identifier: Apache-2.0
# Makefile for STM32F103C8 Reference Project

# Toolchain
PREFIX ?= arm-none-eabi-
CC = $(PREFIX)gcc
AS = $(PREFIX)as
LD = $(PREFIX)ld
OBJCOPY = $(PREFIX)objcopy
SIZE = $(PREFIX)size

# Project name
PROJECT = arm-reference

# Directories
CORE_DIR = Core
DRIVERS_DIR = Drivers
BUILD_DIR = build

# Source files
C_SOURCES = \
	$(CORE_DIR)/Src/main.c \
	$(DRIVERS_DIR)/gpio.c \
	$(DRIVERS_DIR)/uart.c \
	$(DRIVERS_DIR)/timer.c

# Include paths
INCLUDES = \
	-I$(CORE_DIR)/Inc \
	-I$(DRIVERS_DIR)

# MCU settings
MCU = -mcpu=cortex-m3 -mthumb

# Compiler flags
CFLAGS = $(MCU) $(INCLUDES) -Wall -Wextra -O2 -g3
CFLAGS += -DSTM32F103xB -DUSE_STDPERIPH_DRIVER

# Linker flags
LDFLAGS = $(MCU) -TSTM32F103C8_FLASH.ld -Wl,-Map=$(BUILD_DIR)/$(PROJECT).map -Wl,--gc-sections -nostartfiles

# Coverage flags
COVERAGE_FLAGS = --coverage -fprofile-arcs -ftest-coverage

# Object files
OBJECTS = $(addprefix $(BUILD_DIR)/,$(notdir $(C_SOURCES:.c=.o)))

# Default target
.PHONY: all
all: $(BUILD_DIR)/$(PROJECT).elf $(BUILD_DIR)/$(PROJECT).hex $(BUILD_DIR)/$(PROJECT).bin
	@echo "Build complete"
	@$(SIZE) $(BUILD_DIR)/$(PROJECT).elf

# Create build directory
$(BUILD_DIR):
	@mkdir -p $(BUILD_DIR)

# Compile C sources
$(BUILD_DIR)/%.o: $(CORE_DIR)/Src/%.c | $(BUILD_DIR)
	@echo "Compiling $<"
	@$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/%.o: $(DRIVERS_DIR)/%.c | $(BUILD_DIR)
	@echo "Compiling $<"
	@$(CC) $(CFLAGS) -c $< -o $@

# Link
$(BUILD_DIR)/$(PROJECT).elf: $(OBJECTS)
	@echo "Linking $@"
	@$(CC) $(LDFLAGS) $(OBJECTS) -o $@

# Generate hex
$(BUILD_DIR)/$(PROJECT).hex: $(BUILD_DIR)/$(PROJECT).elf
	@echo "Generating $@"
	@$(OBJCOPY) -O ihex $< $@

# Generate bin
$(BUILD_DIR)/$(PROJECT).bin: $(BUILD_DIR)/$(PROJECT).elf
	@echo "Generating $@"
	@$(OBJCOPY) -O binary $< $@

# Clean
.PHONY: clean
clean:
	@echo "Cleaning build artifacts"
	@rm -rf $(BUILD_DIR)
	@rm -f *.gcda *.gcno *.gcov

# Flash target (stub)
.PHONY: flash
flash: all
	@echo "Flash target not implemented"

# Debug target (stub)
.PHONY: debug
debug: CFLAGS += -DDEBUG -Og
debug: all

# Coverage build
.PHONY: coverage
coverage: CFLAGS += $(COVERAGE_FLAGS)
coverage: LDFLAGS += $(COVERAGE_FLAGS)
coverage: all

# Edge case targets
.PHONY: edge-empty
edge-empty:
	@echo "Building edge case: empty file"
	@$(CC) $(CFLAGS) -c edge_cases/empty_file.c -o $(BUILD_DIR)/empty_file.o || true

.PHONY: edge-syntax
edge-syntax:
	@echo "Building edge case: syntax error"
	@$(CC) $(CFLAGS) -c edge_cases/syntax_error.c -o $(BUILD_DIR)/syntax_error.o || true

.PHONY: edge-overflow
edge-overflow:
	@echo "Building edge case: linker overflow"
	@$(CC) $(CFLAGS) -c edge_cases/linker_overflow.c -o $(BUILD_DIR)/linker_overflow.o
	@$(CC) $(LDFLAGS) $(BUILD_DIR)/linker_overflow.o -o $(BUILD_DIR)/overflow.elf || true

.PHONY: edge-asm
edge-asm:
	@echo "Building edge case: inline assembly"
	@$(CC) $(CFLAGS) -c edge_cases/inline_assembly.c -o $(BUILD_DIR)/inline_assembly.o

.PHONY: edge-missing
edge-missing:
	@echo "Building edge case: missing include"
	@$(CC) $(CFLAGS) -c edge_cases/missing_include.c -o $(BUILD_DIR)/missing_include.o || true

# Help
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  all       - Build project (default)"
	@echo "  clean     - Remove build artifacts"
	@echo "  flash     - Flash to device"
	@echo "  debug     - Build with debug symbols"
	@echo "  coverage  - Build with coverage instrumentation"
	@echo "  edge-*    - Build edge case tests"
