bits 64
org 0x1000

; expect rax = 10
; expect rflags = 0x6
; expect rip = 0x100a
; expect executed = 3

; load constant '15' into rax.
    mov rax, 15
; test sub works, expected: 10
    sub rax, 5
    hlt
