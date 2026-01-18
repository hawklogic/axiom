@ ARM Assembly test file for syntax highlighting
@ Cortex-M4 startup and GPIO example

.syntax unified
.cpu cortex-m4
.thumb

.section .text
.global _start
.global main

@ Vector table
.word 0x20001000    @ Initial stack pointer
.word _start + 1    @ Reset handler

@ Constants
.equ RCC_BASE,      0x40023800
.equ GPIOA_BASE,    0x40020000
.equ RCC_AHB1ENR,   0x30
.equ GPIOA_MODER,   0x00
.equ GPIOA_ODR,     0x14

_start:
    @ Enable GPIOA clock
    ldr r0, =RCC_BASE
    ldr r1, [r0, #RCC_AHB1ENR]
    orr r1, r1, #0x01       @ Set GPIOA enable bit
    str r1, [r0, #RCC_AHB1ENR]
    
    @ Configure PA5 as output (LED)
    ldr r0, =GPIOA_BASE
    ldr r1, [r0, #GPIOA_MODER]
    bic r1, r1, #(0x3 << 10)   @ Clear PA5 mode bits
    orr r1, r1, #(0x1 << 10)   @ Set PA5 as output
    str r1, [r0, #GPIOA_MODER]
    
    bl main                 @ Call main function
    
    @ Infinite loop
loop:
    b loop

main:
    push {lr}
    
    @ Blink LED loop
blink_loop:
    @ Turn LED on
    ldr r0, =GPIOA_BASE
    ldr r1, [r0, #GPIOA_ODR]
    orr r1, r1, #(1 << 5)      @ Set PA5
    str r1, [r0, #GPIOA_ODR]
    
    @ Delay
    ldr r2, =0x100000
delay1:
    subs r2, r2, #1
    bne delay1
    
    @ Turn LED off
    ldr r0, =GPIOA_BASE
    ldr r1, [r0, #GPIOA_ODR]
    bic r1, r1, #(1 << 5)      @ Clear PA5
    str r1, [r0, #GPIOA_ODR]
    
    @ Delay
    ldr r2, =0x100000
delay2:
    subs r2, r2, #1
    bne delay2
    
    b blink_loop            @ Repeat forever
    
    pop {pc}                @ Return (never reached)

.end