bits 64
org 0x1000

; dec doesn't affect CF either: the cmp sets it, and `mov` leaves flags alone.
; 1 - 1 = 0 gives ZF + PF, plus the CF kept from the cmp.
; expect rax = 0
; expect rflags = 0x47

    mov eax, 5
    cmp eax, 7
    mov eax, 1
    dec eax
    hlt
