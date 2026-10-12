bits 64
org 0x1000

; 128-bit subtraction in two halves, rdx:rax - rcx:rbx = 2^64 - 1.
; The low sub borrows, and sbb takes the borrow from the high half.
; expect rax = 0xffff_ffff_ffff_ffff
; expect rdx = 0

    xor eax, eax        ; low half
    mov edx, 1          ; high half: rdx:rax = 2^64
    mov ebx, 1
    xor ecx, ecx
    sub rax, rbx        ; low: -1, CF = 1
    sbb rdx, rcx        ; high: 1 - 0 - 1
    hlt
