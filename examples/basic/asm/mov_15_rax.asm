bits 64
org 0x1000

; expect rax = 30
; expect rflags = 0x16
; expect rip = 0x100b
; expect executed = 5

    mov rax, 15
    add rax, rax
    nop
    nop
    hlt
