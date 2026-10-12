bits 64
org 0x1000

; lea only does the address arithmetic: no memory access and no flags.
; expect rax = 0x128
; expect rdx = 0xff
; expect rflags = 0x2

    mov ebx, 0x100
    mov ecx, 3
    lea rax, [rbx + rcx*8 + 0x10]   ; 0x100 + 24 + 16
    lea edx, [rbx - 1]              ; 32-bit destination
    hlt
