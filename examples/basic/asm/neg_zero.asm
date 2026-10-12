bits 64
org 0x1000

; neg of 0 is the one case that clears CF. The cmp sets CF, SF and AF first, so this
; also checks that neg recomputes every status flag: only ZF + PF are left.
; expect rax = 0
; expect rflags = 0x46

    mov eax, 5
    cmp eax, 7
    mov eax, 0
    neg eax
    hlt
