# Blink Firmware Makefile

# Toolchain
PREFIX = arm-none-eabi-
CC = $(PREFIX)gcc
OBJCOPY = $(PREFIX)objcopy
SIZE = $(PREFIX)size

# Target
TARGET = blink
MCU = cortex-m3

# Directories
SRC_DIR = src
DRV_DIR = drivers
INC_DIR = include
BUILD_DIR = build

# Source files
SRCS = $(wildcard $(SRC_DIR)/*.c)
SRCS += $(wildcard $(DRV_DIR)/*.c)

# Object files
OBJS = $(SRCS:%.c=$(BUILD_DIR)/%.o)

# Include paths
INCLUDES = -I$(INC_DIR) -I$(DRV_DIR)

# Compiler flags
CFLAGS = -mcpu=$(MCU) -mthumb
CFLAGS += -Wall -Wextra -Werror
CFLAGS += -ffunction-sections -fdata-sections
CFLAGS += -Os
CFLAGS += $(INCLUDES)
CFLAGS += -std=c11

# Linker flags
LDFLAGS = -mcpu=$(MCU) -mthumb
LDFLAGS += -T linker.ld
LDFLAGS += -Wl,--gc-sections
LDFLAGS += -nostartfiles

.PHONY: all clean flash size

all: $(BUILD_DIR)/$(TARGET).bin

$(BUILD_DIR)/$(TARGET).elf: $(OBJS)
	@mkdir -p $(dir $@)
	$(CC) $(LDFLAGS) $^ -o $@

$(BUILD_DIR)/$(TARGET).bin: $(BUILD_DIR)/$(TARGET).elf
	$(OBJCOPY) -O binary $< $@
	@echo "Built: $@"

$(BUILD_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

size: $(BUILD_DIR)/$(TARGET).elf
	$(SIZE) $<

flash: $(BUILD_DIR)/$(TARGET).bin
	st-flash write $< 0x08000000

clean:
	rm -rf $(BUILD_DIR)
