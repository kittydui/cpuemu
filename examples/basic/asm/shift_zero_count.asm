bits 64
org 0x1000

; A shift count of 0 changes neither the value nor the flags, so the CF, AF and SF
; from the cmp are still there.
; expect rax = 5
; expect rflags = 0x93

    mov eax, 5
    cmp eax, 7
    mov cl, 0
    shl eax, cl
    hlt
