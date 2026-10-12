bits 64
org 0x1000

; 32-bit compare of INT_MIN with 1: 0x80000000 - 1 = 0x7FFFFFFF.
; A negative minus a positive gave a positive, so OF is set and SF is clear.
; SF != OF means `jl` would jump: INT_MIN < 1 as signed. No borrow as unsigned, so CF is clear.
; The low byte 0xFF has eight 1-bits (PF), and 0x00 - 0x01 borrows out of bit 3 (AF).
; expect rax = 0x8000_0000
; expect rflags = 0x816
; expect rip = 0x1009
; expect executed = 3

    mov eax, 0x8000_0000
    cmp eax, 1
    hlt
