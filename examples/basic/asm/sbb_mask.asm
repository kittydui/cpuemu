bits 64
org 0x1000

; The compiler idiom `sbb reg, reg`: x - x - CF is -1 when CF is set and 0 when it's clear,
; turning a comparison into an all-ones/all-zeros mask without a branch.
; expect rax = 0xffff_ffff
; expect rdx = 0

    mov ecx, 5
    cmp ecx, 7          ; CF = 1
    sbb eax, eax        ; -1 (the 32-bit write clears the upper half of rax)
    mov edx, 0x1234
    cmp ecx, 3          ; CF = 0
    sbb edx, edx        ; 0
    hlt
