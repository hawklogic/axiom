/* SPDX-License-Identifier: Apache-2.0 */
/* File with ARM inline assembly for testing */

#include <stdint.h>

/**
 * @brief Disable interrupts using CPSID
 */
void disable_interrupts(void) {
    __asm volatile ("CPSID i" : : : "memory");
}

/**
 * @brief Enable interrupts using CPSIE
 */
void enable_interrupts(void) {
    __asm volatile ("CPSIE i" : : : "memory");
}

/**
 * @brief Read PRIMASK register using MRS
 */
uint32_t get_primask(void) {
    uint32_t result;
    __asm volatile ("MRS %0, PRIMASK" : "=r" (result));
    return result;
}

/**
 * @brief Write PRIMASK register using MSR
 */
void set_primask(uint32_t value) {
    __asm volatile ("MSR PRIMASK, %0" : : "r" (value) : "memory");
}

/**
 * @brief Data Synchronization Barrier
 */
void data_sync_barrier(void) {
    __asm volatile ("DSB" : : : "memory");
}

/**
 * @brief Instruction Synchronization Barrier
 */
void instruction_sync_barrier(void) {
    __asm volatile ("ISB" : : : "memory");
}

/**
 * @brief Data Memory Barrier
 */
void data_memory_barrier(void) {
    __asm volatile ("DMB" : : : "memory");
}

/**
 * @brief No Operation
 */
void nop(void) {
    __asm volatile ("NOP");
}

/**
 * @brief Wait For Interrupt
 */
void wait_for_interrupt(void) {
    __asm volatile ("WFI");
}

/**
 * @brief Wait For Event
 */
void wait_for_event(void) {
    __asm volatile ("WFE");
}

/**
 * @brief Send Event
 */
void send_event(void) {
    __asm volatile ("SEV");
}
