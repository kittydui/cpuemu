bits 64
org 0x1000

; inc doesn't affect CF: the cmp sets it (5 - 7 borrows) and it has to survive the inc.
; 5 + 1 = 6 has two 1-bits (PF), so rflags = reset bit + CF + PF.
; expect rax = 6
; expect rflags = 0x7

    mov eax, 5
    cmp eax, 7
    inc eax
    hlt
