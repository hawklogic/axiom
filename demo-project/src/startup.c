/**
 * @file startup.c
 * @brief Startup code and vector table for STM32F1
 */

#include "types.h"

/* Symbols defined by linker script */
extern u32 _estack;
extern u32 _sidata;
extern u32 _sdata;
extern u32 _edata;
extern u32 _sbss;
extern u32 _ebss;

/* Main function */
extern int main(void);

/* Exception handlers */
void Reset_Handler(void);
void NMI_Handler(void) __attribute__((weak, alias("Default_Handler")));
void HardFault_Handler(void) __attribute__((weak, alias("Default_Handler")));
void MemManage_Handler(void) __attribute__((weak, alias("Default_Handler")));
void BusFault_Handler(void) __attribute__((weak, alias("Default_Handler")));
void UsageFault_Handler(void) __attribute__((weak, alias("Default_Handler")));
void SVC_Handler(void) __attribute__((weak, alias("Default_Handler")));
void DebugMon_Handler(void) __attribute__((weak, alias("Default_Handler")));
void PendSV_Handler(void) __attribute__((weak, alias("Default_Handler")));
void SysTick_Handler(void) __attribute__((weak, alias("Default_Handler")));

/**
 * Vector table - placed at start of flash
 */
__attribute__((section(".isr_vector")))
const void *vector_table[] = {
    &_estack,           /* Initial stack pointer */
    Reset_Handler,      /* Reset handler */
    NMI_Handler,        /* NMI handler */
    HardFault_Handler,  /* Hard fault handler */
    MemManage_Handler,  /* MPU fault handler */
    BusFault_Handler,   /* Bus fault handler */
    UsageFault_Handler, /* Usage fault handler */
    0, 0, 0, 0,         /* Reserved */
    SVC_Handler,        /* SVCall handler */
    DebugMon_Handler,   /* Debug monitor handler */
    0,                  /* Reserved */
    PendSV_Handler,     /* PendSV handler */
    SysTick_Handler,    /* SysTick handler */
    /* IRQs would continue here... */
};

/**
 * Reset handler - entry point after reset
 */
void Reset_Handler(void)
{
    /* Copy initialized data from flash to RAM */
    u32 *src = &_sidata;
    u32 *dst = &_sdata;
    while (dst < &_edata) {
        *dst++ = *src++;
    }
    
    /* Zero-fill BSS section */
    dst = &_sbss;
    while (dst < &_ebss) {
        *dst++ = 0;
    }
    
    /* Call main */
    main();
    
    /* Infinite loop if main returns */
    while (1);
}

/**
 * Default handler for unimplemented interrupts
 */
void Default_Handler(void)
{
    while (1);
}
