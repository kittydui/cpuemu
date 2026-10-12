bits 64
org 0x1000

; The sbb edge case: 0 - 0xFF - 1 = -0x100, so al ends up 0 but it still borrowed
; (b + CF = 0x100 doesn't fit in a byte), so CF = 1.
; Also ZF + PF (result 0) and AF (borrow out of bit 3); SF and OF stay clear.
; expect rax = 0
; expect rflags = 0x57

    mov eax, 5
    cmp eax, 7          ; CF = 1
    mov al, 0
    sbb al, 0xff
    hlt
