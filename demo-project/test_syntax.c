#define HELLO 9
// Test file for syntax highlighting
#include <stdio.h>
#include <stdint.h>

#define MAX_BUFFER 256
#define LED_PIN 13

typedef struct {
    uint32_t address;
    uint16_t data;
    bool enabled;
} register_t;

/* Multi-line comment
   for testing purposes */
static volatile register_t gpio_regs[8];

int main(void) {
    const char* message = "Hello, Axiom!";
    uint32_t counter = 0x1000;
    float voltage = 3.3f;
    
    // Initialize GPIO
    for (int i = 0; i < 8; i++) {
        gpio_regs[i].address = 0x40000000 + (i * 4);
        gpio_regs[i].data = 0;
        gpio_regs[i].enabled = false;
    }
    
    if (voltage > 3.0) {
        printf("%s - Counter: %u\n", message, counter);
    }
    
    return 0;
}