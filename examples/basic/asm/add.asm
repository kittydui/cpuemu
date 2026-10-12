bits 64
org 0x1000

; expect rax = 30
; expect rflags = 0x16
; expect rip = 0x1009
; expect executed = 3

; load constant '15' into rax.
    mov rax, 15
; test add works, expected: 30
    add rax, rax
    hlt
