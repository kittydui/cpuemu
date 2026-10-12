bits 64
org 0x1000

; 0b1100 | 0b0011 = 0b1111. Four 1-bits is even parity (PF).
; expect rax = 15
; expect rflags = 0x6
; expect rip = 0x100a
; expect executed = 3

    mov rax, 0b1100
    or rax, 0b0011
    hlt
