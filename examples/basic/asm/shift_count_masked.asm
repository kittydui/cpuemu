bits 64
org 0x1000

; The count is masked to 6 bits for 64-bit operands and 5 bits otherwise, so both of
; these really shift by 1: 65 & 0x3F = 1 and 33 & 0x1F = 1.
; expect rax = 2
; expect rdx = 2

    mov rax, 1
    shl rax, 65
    mov edx, 1
    shl edx, 33
    hlt
