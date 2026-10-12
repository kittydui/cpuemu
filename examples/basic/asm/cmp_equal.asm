bits 64
org 0x1000

; cmp subtracts without storing: rax stays 15, and 15 - 15 = 0 sets ZF and PF.
; expect rax = 15
; expect rflags = 0x46
; expect rip = 0x100a
; expect executed = 3

    mov rax, 15
    cmp rax, 15
    hlt
