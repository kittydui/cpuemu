bits 64
org 0x1000

; The edge case that `result < a` gets wrong: 0xFF + 0xFF + 1 = 0x1FF, so al ends up
; 0xFF again (equal to `a`), but it still carried out, so CF = 1.
; Also SF (top bit), PF (eight 1-bits) and AF (carry out of bit 3); OF stays clear.
; expect rax = 0xff
; expect rflags = 0x97

    mov eax, 5
    cmp eax, 7          ; CF = 1
    mov al, 0xff
    adc al, 0xff
    hlt
