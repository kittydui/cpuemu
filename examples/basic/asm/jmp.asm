bits 64
org 0x1000

; A forward jmp skips the `mov eax, 2`, so rax stays 1.
; expect rax = 1
; expect rip = 0x100d
; expect executed = 3

    mov eax, 1
    jmp skip
    mov eax, 2
skip:
    hlt
