bits 64
org 0x1000

; x ^ x is always 0, so this clears rax. The result 0 sets ZF and PF.
; expect rax = 0
; expect rflags = 0x46
; expect rip = 0x1009
; expect executed = 3

    mov rax, 30
    xor rax, rax
    hlt
