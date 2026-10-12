bits 64
org 0x1000

; 0b1100 & 0b1010 = 0b1000. One 1-bit is odd parity, so no flags are set.
; expect rax = 8
; expect rflags = 0x2
; expect rip = 0x100a
; expect executed = 3

    mov rax, 0b1100
    and rax, 0b1010
    hlt
