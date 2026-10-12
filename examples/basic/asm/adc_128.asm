bits 64
org 0x1000

; 128-bit addition in two halves, rdx:rax + rcx:rbx = (2^64 - 1) + 1 = 2^64.
; The low add carries out, and adc carries it into the high half.
; expect rax = 0
; expect rdx = 1

    mov rax, -1         ; low half
    xor edx, edx        ; high half
    mov ebx, 1
    xor ecx, ecx
    add rax, rbx        ; low: 0, CF = 1
    adc rdx, rcx        ; high: 0 + 0 + 1
    hlt
