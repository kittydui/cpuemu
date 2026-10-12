bits 64
org 0x1000

; expect rax = 0xffff_ffff_ffff_fffb
; expect rflags = 0x83
; expect executed = 3

; 15 - 20 = -5
    mov rax, 15
    sub rax, 20
    hlt