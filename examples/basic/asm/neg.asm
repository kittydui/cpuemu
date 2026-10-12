bits 64
org 0x1000

; -5 as 32 bits (the upper half of rax is cleared). CF is set because the source wasn't 0,
; SF because the result is negative, and AF because 0 - 5 borrows out of bit 3.
; The low byte 0xFB has seven 1-bits, so PF is clear.
; expect rax = 0xffff_fffb
; expect rflags = 0x93

    mov eax, 5
    neg eax
    hlt
